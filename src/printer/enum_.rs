use super::EnumItem;
use super::Module;

pub struct Enum<'a> {
    module: &'a mut Module,
}

impl<'a> Enum<'a> {
    pub(super) fn new(module: &'a mut Module, name: impl Into<String>) -> Self {
        let name = name.into();
        module.print_indentation();
        module.println(&format!("export enum {name} {{"));
        module.indent();

        Self { module }
    }

    pub fn blank(&mut self) {
        self.module.blank();
    }

    pub fn item(&mut self, name: &str) -> EnumItem {
        EnumItem::new(self.module, name)
    }

    pub fn end(self) {
        self.module.unindent();
        self.module.print_indentation();
        self.module.println("}");
    }
}
