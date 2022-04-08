#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub declarations: Vec<Declaration>,
}

impl Module {
    pub fn new(declarations: Vec<Declaration>) -> Self {
        Self { declarations }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Export(ExportDeclaration),
    Import(ImportDeclaration),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportDeclaration {
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Literal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportSpecifier {
    Named(ExportNamedSpecifier),
    Namespace(ExportNamespaceSpecifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportNamedSpecifier {
    pub local: Identifier,
    pub imported: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportNamespaceSpecifier {
    pub local: Identifier,
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
