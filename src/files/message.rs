use std::collections::BTreeSet;

use protobuf::descriptor::field_descriptor_proto::{Label, Type};
use protobuf::descriptor::field_options::JSType;
use protobuf::descriptor::{DescriptorProto, FieldDescriptorProto};
use protobuf::plugin::code_generator_response::File;

use crate::printer::{Block, Class, Module, SwitchBlock};

pub fn message(message: &DescriptorProto) -> File {
    let name = message.name();
    let mut module = Module::new(format!("{name}.ts"));

    module
        .import(&["BinaryReader", "BinaryWriter"])
        .from("google-protobuf");

    let imported_messages = message
        .field
        .iter()
        .filter(|field| field.type_() == Type::TYPE_MESSAGE || field.type_() == Type::TYPE_ENUM)
        .map(get_message_name)
        .collect::<BTreeSet<_>>();
    for imported_message in imported_messages {
        module
            .import(&[imported_message])
            .from(&format!("./{imported_message}"));
    }

    module.blank();

    let mut class = module.class(name);
    for field in &message.field {
        let field_type = field.type_();
        let property = class.property(field.json_name());
        if field.label() == Label::LABEL_REPEATED {
            property
                .typed(&(type_to_ts(field).to_string() + "[]"))
                .assign("[]")
                .end();
        } else if field_type == Type::TYPE_MESSAGE {
            property.optional().typed(type_to_ts(field)).end();
        } else {
            property
                .typed(type_to_ts(field))
                .assign(default_expr(field))
                .end();
        }
    }

    class.blank();

    if message.name() == "Timestamp" && message.field.len() == 2 {
        timestamp_methods(&mut class);
    } else if message.name() == "Duration" && message.field.len() == 2 {
        duration_methods(&mut class);
    } else if message.name() == "BoolValue" && message.field.len() == 1 {
        value_methods(&mut class, "BoolValue", "boolean");
    } else if message.name() == "BytesValue" && message.field.len() == 1 {
        value_methods(&mut class, "BytesValue", "Uint8Array");
    }

    let mut serialize = class.method("serialize", &[("writer", "BinaryWriter")], "void");
    for field in &message.field {
        serialize_field(field, &mut serialize);
    }
    serialize.end();

    class.blank();

    let mut deserialize = class.method("deserialize", &[("reader", "BinaryReader")], "void");
    let mut while_ = deserialize.while_("reader.nextField()");
    let mut switch = while_.switch("reader.getFieldNumber()");
    for field in &message.field {
        deserialize_field(field, &mut switch);
    }
    switch.end();
    while_.end();
    deserialize.end();

    class.end();

    module.into()
}

fn timestamp_methods(class: &mut Class) {
    let mut from_date = class.method("static fromDate", &[("date", "Date")], "Timestamp");
    from_date.call("const ts = new Timestamp();");
    from_date.call("ts.seconds = BigInt(Math.floor(date.getTime() / 1000));");
    from_date.call("ts.nanos = (date.getTime() % 1000) * 1_000_000;");
    from_date.call("return ts;");
    from_date.end();
    class.blank();

    let mut to_date = class.method("toDate", &[], "Date");
    to_date.call("const ts = new Timestamp();");
    to_date.call("const fromSeconds = Number(this.seconds * 1000n);");
    to_date.call("const fromNanos = Math.floor(this.nanos / 1_000_000);");
    to_date.call("return new Date(fromSeconds + fromNanos);");
    to_date.end();
    class.blank();
}

fn duration_methods(class: &mut Class) {
    let mut between = class.method(
        "static between",
        &[("date1", "Date"), ("date2", "Date")],
        "Duration",
    );
    between.call("const millis = Math.abs(date1.getTime() - date2.getTime());");
    between.call("return Duration.fromMillis(millis);");
    between.end();
    class.blank();

    let mut from_millis = class.method("static fromMillis", &[("millis", "number")], "Duration");
    from_millis.call("const dur = new Duration();");
    from_millis.call("dur.seconds = BigInt(Math.floor(millis / 1000));");
    from_millis.call("dur.nanos = (millis % 1000) * 1_000_000;");
    from_millis.call("return dur;");
    from_millis.end();
    class.blank();

    let mut to_millis = class.method("toMillis", &[], "number");
    to_millis.call("const fromSeconds = Number(this.seconds * 1000n);");
    to_millis.call("const fromNanos = Math.floor(this.nanos / 1_000_000);");
    to_millis.call("return fromSeconds + fromNanos;");
    to_millis.end();
    class.blank();
}

fn value_methods(class: &mut Class, type_: &str, param: &str) {
    let mut of = class.method("static of", &[("value", param)], type_);
    of.call(&format!("const v = new {type_}();"));
    of.call("v.value = value;");
    of.call("return v;");
    of.end();
    class.blank();
}

