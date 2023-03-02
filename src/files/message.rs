use crate::printer::{Block, Module, SwitchBlock};
use protobuf::descriptor::field_descriptor_proto::{Label, Type};
use protobuf::descriptor::{DescriptorProto, FieldDescriptorProto};
use protobuf::plugin::code_generator_response::File;
use std::collections::BTreeSet;

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
                .assign(default_expr(&field_type))
                .end();
        }
    }

    class.blank();

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

fn serialize_field(field: &FieldDescriptorProto, block: &mut impl Block) {
    let field_name = field.json_name();
    let mut then = match (field.type_(), field.label()) {
        (Type::TYPE_STRING, _) | (Type::TYPE_BYTES, _) | (_, Label::LABEL_REPEATED) => {
            block.if_(&format!("this.{field_name}.length > 0"))
        }
        _ => {
            let field_default = default_expr(&field.type_());
            block.if_(&format!("this.{field_name} !== {field_default}"))
        }
    };
    serialize_field_value(field, &mut then);
    then.end();
}

fn serialize_field_value(field: &FieldDescriptorProto, block: &mut impl Block) {
    let number = field.number();
    let field_name = field.json_name();

    let method = method_suffix(&field.type_());
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
            if field.label() == Label::LABEL_REPEATED {
                block.call(&format!(
                    "writer.writeRepeated{method}String({number}, this.{field_name}.map((each) => each.toString(10)));"
                ));
            } else {
                block.call(&format!(
                    "writer.write{method}String({number}, this.{field_name}.toString(10));"
                ));
            }
        }

        _ => {
            block.call(&format!(
                "writer.write{repeated}{method}({number}, this.{field_name});"
            ));
        }
    }
}

fn deserialize_field(field: &FieldDescriptorProto, switch: &mut SwitchBlock) {
    let mut case = switch.case(&field.number().to_string());
    let field_name = field.json_name();
    let field_type = field.type_();
    let method = method_suffix(&field_type);
    let repeated = field.label() == Label::LABEL_REPEATED;

    match field_type {
        Type::TYPE_MESSAGE => {
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
        }

        Type::TYPE_INT64
        | Type::TYPE_UINT64
        | Type::TYPE_SFIXED64
        | Type::TYPE_SINT64
        | Type::TYPE_FIXED64 => {
            if repeated {
                let packed = field.options.packed.unwrap_or(true);
                if packed {
                    case.call(&format!(
                        "this.{field_name} = reader.readPacked{method}String().map(BigInt);"
                    ));
                } else {
                    case.call(&format!(
                        "this.{field_name}.push(BigInt(reader.read{method}String()));"
                    ));
                }
            } else {
                case.call(&format!(
                    "this.{field_name} = BigInt(reader.read{method}String());"
                ));
            }
        }

        _ => {
            let packed = repeated && field.options.packed.unwrap_or(is_packable(&field_type));
            if packed {
                case.call(&format!("this.{field_name} = reader.readPacked{method}();"));
            } else if repeated {
                case.call(&format!("this.{field_name}.push(reader.read{method}());"));
            } else {
                case.call(&format!("this.{field_name} = reader.read{method}();"));
            }
        }
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
        | Type::TYPE_SINT64 => "bigint",
        Type::TYPE_MESSAGE | Type::TYPE_ENUM => get_message_name(field),
        _ => "any",
    }
}

fn get_message_name(field: &FieldDescriptorProto) -> &str {
    &field.type_name()[1..]
}

fn default_expr(field_type: &Type) -> &'static str {
    match field_type {
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
        | Type::TYPE_SINT64 => "0n",
        _ => "undefined",
    }
}
