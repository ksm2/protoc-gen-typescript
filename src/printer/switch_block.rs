use super::Block;
use super::Module;

pub struct SwitchBlock<'a> {
    module: &'a mut Module,
}

impl<'a> SwitchBlock<'a> {
    pub(super) fn new(module: &'a mut Module, cond: &str) -> Self {
        module.print_indentation();
        module.println(&format!("switch ({cond}) {{"));
        module.indent();
        Self { module }
    }

    pub fn case(&mut self, value: &str) -> CaseBlock {
        CaseBlock::new(self.module, value)
    }

    pub fn end(self) {
        self.module.unindent();
        self.module.print_indentation();
        self.module.println("}");
    }
}

pub struct CaseBlock<'a> {
    module: &'a mut Module,
}

impl<'a> CaseBlock<'a> {
    pub(super) fn new(module: &'a mut Module, value: &str) -> Self {
        module.print_indentation();
        module.println(&format!("case {value}: {{"));
        module.indent();
        Self { module }
    }

    pub fn end(self) {
        self.module.print_indentation();
        self.module.println("break;");

        self.module.unindent();
        self.module.print_indentation();
        self.module.println("}");
    }
}

impl<'a> Block for CaseBlock<'a> {
    fn module(&mut self) -> &mut Module {
        self.module
    }
}
