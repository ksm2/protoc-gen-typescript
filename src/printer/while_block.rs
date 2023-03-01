use super::Block;
use super::Module;

pub struct WhileBlock<'a> {
    module: &'a mut Module,
}

impl<'a> WhileBlock<'a> {
    pub(super) fn new(module: &'a mut Module, cond: &str) -> Self {
        module.print_indentation();
        module.println(&format!("while ({cond}) {{"));
        module.indent();
        Self { module }
    }

    pub fn end(self) {
        self.module.unindent();
        self.module.print_indentation();
        self.module.println("}");
    }
}

impl<'a> Block for WhileBlock<'a> {
    fn module(&mut self) -> &mut Module {
        self.module
    }
}
