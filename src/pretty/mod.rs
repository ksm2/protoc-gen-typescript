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

        let default_import = self
            .specifiers
            .iter()
            .filter(|s| matches!(s, ImportSpecifier::Default(_)))
            .map(|s| s.to_doc().append(comma()).append(space()))
            .next()
            .unwrap_or_else(RcDoc::nil);

        let named_imports = self
            .specifiers
            .iter()
            .filter(|s| matches!(s, ImportSpecifier::Named(_)))
            .map(|s| s.to_doc())
            .collect::<Vec<_>>();

        let named_imports = if named_imports.is_empty() {
            RcDoc::nil()
        } else {
            braces(comma_separated(named_imports))
        };

        import_decl(default_import.append(named_imports), self.source.to_doc())
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
