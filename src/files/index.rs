use crate::utils::*;
use protobuf::descriptor::DescriptorProto;
use protobuf::plugin::code_generator_response::File;
use swc::common::Span;
use swc::ecmascript::ast::*;
use swc::Compiler;

pub fn index(c: &Compiler, messages: &[&DescriptorProto]) -> File {
    let module = module(
        messages
            .iter()
            .map(|descr| descr.name())
            .map(to_named_export)
            .map(ModuleDecl::ExportNamed),
    );

    super::file(c, "index.ts", &module)
}

fn to_named_export(name: &str) -> NamedExport {
    NamedExport {
        span: Span::default(),
        specifiers: vec![ExportSpecifier::Named(ExportNamedSpecifier {
            span: Span::default(),
            exported: None,
            orig: ModuleExportName::Ident(id(name)),
            is_type_only: false,
        })],
        type_only: false,
        src: Some(Str::from(format!("./{name}.js"))),
        asserts: None,
    }
}
