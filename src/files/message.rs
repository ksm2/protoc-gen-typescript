use crate::printer::{Block, Module};
use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::descriptor::{DescriptorProto, FieldDescriptorProto};
use protobuf::plugin::code_generator_response::File;
use std::collections::HashSet;

pub fn message(message: &DescriptorProto) -> File {
    let name = message.name();
    let mut module = Module::new(format!("{name}.ts"));

    module
        .import(&["BinaryReader", "BinaryWriter"])
        .from("google-protobuf");

    let imported_messages = message
        .field
        .iter()
        .filter(|field| field.type_() == Type::TYPE_MESSAGE)
        .map(get_message_name)
        .collect::<HashSet<_>>();
    for imported_message in imported_messages {
        module
            .import(&[imported_message])
            .from(&format!("./{imported_message}"));
    }

    module.blank();

    let mut class = module.class(name);
    for field in &message.field {
        let field_type = field.type_();
        let property = class.property(field.name());
        if field_type == Type::TYPE_MESSAGE {
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
        let field_name = field.name();
        let field_default = default_expr(&field.type_());
        let mut then = serialize.if_(&format!("this.{field_name} !== {field_default}"));
        serialize_field(field, &mut then);
        then.end();
    }
    serialize.end();

    class.blank();

    let deserialize = class.method("deserialize", &[("reader", "BinaryReader")], "void");
    deserialize.end();

    class.end();

    module.into()
}

fn serialize_field(field: &FieldDescriptorProto, then: &mut impl Block) {
    let number = field.number();
    let field_name = field.name();

    let method = match field.type_() {
        Type::TYPE_BOOL => "writeBool",
        Type::TYPE_BYTES => "writeBytes",
        Type::TYPE_DOUBLE => "writeDouble",
        Type::TYPE_ENUM => "writeEnum",
        Type::TYPE_FIXED32 => "writeFixed32",
        Type::TYPE_FIXED64 => "writeFixed64",
        Type::TYPE_FLOAT => "writeFloat",
        Type::TYPE_GROUP => unreachable!(),
        Type::TYPE_INT32 => "writeInt32",
        Type::TYPE_INT64 => "writeInt64",
        Type::TYPE_MESSAGE => "writeMessage",
        Type::TYPE_SFIXED32 => "writeSfixed32",
        Type::TYPE_SFIXED64 => "writeSfixed64",
        Type::TYPE_SINT32 => "writeSint32",
        Type::TYPE_SINT64 => "writeSint64",
        Type::TYPE_STRING => "writeString",
        Type::TYPE_UINT32 => "writeUint32",
        Type::TYPE_UINT64 => "writeUint64",
    };

    match field.type_() {
        Type::TYPE_MESSAGE => {
            let type_name = get_message_name(field);
            then.call(&format!(
                "writer.writeMessage({number}, this.{field_name}, (message: {type_name}) => message.serialize(writer));"
            ));
        }

        Type::TYPE_INT64
        | Type::TYPE_UINT64
        | Type::TYPE_SFIXED64
        | Type::TYPE_SINT64
        | Type::TYPE_FIXED64 => {
            then.call(&format!(
                "writer.{method}String({number}, this.{field_name}.toString(10));"
            ));
        }

        _ => {
            then.call(&format!("writer.{method}({number}, this.{field_name});"));
        }
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
        Type::TYPE_MESSAGE => get_message_name(field),
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
