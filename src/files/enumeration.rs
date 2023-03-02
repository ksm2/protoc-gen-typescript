use crate::printer::Module;
use protobuf::descriptor::EnumDescriptorProto;
use protobuf::plugin::code_generator_response::File;

pub fn enumeration(enumeration: &EnumDescriptorProto) -> File {
    let name = enumeration.name();
    let mut module = Module::new(format!("{name}.ts"));

    let mut enum_ = module.enum_(name);
    for enum_value in &enumeration.value {
        enum_
            .item(enum_value.name())
            .value(&enum_value.number().to_string())
            .end();
    }
    enum_.end();

    module.into()
}
