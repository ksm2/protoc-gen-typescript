use crate::printer::Module;
use protobuf::descriptor::{DescriptorProto, EnumDescriptorProto};
use protobuf::plugin::code_generator_response::File;
use std::collections::BTreeSet;

pub fn index(messages: &[&DescriptorProto], enums: &[&EnumDescriptorProto]) -> File {
    let mut module = Module::new("index.ts");

    let names = messages
        .iter()
        .map(|msg| msg.name())
        .chain(enums.iter().map(|e| e.name()))
        .collect::<BTreeSet<_>>();

    module
        .export(&[
            "type GrpcService",
            "type Serializable",
            "type Deserializable",
            "type Class",
        ])
        .from("./types");

    for name in names {
        module.export(&[name]).from(&format!("./{name}"));
    }

    module.into()
}
