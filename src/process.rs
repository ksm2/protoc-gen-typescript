use crate::files;
use protobuf::descriptor::DescriptorProto;
use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse};
use protobuf::Message;
use std::error::Error;
use std::io::{Read, Write};
use std::sync::Arc;
use swc::common::SourceMap;

pub fn process<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), Box<dyn Error>> {
    let request = CodeGeneratorRequest::parse_from_reader(reader)?;
    let response = process_request(request);
    response.write_to_writer(writer)?;
    Ok(())
}

fn process_request(request: CodeGeneratorRequest) -> CodeGeneratorResponse {
    let cm = Arc::new(SourceMap::default());
    let compiler = swc::Compiler::new(cm);

    let messages = extract_messages(&request);

    let mut response = CodeGeneratorResponse::new();
    response.file.push(files::index(&compiler, &messages));
    for message in messages {
        response.file.push(files::message(&compiler, message))
    }

    response
}

fn extract_messages(request: &CodeGeneratorRequest) -> Vec<&DescriptorProto> {
    request
        .proto_file
        .iter()
        .flat_map(|proto| proto.message_type.iter())
        .collect()
}