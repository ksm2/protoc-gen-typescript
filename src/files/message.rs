use crate::utils::*;
use protobuf::descriptor::{DescriptorProto, FieldDescriptorProto, FieldDescriptorProto_Type};
use protobuf::plugin::CodeGeneratorResponse_File;
use swc::common::util::take::Take;
use swc::common::Span;
use swc::ecmascript::ast::*;
use swc::Compiler;

pub fn message(c: &Compiler, message: &DescriptorProto) -> CodeGeneratorResponse_File {
    let name = message.get_name();

    let properties = message
        .field
        .iter()
        .map(field_prop)
        .map(ClassMember::PrivateProp)
        .collect();

    let module = module([
        named_import("google-protobuf", &["BinaryReader", "BinaryWriter"]).into(),
        ExportDecl {
            span: Span::default(),
            decl: ClassDecl {
                ident: id(name),
                declare: false,
                class: Class {
                    body: [properties, vec![serialize_method(message).into()]].concat(),
                    ..Take::dummy()
                },
            }
            .into(),
        }
        .into(),
    ]);

    super::file(c, format!("{name}.ts"), &module)
}

fn named_import(src: &str, specifiers: &[&str]) -> ImportDecl {
    ImportDecl {
        src: src.into(),
        specifiers: specifiers
            .iter()
            .map(|spec| ImportNamedSpecifier {
                span: Span::default(),
                local: id(spec),
                imported: None,
                is_type_only: false,
            })
            .map(ImportSpecifier::Named)
            .collect(),
        ..Take::dummy()
    }
}

fn serialize_method(message: &DescriptorProto) -> ClassMethod {
    ClassMethod {
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
                    type_ann: Some(to_type_ann(ident_type("BinaryWriter"))),
                }),
            }],
            body: Some(BlockStmt {
                span: Default::default(),
                stmts: message.field.iter().map(serialize_field).collect(),
            }),
            return_type: Some(to_type_ann(void_type())),
            ..Take::dummy()
        },
    }
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

    let then = Stmt::Block(BlockStmt::dummy());

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
        | FieldDescriptorProto_Type::TYPE_ENUM => int_expr(0),
        FieldDescriptorProto_Type::TYPE_BOOL => boolean_expr(false),
        FieldDescriptorProto_Type::TYPE_STRING => string_expr(""),
        FieldDescriptorProto_Type::TYPE_BYTES => new_uint8_array(),
        FieldDescriptorProto_Type::TYPE_INT64
        | FieldDescriptorProto_Type::TYPE_UINT64
        | FieldDescriptorProto_Type::TYPE_FIXED64
        | FieldDescriptorProto_Type::TYPE_SFIXED64
        | FieldDescriptorProto_Type::TYPE_SINT64 => big_int_expr(0.into()),
        _ => undefined(),
    }
}

fn new_uint8_array() -> Expr {
    Expr::New(NewExpr {
        span: Span::default(),
        callee: Box::new(Expr::Ident(id("Uint8Array"))),
        args: Some(vec![]),
        type_args: None,
    })
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
    ident_type("Uint8Array")
}
