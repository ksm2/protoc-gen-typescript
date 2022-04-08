use std::error::Error;
use std::io;

use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse, CodeGeneratorResponse_File};
use protobuf::Message;

use crate::pretty::Prettify;

mod ast;
mod pretty;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let request = CodeGeneratorRequest::parse_from_reader(&mut stdin)?;

    let module = ast::Module {
        declarations: request
            .proto_file
            .iter()
            .flat_map(|proto| proto.message_type.iter())
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
    };

    let mut file = CodeGeneratorResponse_File::new();
    file.set_name("index.ts".to_string());
    file.set_content(module.to_pretty(80));

    let mut response = CodeGeneratorResponse::new();
    response.file.push(file);
    response.write_to_writer(&mut io::stdout())?;
    Ok(())
}

fn id<I: Into<String>>(par: I) -> ast::Identifier {
    ast::Identifier(par.into())
}
