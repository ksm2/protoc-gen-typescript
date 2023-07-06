use crate::files;
use protobuf::descriptor::{DescriptorProto, EnumDescriptorProto, ServiceDescriptorProto};
use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse};
use protobuf::Message;
use std::error::Error;
use std::io::{Read, Write};

pub fn process<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), Box<dyn Error>> {
    let request = CodeGeneratorRequest::parse_from_reader(reader)?;
    let response = process_request(request);
    response.write_to_writer(writer)?;
    Ok(())
}

fn process_request(request: CodeGeneratorRequest) -> CodeGeneratorResponse {
    let services = extract_services(&request);
    let messages = extract_messages(&request);
    let enums = extract_enums(&request);

    let mut response = CodeGeneratorResponse::new();
    response
        .file
        .push(files::index(&services, &messages, &enums));
    response.file.push(files::types());
    for service in services {
        response.file.push(files::service_client(service));
    }
    for message in messages {
        response.file.push(files::message(message));
    }
    for enumeration in enums {
        response.file.push(files::enumeration(enumeration));
    }

    response
}

fn extract_services(request: &CodeGeneratorRequest) -> Vec<&ServiceDescriptorProto> {
    request
        .proto_file
        .iter()
        .flat_map(|proto| proto.service.iter())
        .collect()
}

fn extract_messages(request: &CodeGeneratorRequest) -> Vec<&DescriptorProto> {
    request
        .proto_file
        .iter()
        .flat_map(|proto| proto.message_type.iter())
        .collect()
}

fn extract_enums(request: &CodeGeneratorRequest) -> Vec<&EnumDescriptorProto> {
    request
        .proto_file
        .iter()
        .flat_map(|proto| proto.enum_type.iter())
        .collect()
}
