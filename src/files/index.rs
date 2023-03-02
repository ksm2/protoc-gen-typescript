use crate::printer::Module;
use protobuf::descriptor::{DescriptorProto, EnumDescriptorProto};
use protobuf::plugin::code_generator_response::File;
use std::collections::HashSet;

pub fn index(messages: &[&DescriptorProto], enums: &[&EnumDescriptorProto]) -> File {
    let mut module = Module::new("index.ts");

    let names = messages
        .iter()
        .map(|msg| msg.name())
        .chain(enums.iter().map(|e| e.name()))
        .collect::<HashSet<_>>();

    for name in names {
        module.export(&[name]).from(&format!("./{name}"));
    }

    module.into()
}
