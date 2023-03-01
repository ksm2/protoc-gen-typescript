use super::Module;
use crate::printer::block::Block;

pub struct Method<'a> {
    module: &'a mut Module,
}

impl<'a> Method<'a> {
    pub(super) fn new(
        module: &'a mut Module,
        name: &str,
        params: &[(&str, &str)],
        return_type: &str,
    ) -> Self {
        let params = params
            .iter()
            .map(|(param_name, param_type)| format!("{param_name}: {param_type}"))
            .collect::<Vec<_>>()
            .join(", ");
        module.print_indentation();
        module.println(&format!("{name}({params}): {return_type} {{"));
        module.indent();
        Self { module }
    }

    pub fn end(self) {
        self.module.unindent();
        self.module.print_indentation();
        self.module.println("}");
    }
}

impl<'a> Block for Method<'a> {
    fn module(&mut self) -> &mut Module {
        self.module
    }
}
