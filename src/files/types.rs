use protobuf::plugin::code_generator_response::File;

pub fn types() -> File {
    let types = include_str!("../res/types.ts");
    let mut file = File::new();
    file.set_name("types.ts".into());
    file.set_content(types.into());
    file
}
