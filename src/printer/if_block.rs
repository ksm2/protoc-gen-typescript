use super::Module;
use crate::printer::block::Block;

pub struct IfBlock<'a> {
    module: &'a mut Module,
}

impl<'a> IfBlock<'a> {
    pub(super) fn new(module: &'a mut Module, cond: &str) -> Self {
        module.print_indentation();
        module.println(&format!("if ({cond}) {{"));
        module.indent();
        Self { module }
    }

    pub fn end(self) {
        self.module.unindent();
        self.module.print_indentation();
        self.module.println("}");
    }
}

impl<'a> Block for IfBlock<'a> {
    fn module(&mut self) -> &mut Module {
        self.module
    }
}
