use num_bigint::BigInt;
use swc::common::Span;
use swc::ecmascript::ast::*;

pub fn module<I>(decl: I) -> Module
where
    I: IntoIterator<Item = ModuleDecl>,
{
    Module {
        span: Span::default(),
        shebang: None,
        body: decl.into_iter().map(ModuleItem::ModuleDecl).collect(),
    }
}

pub fn private_name(str: &'_ str) -> PrivateName {
    PrivateName {
        span: Span::default(),
        id: id(str),
    }
}

pub fn ident_type(ident: &str) -> TsType {
    TsType::TsTypeRef(TsTypeRef {
        span: Span::default(),
        type_name: TsEntityName::Ident(id(ident)),
        type_params: None,
    })
}

pub fn id(str: &'_ str) -> Ident {
    Ident::new(str.into(), Span::default())
}

pub fn int_expr(num: usize) -> Expr {
    Expr::Lit(Lit::Num(num.into()))
}

pub fn boolean_expr(bool: bool) -> Expr {
    Expr::Lit(Lit::Bool(bool.into()))
}

pub fn string_expr(str: &str) -> Expr {
    Expr::Lit(Lit::Str(str.into()))
}

pub fn big_int_expr(big_int: BigInt) -> Expr {
    Expr::Lit(Lit::BigInt(big_int.into()))
}
