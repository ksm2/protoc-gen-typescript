use crate::printer::Module;
use protobuf::descriptor::DescriptorProto;
use protobuf::plugin::code_generator_response::File;

pub fn index(messages: &[&DescriptorProto]) -> File {
    let mut module = Module::new("index.ts");

    for message in messages {
        let name = message.name();
        module.export(&[name]).from(&format!("./{name}"));
    }

    module.into()
}