fn serialize_field(field: &FieldDescriptorProto, block: &mut impl Block) {
    let field_name = field.json_name();
    let mut then = match (field.type_(), field.label()) {
        (Type::TYPE_STRING, _) | (Type::TYPE_BYTES, _) | (_, Label::LABEL_REPEATED) => {
            block.if_(&format!("this.{field_name}.length > 0"))
        }
        _ => {
            let field_default = default_expr(field);
            block.if_(&format!("this.{field_name} !== {field_default}"))
        }
    };
    serialize_field_value(field, &mut then);
    then.end();
}

fn serialize_field_value(field: &FieldDescriptorProto, block: &mut impl Block) {
    let number = field.number();
    let field_name = field.json_name();

    let repeated = if field.label() == Label::LABEL_REPEATED {
        "Repeated"
    } else {
        ""
    };

    match field.type_() {
        Type::TYPE_MESSAGE => {
            let type_name = get_message_name(field);
            block.call(&format!(
                "writer.write{repeated}Message({number}, this.{field_name} as any, (message: {type_name}) => message.serialize(writer));"
            ));
        }

        Type::TYPE_INT64
        | Type::TYPE_UINT64
        | Type::TYPE_SFIXED64
        | Type::TYPE_SINT64
        | Type::TYPE_FIXED64 => {
            serialize_int64(field, block);
        }

        _ => {
            let method = method_suffix(&field.type_());
            block.call(&format!(
                "writer.write{repeated}{method}({number}, this.{field_name});"
            ));
        }
    }
}

fn serialize_int64(field: &FieldDescriptorProto, block: &mut impl Block) {
    let number = field.number();
    let field_name = field.json_name();
    let js_type = field.options.jstype();
    let method = method_suffix(&field.type_());

    let is_repeated = field.label() == Label::LABEL_REPEATED;
    if is_repeated {
        block.call(&match js_type {
            JSType::JS_NUMBER => {
                format!("writer.writeRepeated{method}({number}, this.{field_name});")
            }
            JSType::JS_STRING => {
                format!("writer.writeRepeated{method}String({number}, this.{field_name});")
            }
            JSType::JS_NORMAL => {
                format!("writer.writeRepeated{method}String({number}, this.{field_name}.map((each) => each.toString(10)));")
            }
        });
    } else {
        block.call(&match js_type {
            JSType::JS_NUMBER => format!("writer.write{method}({number}, this.{field_name});"),
            JSType::JS_STRING => {
                format!("writer.write{method}String({number}, this.{field_name});")
            }
            JSType::JS_NORMAL => {
                format!("writer.write{method}String({number}, this.{field_name}.toString(10));")
            }
        });
    }
}

fn deserialize_field(field: &FieldDescriptorProto, switch: &mut SwitchBlock) {
    let field_name = field.json_name();
    let field_type = field.type_();
    let repeated = field.label() == Label::LABEL_REPEATED;

    match field_type {
        Type::TYPE_MESSAGE => {
            let mut case = switch.case(&field.number().to_string());
            if repeated {
                let type_name = get_message_name(field);
                case.call(&format!("const message = new {type_name}();"));
                case.call(&format!(
                    "reader.readMessage(message, (msg: {type_name}) => msg.deserialize(reader));"
                ));
                case.call(&format!("this.{field_name}.push(message);"));
            } else {
                let type_name = get_message_name(field);
                case.call(&format!("this.{field_name} = new {type_name}();"));
                case.call(&format!("reader.readMessage(this.{field_name}, (message: {type_name}) => message.deserialize(reader));"));
            }
            case.end();
        }

        Type::TYPE_INT64
        | Type::TYPE_UINT64
        | Type::TYPE_SFIXED64
        | Type::TYPE_SINT64
        | Type::TYPE_FIXED64 => deserialize_int64(field, switch),

        _ => {
            let mut case = switch.case(&field.number().to_string());
            let method = method_suffix(&field_type);
            let packed = repeated && field.options.packed.unwrap_or(is_packable(&field_type));
            if packed {
                case.call(&format!("this.{field_name} = reader.readPacked{method}();"));
            } else if repeated {
                case.call(&format!("this.{field_name}.push(reader.read{method}());"));
            } else {
                case.call(&format!("this.{field_name} = reader.read{method}();"));
            }
            case.end();
        }
    }
}

