mod index;
mod message;

pub use index::index;
pub use message::message;

use protobuf::plugin::CodeGeneratorResponse_File;
use swc::common::collections::AHashMap;
use swc::config::SourceMapsConfig;
use swc::ecmascript::ast::*;

fn file(
    c: &swc::Compiler,
    filename: impl Into<String>,
    module: &Module,
) -> CodeGeneratorResponse_File {
    let ast_printed = c
        .print(
            module,
            None,
            None,
            false,
            EsVersion::Es2022,
            SourceMapsConfig::Bool(false),
            &AHashMap::default(),
            None,
            false,
            None,
        )
        .expect("Failed to print");

    let mut file = CodeGeneratorResponse_File::new();
    file.set_name(filename.into());
    file.set_content(ast_printed.code);
    file
}
