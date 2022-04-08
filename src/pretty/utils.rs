use pretty::{Pretty, RcAllocator, RcDoc};

pub fn import_decl<'a>(doc1: RcDoc<'a>, doc2: RcDoc<'a>) -> RcDoc<'a> {
    RcDoc::text("import")
        .append(space())
        .append(doc1)
        .append(space())
        .append(RcDoc::text("from"))
        .append(space())
        .append(doc2)
        .append(semi())
}

pub fn as_stmt<'a>(doc1: RcDoc<'a>, doc2: RcDoc<'a>) -> RcDoc<'a> {
    doc1.append(space())
        .append(RcDoc::text("as"))
        .append(space())
        .append(doc2)
}

pub fn escape_str(str: &str) -> String {
    str.replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}

pub fn space<'a>() -> RcDoc<'a> {
    RcDoc::text(" ")
}

pub fn asterisk<'a>() -> RcDoc<'a> {
    RcDoc::text("*")
}

pub fn comma_separated<'a, I>(docs: I) -> RcDoc<'a>
where
    I: IntoIterator,
    I::Item: Pretty<'a, RcAllocator>,
{
    RcDoc::intersperse(docs, comma().append(RcDoc::space()))
}

pub fn comma<'a>() -> RcDoc<'a> {
    RcDoc::text(",")
}

pub fn semi<'a>() -> RcDoc<'a> {
    RcDoc::text(";")
}

pub fn quote<'a>() -> RcDoc<'a> {
    RcDoc::text("\"")
}

pub fn braces(doc: RcDoc) -> RcDoc {
    RcDoc::group(
        RcDoc::text("{")
            .append(RcDoc::line().append(doc).nest(2))
            .append(RcDoc::line())
            .append(RcDoc::text("}")),
    )
}
