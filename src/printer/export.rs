use crate::printer::Module;

pub struct Export<'a> {
    module: &'a mut Module,
    names: &'a [&'a str],
}

impl<'a> Export<'a> {
    pub(super) fn new(module: &'a mut Module, names: &'a [&'a str]) -> Self {
        Self { module, names }
    }

    pub fn from(self, filename: &str) {
        let names = self.names.join(", ");
        self.module.print_indentation();
        self.module
            .println(&format!("export {{ {names} }} from \"{filename}\";"));
    }
}
