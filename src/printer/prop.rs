use super::Module;

pub struct Property<'a> {
    name: Option<String>,
    module: &'a mut Module,
}

impl<'a> Property<'a> {
    pub(super) fn new(module: &'a mut Module, name: &str) -> Self {
        module.print_indentation();
        Self {
            name: Some(name.to_string()),
            module,
        }
    }

    #[must_use]
    pub fn private(self) -> Self {
        self.module.print("private ");
        self
    }

    #[must_use]
    pub fn readonly(self) -> Self {
        self.module.print("readonly ");
        self
    }

    #[must_use]
    pub fn optional(mut self) -> Self {
        self.print_name();
        self.module.print("?");
        self
    }

    #[must_use]
    pub fn typed(mut self, type_: &str) -> Self {
        self.print_name();
        self.module.print(&format!(": {type_}"));
        self
    }

    #[must_use]
    pub fn assign(mut self, expr: &str) -> Self {
        self.print_name();
        self.module.print(&format!(" = {expr}"));
        self
    }

    pub fn end(mut self) {
        self.print_name();
        self.module.println(";");
    }

    fn print_name(&mut self) {
        if let Some(name) = self.name.take() {
            self.module.print(&name);
        }
    }
}
