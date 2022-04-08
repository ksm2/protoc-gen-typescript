#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Import(ImportDeclaration),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDeclaration {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: Literal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportSpecifier {
    Named(ImportNamedSpecifier),
    Default(ImportDefaultSpecifier),
    Namespace(ImportNamespaceSpecifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportNamedSpecifier {
    pub local: Identifier,
    pub imported: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDefaultSpecifier {
    pub local: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportNamespaceSpecifier {
    pub local: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);
