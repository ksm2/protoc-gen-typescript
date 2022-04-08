use crate::pretty::Prettify;

mod ast;
mod pretty;

fn main() {
    let import1 = ast::ImportDeclaration {
        source: ast::Literal::String("./foo.js".to_string()),
        specifiers: vec![
            ast::ImportSpecifier::Named(ast::ImportNamedSpecifier {
                local: ast::Identifier("test".to_string()),
                imported: ast::Identifier("test".to_string()),
            }),
            ast::ImportSpecifier::Default(ast::ImportDefaultSpecifier {
                local: ast::Identifier("foo".to_string()),
            }),
        ],
    };

    let import2 = ast::ImportDeclaration {
        source: ast::Literal::String("./foo.js".to_string()),
        specifiers: vec![ast::ImportSpecifier::Namespace(
            ast::ImportNamespaceSpecifier {
                local: ast::Identifier("FOO".to_string()),
            },
        )],
    };

    let module = ast::Module {
        declarations: vec![
            ast::Declaration::Import(import1),
            ast::Declaration::Import(import2),
        ],
    };

    println!("{}", module.to_pretty(20));
    println!("{}", module.to_pretty(100));
}
