use num_bigint::BigInt;
use protobuf::descriptor::{DescriptorProto, FieldDescriptorProto, FieldDescriptorProto_Type};
use protobuf::plugin::{CodeGeneratorRequest, CodeGeneratorResponse, CodeGeneratorResponse_File};
use protobuf::Message;
use std::error::Error;
use std::io;
use std::sync::Arc;
use swc::common::collections::AHashMap;
use swc::common::SourceMap;
use swc::common::Span;
use swc::config::SourceMapsConfig;
use swc::ecmascript::ast;
use swc::ecmascript::ast::*;

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

    let module = Module {
        span: Span::default(),
        shebang: None,
        body: messages
            .iter()
            .map(|descr| descr.get_name())
            .map(|name| NamedExport {
                span: Span::default(),
                specifiers: vec![ExportSpecifier::Named(ExportNamedSpecifier {
                    span: Span::default(),
                    exported: None,
                    orig: ModuleExportName::Ident(id(name)),
                    is_type_only: false,
                })],
                type_only: false,
                src: Some(ast::Str::from(format!("./{}.js", name))),
                asserts: None,
            })
            .map(ModuleDecl::ExportNamed)
            .map(ModuleItem::ModuleDecl)
            .collect(),
    };

    let mut response = CodeGeneratorResponse::new();
    response.file.push(file(&c, "index.ts", &module));
    for message in messages {
        let name = message.get_name();

        let module = Module {
            span: Span::default(),
            shebang: None,
            body: vec![
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    span: Span::default(),
                    src: ast::Str::from("google-protobuf"),
                    type_only: false,
                    asserts: None,
                    specifiers: vec![
                        ImportSpecifier::Named(ImportNamedSpecifier {
                            span: Span::default(),
                            local: id("BinaryReader"),
                            imported: None,
                            is_type_only: false,
                        }),
                        ImportSpecifier::Named(ImportNamedSpecifier {
                            span: Span::default(),
                            local: id("BinaryWriter"),
                            imported: None,
                            is_type_only: false,
                        }),
                    ],
                })),
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                    span: Span::default(),
                    decl: Decl::Class(ClassDecl {
                        ident: id(name),
                        declare: false,
                        class: Class {
                            span: Span::default(),
                            decorators: vec![],
                            super_class: None,
                            is_abstract: false,
                            type_params: None,
                            super_type_params: None,
                            implements: vec![],
                            body: [
                                message
                                    .field
                                    .iter()
                                    .map(field_prop)
                                    .map(ClassMember::PrivateProp)
                                    .collect(),
                                vec![serialize_method(message)],
                            ]
                            .concat(),
                        },
                    }),
                })),
            ],
        };
        response.file.push(file(&c, format!("{name}.ts"), &module))
    }
    response.write_to_writer(&mut io::stdout())?;
    Ok(())
}

fn serialize_method(message: &DescriptorProto) -> ClassMember {
    ClassMember::Method(ClassMethod {
        span: Span::default(),
        key: PropName::Ident(id("serialize")),
        kind: MethodKind::Method,
        is_static: false,
        accessibility: None,
        is_abstract: false,
        is_optional: false,
        is_override: false,
        function: Function {
            params: vec![Param {
                span: Default::default(),
                decorators: vec![],
                pat: Pat::Ident(BindingIdent {
                    id: id("writer"),
                    type_ann: Some(to_type_ann(ident_type(id("BinaryWriter")))),
                }),
            }],
            decorators: vec![],
            span: Default::default(),
            body: Some(BlockStmt {
                span: Default::default(),
                stmts: message.field.iter().map(serialize_field).collect(),
            }),
            is_generator: false,
            is_async: false,
            type_params: None,
            return_type: Some(to_type_ann(void_type())),
        },
    })
}

fn field_prop(field: &FieldDescriptorProto) -> PrivateProp {
    PrivateProp {
        span: Span::default(),
        key: private_name(field.get_name()),
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

fn serialize_field(field: &FieldDescriptorProto) -> Stmt {
    let cond = Expr::Bin(BinExpr {
        span: Span::default(),
        op: BinaryOp::NotEqEq,
        left: Box::new(Expr::Member(MemberExpr {
            span: Span::default(),
            obj: Box::new(Expr::This(ThisExpr {
                span: Span::default(),
            })),
            prop: MemberProp::PrivateName(private_name(field.get_name())),
        })),
        right: Box::new(default_expr(field)),
    });

    let then = Stmt::Empty(EmptyStmt {
        span: Span::default(),
    });

    let if_stmt = IfStmt {
        span: Span::default(),
        alt: None,
        test: Box::new(cond),
        cons: Box::new(then),
    };

    Stmt::If(if_stmt)
}

fn default_expr(field: &FieldDescriptorProto) -> Expr {
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

fn number_zero() -> Expr {
    Expr::Lit(Lit::Num(ast::Number::from(0)))
}

fn boolean_false() -> Expr {
    Expr::Lit(Lit::Bool(ast::Bool::from(false)))
}

fn empty_string() -> Expr {
    Expr::Lit(Lit::Str(ast::Str::from("")))
}

fn new_uint8_array() -> Expr {
    Expr::New(NewExpr {
        span: Span::default(),
        callee: Box::new(Expr::Ident(id("Uint8Array"))),
        args: Some(vec![]),
        type_args: None,
    })
}

fn big_int_zero() -> Expr {
    Expr::Lit(Lit::BigInt(ast::BigInt::from(BigInt::from(0))))
}

fn undefined() -> Expr {
    Expr::Ident(id("undefined"))
}

fn type_to_ts_ann(field_type: &FieldDescriptorProto_Type) -> TsTypeAnn {
    to_type_ann(type_to_ts(field_type))
}

fn to_type_ann(ts_type: TsType) -> TsTypeAnn {
    TsTypeAnn {
        span: Span::default(),
        type_ann: Box::new(ts_type),
    }
}

fn type_to_ts(field_type: &FieldDescriptorProto_Type) -> TsType {
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

fn any_type() -> TsType {
    keyword_type(TsKeywordTypeKind::TsAnyKeyword)
}

fn void_type() -> TsType {
    keyword_type(TsKeywordTypeKind::TsVoidKeyword)
}

fn boolean_type() -> TsType {
    keyword_type(TsKeywordTypeKind::TsBooleanKeyword)
}

fn string_type() -> TsType {
    keyword_type(TsKeywordTypeKind::TsStringKeyword)
}

fn number_type() -> TsType {
    keyword_type(TsKeywordTypeKind::TsNumberKeyword)
}

fn big_int_type() -> TsType {
    keyword_type(TsKeywordTypeKind::TsBigIntKeyword)
}

fn keyword_type(kind: TsKeywordTypeKind) -> TsType {
    TsType::TsKeywordType(TsKeywordType {
        span: Span::default(),
        kind,
    })
}

fn uint8_array_type() -> TsType {
    ident_type(id("Uint8Array"))
}

fn ident_type(ident: Ident) -> TsType {
    TsType::TsTypeRef(TsTypeRef {
        span: Span::default(),
        type_name: TsEntityName::Ident(ident),
        type_params: None,
    })
}

fn file(
    c: &swc::Compiler,
    filename: impl Into<String>,
    module: &Module,
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

fn private_name(str: &'_ str) -> PrivateName {
    PrivateName {
        span: Span::default(),
        id: id(str),
    }
}

fn id(str: &'_ str) -> Ident {
    Ident::new(str.into(), Span::default())
}
