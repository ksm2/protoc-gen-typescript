use pretty::{Pretty, RcAllocator, RcDoc};

enum SExpr {
    Identifier(String),
    List(Vec<SExpr>),
}

impl SExpr {
    pub fn to_doc(&self) -> RcDoc<()> {
        use Self::*;

        match *self {
            Identifier(ref i) => RcDoc::as_string(i),
            List(ref v) => RcDoc::text("import")
                .append(RcDoc::space())
                .append(braces(comma_separated(v.into_iter().map(|x| x.to_doc()))))
                .append(RcDoc::space())
                .append(RcDoc::text("from"))
                .append(RcDoc::space())
                .append(RcDoc::text("\"./foo.js\""))
                .append(semi()),
        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

fn braces(doc: RcDoc<()>) -> RcDoc<()> {
    RcDoc::group(
        RcDoc::text("{")
            .append(RcDoc::line().append(doc).nest(2))
            .append(RcDoc::line())
            .append(RcDoc::text("}")),
    )
}

fn brackets(doc: RcDoc<()>) -> RcDoc<()> {
    RcDoc::group(
        RcDoc::text("[")
            .append(RcDoc::line_().append(doc).nest(2))
            .append(RcDoc::line_())
            .append(RcDoc::text("]")),
    )
}

fn comma_separated<'a, I>(docs: I) -> RcDoc<'a, ()>
where
    I: IntoIterator,
    I::Item: Pretty<'a, RcAllocator, ()>,
{
    RcDoc::intersperse(docs, comma().append(RcDoc::line())).append(trailing_comma())
}

fn trailing_comma<'a>() -> RcDoc<'a, ()> {
    comma().flat_alt(RcDoc::nil())
}

fn comma<'a>() -> RcDoc<'a, ()> {
    RcDoc::text(",")
}

fn semi<'a>() -> RcDoc<'a, ()> {
    RcDoc::text(";")
}

fn main() {
    let list = SExpr::List(vec![
        SExpr::Identifier("lorem".to_string()),
        SExpr::Identifier("ipsum".to_string()),
        SExpr::Identifier("dolor".to_string()),
        SExpr::Identifier("amet".to_string()),
    ]);
    println!("{}", list.to_pretty(80));
    println!("{}", list.to_pretty(17));
}
