use super::IfBlock;
use super::Module;
use super::SwitchBlock;
use super::WhileBlock;

pub trait Block {
    fn if_(&mut self, cond: &str) -> IfBlock {
        IfBlock::new(self.module(), cond)
    }

    fn while_(&mut self, cond: &str) -> WhileBlock {
        WhileBlock::new(self.module(), cond)
    }

    fn switch(&mut self, cond: &str) -> SwitchBlock {
        SwitchBlock::new(self.module(), cond)
    }

    fn call(&mut self, code: &str) {
        self.module().print_indentation();
        self.module().println(code);
    }

    fn module(&mut self) -> &mut Module;
}
