use super::Module;

pub struct EnumItem<'a> {
    module: &'a mut Module,
}

impl<'a> EnumItem<'a> {
    pub(super) fn new(module: &'a mut Module, name: &str) -> Self {
        module.print_indentation();
        module.print(name);
        Self { module }
    }

    #[must_use]
    pub fn value(self, expr: &str) -> Self {
        self.module.print(&format!(" = {expr}"));
        self
    }

    pub fn end(self) {
        self.module.println(",");
    }
}
