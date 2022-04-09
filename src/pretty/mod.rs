use pretty::RcDoc;

use crate::ast::*;

use self::utils::*;

mod utils;

pub trait Prettify {
    fn to_doc(&self) -> RcDoc;

    fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Prettify for Module {
    fn to_doc(&self) -> RcDoc {
        RcDoc::intersperse(
            self.declarations.iter().map(|d| d.to_doc()),
            RcDoc::hardline(),
        )
        .append(RcDoc::hardline())
    }
}

impl Prettify for Declaration {
    fn to_doc(&self) -> RcDoc {
        match self {
            Declaration::Import(ref id) => id.to_doc(),
            Declaration::Export(ref id) => id.to_doc(),
        }
    }
}

impl Prettify for ExportDeclaration {
    fn to_doc(&self) -> RcDoc {
        RcDoc::text("export").append(space()).append(match self {
            ExportDeclaration::From(ref clause) => clause.to_doc(),
            ExportDeclaration::Named(ref named) => named.to_doc(),
        })
    }
}

impl Prettify for FromClause {
    fn to_doc(&self) -> RcDoc {
        self.export
            .to_doc()
            .append(space())
            .append(from_clause(self.source.to_doc()))
    }
}

impl Prettify for ExportFromClause {
    fn to_doc(&self) -> RcDoc {
        match self {
            ExportFromClause::Namespace(ref ns) => ns.to_doc(),
            ExportFromClause::Named(ref named) => named.to_doc(),
        }
    }
}

impl Prettify for NamedExports {
    fn to_doc(&self) -> RcDoc {
        braces(comma_separated_(self.exports.iter().map(|e| e.to_doc())))
    }
}

impl Prettify for ExportNamedSpecifier {
    fn to_doc(&self) -> RcDoc {
        if self.local == self.imported {
            self.local.to_doc()
        } else {
            as_stmt(self.imported.to_doc(), self.local.to_doc())
        }
    }
}

impl Prettify for ExportNamespaceSpecifier {
    fn to_doc(&self) -> RcDoc {
        if let Some(local) = &self.local {
            as_stmt(asterisk(), local.to_doc())
        } else {
            asterisk()
        }
    }
}

impl Prettify for ImportDeclaration {
    fn to_doc(&self) -> RcDoc {
        let namespace_import = self
            .specifiers
            .iter()
            .filter(|s| matches!(s, ImportSpecifier::Namespace(_)))
            .map(|s| s.to_doc())
            .next();

        if let Some(doc) = namespace_import {
            return import_decl(doc, self.source.to_doc());
        }

        let mut default_import = self
            .specifiers
            .iter()
            .filter(|s| matches!(s, ImportSpecifier::Default(_)))
            .map(|s| s.to_doc())
            .next()
            .into_iter()
            .collect::<Vec<_>>();

        let named_imports = self
            .specifiers
            .iter()
            .filter(|s| matches!(s, ImportSpecifier::Named(_)))
            .map(|s| s.to_doc())
            .collect::<Vec<_>>();

        let mut named_imports = if named_imports.is_empty() {
            Vec::new()
        } else {
            vec![braces(comma_separated(named_imports))]
        };

        default_import.append(&mut named_imports);
        let imports = comma_separated(default_import);

        import_decl(imports, self.source.to_doc())
    }
}

impl Prettify for ImportSpecifier {
    fn to_doc(&self) -> RcDoc {
        match self {
            ImportSpecifier::Named(ref sp) => sp.to_doc(),
            ImportSpecifier::Default(ref sp) => sp.to_doc(),
            ImportSpecifier::Namespace(ref sp) => sp.to_doc(),
        }
    }
}

impl Prettify for ImportNamedSpecifier {
    fn to_doc(&self) -> RcDoc {
        if self.local == self.imported {
            self.local.to_doc()
        } else {
            as_stmt(self.imported.to_doc(), self.local.to_doc())
        }
    }
}

impl Prettify for ImportDefaultSpecifier {
    fn to_doc(&self) -> RcDoc {
        self.local.to_doc()
    }
}

impl Prettify for ImportNamespaceSpecifier {
    fn to_doc(&self) -> RcDoc {
        as_stmt(asterisk(), self.local.to_doc())
    }
}

impl Prettify for Literal {
    fn to_doc(&self) -> RcDoc {
        match self {
            Literal::String(ref str) => quote()
                .append(RcDoc::as_string(escape_str(str)))
                .append(quote()),
        }
    }
}

impl Prettify for Identifier {
    fn to_doc(&self) -> RcDoc {
        RcDoc::text(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Prettify;

    macro_rules! assert_pretty {
        ($expected: expr, $actual: expr) => {
            assert_eq!($expected, $actual.to_pretty(80));
        };
    }

    #[test]
    fn named_imports() {
        assert_pretty!(
            "import { foo as bar, baz } from \"./default\";",
            ImportDeclaration::new(
                vec![
                    ImportSpecifier::Named(ImportNamedSpecifier {
                        imported: Identifier("foo".to_string()),
                        local: Identifier("bar".to_string()),
                    }),
                    ImportSpecifier::Named(ImportNamedSpecifier {
                        imported: Identifier("baz".to_string()),
                        local: Identifier("baz".to_string()),
                    })
                ],
                Literal::String("./default".to_string()),
            )
        );
    }

    #[test]
    fn default_import() {
        assert_pretty!(
            "import default from \"./default\";",
            ImportDeclaration::new(
                vec![ImportSpecifier::Default(ImportDefaultSpecifier {
                    local: Identifier("default".to_string())
                })],
                Literal::String("./default".to_string()),
            )
        );
    }

    #[test]
    fn default_and_named_import() {
        assert_pretty!(
            "import default, { foo as bar } from \"./default\";",
            ImportDeclaration::new(
                vec![
                    ImportSpecifier::Default(ImportDefaultSpecifier {
                        local: Identifier("default".to_string())
                    }),
                    ImportSpecifier::Named(ImportNamedSpecifier {
                        imported: Identifier("foo".to_string()),
                        local: Identifier("bar".to_string()),
                    })
                ],
                Literal::String("./default".to_string()),
            )
        );
    }

    #[test]
    fn ns_import() {
        assert_pretty!(
            "import * as ns from \"./ns\";",
            ImportDeclaration::new(
                vec![ImportSpecifier::Namespace(ImportNamespaceSpecifier {
                    local: Identifier("ns".to_string())
                })],
                Literal::String("./ns".to_string()),
            )
        );
    }

    #[test]
    fn string_literal() {
        assert_pretty!("\"./ns\"", Literal::String("./ns".to_string()));
    }
}