fn deserialize_int64(field: &FieldDescriptorProto, switch: &mut SwitchBlock) {
    let mut case = switch.case(&field.number().to_string());

    let field_name = field.json_name();
    let repeated = field.label() == Label::LABEL_REPEATED;
    let method = method_suffix(&field.type_());

    if repeated {
        let packed = field.options.packed.unwrap_or(true);
        if packed {
            case.call(&match field.options.jstype() {
                JSType::JS_NUMBER => {
                    format!("this.{field_name} = reader.readPacked{method}();")
                }
                JSType::JS_STRING => {
                    format!("this.{field_name} = reader.readPacked{method}String();")
                }
                JSType::JS_NORMAL => {
                    format!("this.{field_name} = reader.readPacked{method}String().map(BigInt);")
                }
            });
        } else {
            case.call(&match field.options.jstype() {
                JSType::JS_NUMBER => {
                    format!("this.{field_name}.push(reader.read{method}());")
                }
                JSType::JS_STRING => {
                    format!("this.{field_name}.push(reader.read{method}String());")
                }
                JSType::JS_NORMAL => {
                    format!("this.{field_name}.push(BigInt(reader.read{method}String()));")
                }
            });
        }
    } else {
        case.call(&match field.options.jstype() {
            JSType::JS_NUMBER => {
                format!("this.{field_name} = reader.read{method}();")
            }
            JSType::JS_STRING => {
                format!("this.{field_name} = reader.read{method}String();")
            }
            JSType::JS_NORMAL => {
                format!("this.{field_name} = BigInt(reader.read{method}String());")
            }
        });
    }

    case.end();
}

fn is_packable(field_type: &Type) -> bool {
    !matches!(
        field_type,
        Type::TYPE_STRING | Type::TYPE_BYTES | Type::TYPE_MESSAGE
    )
}

fn method_suffix(field_type: &Type) -> &'static str {
    match field_type {
        Type::TYPE_BOOL => "Bool",
        Type::TYPE_BYTES => "Bytes",
        Type::TYPE_DOUBLE => "Double",
        Type::TYPE_ENUM => "Enum",
        Type::TYPE_FIXED32 => "Fixed32",
        Type::TYPE_FIXED64 => "Fixed64",
        Type::TYPE_FLOAT => "Float",
        Type::TYPE_GROUP => unreachable!(),
        Type::TYPE_INT32 => "Int32",
        Type::TYPE_INT64 => "Int64",
        Type::TYPE_MESSAGE => "Message",
        Type::TYPE_SFIXED32 => "Sfixed32",
        Type::TYPE_SFIXED64 => "Sfixed64",
        Type::TYPE_SINT32 => "Sint32",
        Type::TYPE_SINT64 => "Sint64",
        Type::TYPE_STRING => "String",
        Type::TYPE_UINT32 => "Uint32",
        Type::TYPE_UINT64 => "Uint64",
    }
}

fn type_to_ts(field: &FieldDescriptorProto) -> &str {
    let field_type = field.type_();
    match field_type {
        Type::TYPE_DOUBLE
        | Type::TYPE_FLOAT
        | Type::TYPE_INT32
        | Type::TYPE_FIXED32
        | Type::TYPE_UINT32
        | Type::TYPE_SFIXED32
        | Type::TYPE_SINT32 => "number",
        Type::TYPE_BOOL => "boolean",
        Type::TYPE_STRING => "string",
        Type::TYPE_BYTES => "Uint8Array",
        Type::TYPE_INT64
        | Type::TYPE_UINT64
        | Type::TYPE_FIXED64
        | Type::TYPE_SFIXED64
        | Type::TYPE_SINT64 => match field.options.jstype() {
            JSType::JS_NUMBER => "number",
            JSType::JS_STRING => "string",
            JSType::JS_NORMAL => "bigint",
        },
        Type::TYPE_MESSAGE | Type::TYPE_ENUM => get_message_name(field),
        _ => "any",
    }
}

fn get_message_name(field: &FieldDescriptorProto) -> &str {
    &field.type_name()[1..]
}

fn default_expr(field: &FieldDescriptorProto) -> &'static str {
    match field.type_() {
        Type::TYPE_DOUBLE
        | Type::TYPE_FLOAT
        | Type::TYPE_INT32
        | Type::TYPE_FIXED32
        | Type::TYPE_UINT32
        | Type::TYPE_SFIXED32
        | Type::TYPE_SINT32
        | Type::TYPE_ENUM => "0",
        Type::TYPE_BOOL => "false",
        Type::TYPE_STRING => "\"\"",
        Type::TYPE_BYTES => "new Uint8Array()",
        Type::TYPE_INT64
        | Type::TYPE_UINT64
        | Type::TYPE_FIXED64
        | Type::TYPE_SFIXED64
        | Type::TYPE_SINT64 => match field.options.jstype() {
            JSType::JS_NUMBER => "0",
            JSType::JS_STRING => "\"0\"",
            JSType::JS_NORMAL => "0n",
        },
        _ => "undefined",
    }
}
