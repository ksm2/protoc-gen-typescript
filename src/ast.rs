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
pub enum ExportDeclaration {
    From(FromClause),
    Named(NamedExports),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromClause {
    pub export: ExportFromClause,
    pub source: Literal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportFromClause {
    Namespace(ExportNamespaceSpecifier),
    Named(NamedExports),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedExports {
    pub exports: Vec<ExportNamedSpecifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportNamedSpecifier {
    pub local: Identifier,
    pub imported: Identifier,
}

impl ExportNamedSpecifier {
    pub fn new(local: Identifier) -> Self {
        Self::with_alias(local.clone(), local)
    }

    pub fn with_alias(local: Identifier, imported: Identifier) -> Self {
        Self { local, imported }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportNamespaceSpecifier {
    pub local: Option<Identifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDeclaration {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: Literal,
}

impl ImportDeclaration {
    pub fn new(specifiers: Vec<ImportSpecifier>, source: Literal) -> Self {
        Self { specifiers, source }
    }
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
