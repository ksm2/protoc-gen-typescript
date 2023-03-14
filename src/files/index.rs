use crate::printer::Module;
use protobuf::descriptor::{DescriptorProto, EnumDescriptorProto, ServiceDescriptorProto};
use protobuf::plugin::code_generator_response::File;
use std::collections::BTreeSet;

pub fn index(
    services: &[&ServiceDescriptorProto],
    messages: &[&DescriptorProto],
    enums: &[&EnumDescriptorProto],
) -> File {
    let mut module = Module::new("index.ts");

    let svc_names = services.iter().map(|svc| svc.name());
    let msg_names = messages.iter().map(|msg| msg.name());
    let enm_names = enums.iter().map(|e| e.name());

    let names = svc_names
        .chain(msg_names)
        .chain(enm_names)
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
