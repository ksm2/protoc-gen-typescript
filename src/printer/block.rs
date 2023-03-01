use super::IfBlock;
use super::Module;

pub trait Block {
    fn if_(&mut self, cond: &str) -> IfBlock {
        IfBlock::new(self.module(), cond)
    }

    fn call(&mut self, code: &str) {
        self.module().print_indentation();
        self.module().println(code);
    }

    fn module(&mut self) -> &mut Module;
}
