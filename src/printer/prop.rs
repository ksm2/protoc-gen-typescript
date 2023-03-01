use super::Module;

pub struct Property<'a> {
    module: &'a mut Module,
}

impl<'a> Property<'a> {
    pub(super) fn new(module: &'a mut Module, name: &str) -> Self {
        module.print_indentation();
        module.print(name);
        Self { module }
    }

    #[must_use]
    pub fn optional(self) -> Self {
        self.module.print("?");
        self
    }

    #[must_use]
    pub fn typed(self, type_: &str) -> Self {
        self.module.print(&format!(": {type_}"));
        self
    }

    #[must_use]
    pub fn assign(self, expr: &str) -> Self {
        self.module.print(&format!(" = {expr}"));
        self
    }

    pub fn end(self) {
        self.module.println(";");
    }
}
