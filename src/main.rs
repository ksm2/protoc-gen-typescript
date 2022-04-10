use num_bigint::BigInt;
use protobuf::descriptor::{FieldDescriptorProto, FieldDescriptorProto_Type};
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
        let name = message.get_name();

        let module = ast::Module {
            span: Span::default(),
            shebang: None,
            body: vec![
                ast::ModuleItem::ModuleDecl(ast::ModuleDecl::Import(ast::ImportDecl {
                    span: Span::default(),
                    src: ast::Str::from("google-protobuf"),
                    type_only: false,
                    asserts: None,
                    specifiers: vec![
                        ast::ImportSpecifier::Named(ast::ImportNamedSpecifier {
                            span: Span::default(),
                            local: id("BinaryReader"),
                            imported: None,
                            is_type_only: false,
                        }),
                        ast::ImportSpecifier::Named(ast::ImportNamedSpecifier {
                            span: Span::default(),
                            local: id("BinaryWriter"),
                            imported: None,
                            is_type_only: false,
                        }),
                    ],
                })),
                ast::ModuleItem::ModuleDecl(ast::ModuleDecl::ExportDecl(ast::ExportDecl {
                    span: Span::default(),
                    decl: ast::Decl::Class(ast::ClassDecl {
                        ident: id(name),
                        declare: false,
                        class: ast::Class {
                            span: Span::default(),
                            decorators: vec![],
                            super_class: None,
                            is_abstract: false,
                            type_params: None,
                            super_type_params: None,
                            implements: vec![],
                            body: message
                                .field
                                .iter()
                                .map(field_prop)
                                .map(ast::ClassMember::PrivateProp)
                                .collect(),
                        },
                    }),
                })),
            ],
        };
        response
            .file
            .push(file(&c, format!("{}.ts", name), &module))
    }
    response.write_to_writer(&mut io::stdout())?;
    Ok(())
}

fn field_prop(field: &FieldDescriptorProto) -> ast::PrivateProp {
    ast::PrivateProp {
        span: Span::default(),
        key: ast::PrivateName {
            span: Span::default(),
            id: id(field.get_name()),
        },
        value: Some(Box::new(default_expr(field))),
        type_ann: Some(type_to_ts_ann(&field.get_field_type())),
        is_static: false,
        decorators: vec![],
        accessibility: None,
        is_optional: false,
        is_override: false,
        readonly: false,
        definite: false,
    }
}

fn default_expr(field: &FieldDescriptorProto) -> ast::Expr {
    match field.get_field_type() {
        FieldDescriptorProto_Type::TYPE_DOUBLE
        | FieldDescriptorProto_Type::TYPE_FLOAT
        | FieldDescriptorProto_Type::TYPE_INT32
        | FieldDescriptorProto_Type::TYPE_FIXED32
        | FieldDescriptorProto_Type::TYPE_UINT32
        | FieldDescriptorProto_Type::TYPE_SFIXED32
        | FieldDescriptorProto_Type::TYPE_SINT32
        | FieldDescriptorProto_Type::TYPE_ENUM => number_zero(),
        FieldDescriptorProto_Type::TYPE_BOOL => boolean_false(),
        FieldDescriptorProto_Type::TYPE_STRING => empty_string(),
        FieldDescriptorProto_Type::TYPE_BYTES => new_uint8_array(),
        FieldDescriptorProto_Type::TYPE_INT64
        | FieldDescriptorProto_Type::TYPE_UINT64
        | FieldDescriptorProto_Type::TYPE_FIXED64
        | FieldDescriptorProto_Type::TYPE_SFIXED64
        | FieldDescriptorProto_Type::TYPE_SINT64 => big_int_zero(),
        _ => undefined(),
    }
}

fn number_zero() -> ast::Expr {
    ast::Expr::Lit(ast::Lit::Num(ast::Number::from(0)))
}

fn boolean_false() -> ast::Expr {
    ast::Expr::Lit(ast::Lit::Bool(ast::Bool::from(false)))
}

fn empty_string() -> ast::Expr {
    ast::Expr::Lit(ast::Lit::Str(ast::Str::from("")))
}

fn new_uint8_array() -> ast::Expr {
    ast::Expr::New(ast::NewExpr {
        span: Span::default(),
        callee: Box::new(ast::Expr::Ident(id("Uint8Array"))),
        args: Some(vec![]),
        type_args: None,
    })
}

fn big_int_zero() -> ast::Expr {
    ast::Expr::Lit(ast::Lit::BigInt(ast::BigInt::from(BigInt::from(0))))
}

fn undefined() -> ast::Expr {
    ast::Expr::Ident(id("undefined"))
}

fn type_to_ts_ann(field_type: &FieldDescriptorProto_Type) -> ast::TsTypeAnn {
    ast::TsTypeAnn {
        span: Span::default(),
        type_ann: Box::new(type_to_ts(field_type)),
    }
}

fn type_to_ts(field_type: &FieldDescriptorProto_Type) -> ast::TsType {
    match field_type {
        FieldDescriptorProto_Type::TYPE_DOUBLE
        | FieldDescriptorProto_Type::TYPE_FLOAT
        | FieldDescriptorProto_Type::TYPE_INT32
        | FieldDescriptorProto_Type::TYPE_FIXED32
        | FieldDescriptorProto_Type::TYPE_UINT32
        | FieldDescriptorProto_Type::TYPE_SFIXED32
        | FieldDescriptorProto_Type::TYPE_SINT32 => number_type(),
        FieldDescriptorProto_Type::TYPE_BOOL => boolean_type(),
        FieldDescriptorProto_Type::TYPE_STRING => string_type(),
        FieldDescriptorProto_Type::TYPE_BYTES => uint8_array_type(),
        FieldDescriptorProto_Type::TYPE_INT64
        | FieldDescriptorProto_Type::TYPE_UINT64
        | FieldDescriptorProto_Type::TYPE_FIXED64
        | FieldDescriptorProto_Type::TYPE_SFIXED64
        | FieldDescriptorProto_Type::TYPE_SINT64 => big_int_type(),
        _ => any_type(),
    }
}

fn any_type() -> ast::TsType {
    keyword_type(ast::TsKeywordTypeKind::TsAnyKeyword)
}

fn boolean_type() -> ast::TsType {
    keyword_type(ast::TsKeywordTypeKind::TsBooleanKeyword)
}

fn string_type() -> ast::TsType {
    keyword_type(ast::TsKeywordTypeKind::TsStringKeyword)
}

fn number_type() -> ast::TsType {
    keyword_type(ast::TsKeywordTypeKind::TsNumberKeyword)
}

fn big_int_type() -> ast::TsType {
    keyword_type(ast::TsKeywordTypeKind::TsBigIntKeyword)
}

fn keyword_type(kind: ast::TsKeywordTypeKind) -> ast::TsType {
    ast::TsType::TsKeywordType(ast::TsKeywordType {
        span: Span::default(),
        kind,
    })
}

fn uint8_array_type() -> ast::TsType {
    ast::TsType::TsTypeRef(ast::TsTypeRef {
        span: Span::default(),
        type_name: ast::TsEntityName::Ident(id("Uint8Array")),
        type_params: None,
    })
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
