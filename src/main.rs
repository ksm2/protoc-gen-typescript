use std::error::Error;
use std::io;
use std::sync::Arc;

use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse, CodeGeneratorResponse_File};
use protobuf::Message;
use swc::common::collections::AHashMap;
use swc::common::SourceMap;
use swc::common::Span;
use swc::config::SourceMapsConfig;
use swc::ecmascript::ast;
use swc::ecmascript::ast::EsVersion;

fn main() -> Result<(), Box<dyn Error>> {
    let cm = Arc::new(SourceMap::default());
    let c = swc::Compiler::new(cm);

    let mut stdin = io::stdin();
    let request = CodeGeneratorRequest::parse_from_reader(&mut stdin)?;

    let messages = request
        .proto_file
        .iter()
        .flat_map(|proto| proto.message_type.iter())
        .collect::<Vec<_>>();

    let module = ast::Module {
        span: Span::default(),
        shebang: None,
        body: messages
            .iter()
            .map(|descr| descr.get_name())
            .map(|name| ast::NamedExport {
                span: Span::default(),
                specifiers: vec![ast::ExportSpecifier::Named(ast::ExportNamedSpecifier {
                    span: Span::default(),
                    exported: None,
                    orig: ast::ModuleExportName::Ident(id(name)),
                    is_type_only: false,
                })],
                type_only: false,
                src: Some(ast::Str::from(format!("./{}.js", name))),
                asserts: None,
            })
            .map(ast::ModuleDecl::ExportNamed)
            .map(ast::ModuleItem::ModuleDecl)
            .collect(),
    };

    let mut response = CodeGeneratorResponse::new();
    response.file.push(file(&c, "index.ts", &module));
    for message in messages {
        let module = ast::Module {
            span: Span::default(),
            shebang: None,
            body: vec![],
        };
        response
            .file
            .push(file(&c, format!("{}.ts", message.get_name()), &module))
    }
    response.write_to_writer(&mut io::stdout())?;
    Ok(())
}

fn file(
    c: &swc::Compiler,
    filename: impl Into<String>,
    module: &ast::Module,
) -> CodeGeneratorResponse_File {
    let ast_printed = c
        .print(
            module,
            None,
            None,
            false,
            EsVersion::Es2022,
            SourceMapsConfig::Bool(false),
            &AHashMap::default(),
            None,
            false,
            None,
        )
        .expect("Failed to print");

    let mut file = CodeGeneratorResponse_File::new();
    file.set_name(filename.into());
    file.set_content(ast_printed.code);
    file
}

fn id(str: &'_ str) -> ast::Ident {
    ast::Ident::new(str.into(), Span::default())
}
