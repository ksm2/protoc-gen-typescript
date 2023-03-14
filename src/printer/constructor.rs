use super::Module;
use crate::printer::block::Block;

pub struct Constructor<'a> {
    module: &'a mut Module,
}

impl<'a> Constructor<'a> {
    pub(super) fn new(module: &'a mut Module, params: &[(&str, &str)]) -> Self {
        let params = params
            .iter()
            .map(|(param_name, param_type)| format!("{param_name}: {param_type}"))
            .collect::<Vec<_>>()
            .join(", ");
        module.print_indentation();
        module.println(&format!("constructor({params}) {{"));
        module.indent();
        Self { module }
    }

    pub fn end(self) {
        self.module.unindent();
        self.module.print_indentation();
        self.module.println("}");
    }
}

impl<'a> Block for Constructor<'a> {
    fn module(&mut self) -> &mut Module {
        self.module
    }
}
