use super::Method;
use super::Module;
use super::Property;

pub struct Class<'a> {
    module: &'a mut Module,
}

impl<'a> Class<'a> {
    pub(super) fn new(module: &'a mut Module, name: impl Into<String>) -> Self {
        let name = name.into();
        module.print_indentation();
        module.println(&format!("export class {name} {{"));
        module.indent();

        Self { module }
    }

    pub fn blank(&mut self) {
        self.module.blank();
    }

    pub fn property(&mut self, name: &str) -> Property {
        Property::new(self.module, name)
    }

    pub fn method(&mut self, name: &str, params: &[(&str, &str); 1], return_type: &str) -> Method {
        Method::new(self.module, name, params, return_type)
    }

    pub fn end(self) {
        self.module.unindent();
        self.module.print_indentation();
        self.module.println("}");
    }
}
