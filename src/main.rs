use std::error::Error;
use std::io;

use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse, CodeGeneratorResponse_File};
use protobuf::rustproto::exts::lite_runtime_all;
use protobuf::Message;

use crate::pretty::Prettify;

mod ast;
mod pretty;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let request = CodeGeneratorRequest::parse_from_reader(&mut stdin)?;

    let messages = request
        .proto_file
        .iter()
        .flat_map(|proto| proto.message_type.iter())
        .collect::<Vec<_>>();

    let module = ast::Module::new(
        messages
            .iter()
            .map(|descr| descr.get_name())
            .map(|name| ast::ImportDeclaration {
                source: ast::Literal::String(format!("./{}.js", name)),
                specifiers: vec![ast::ImportSpecifier::Named(ast::ImportNamedSpecifier {
                    local: id(name),
                    imported: id(name),
                })],
            })
            .map(ast::Declaration::Import)
            .collect(),
    );

    let mut response = CodeGeneratorResponse::new();
    response.file.push(file("index.ts", module));
    for message in messages {
        let module = ast::Module::new(vec![]);
        response
            .file
            .push(file(format!("{}.ts", message.get_name()), module))
    }
    response.write_to_writer(&mut io::stdout())?;
    Ok(())
}

fn file(filename: impl Into<String>, module: ast::Module) -> CodeGeneratorResponse_File {
    let mut file = CodeGeneratorResponse_File::new();
    file.set_name(filename.into());
    file.set_content(module.to_pretty(80));
    file
}

fn id(par: impl Into<String>) -> ast::Identifier {
    ast::Identifier(par.into())
}
