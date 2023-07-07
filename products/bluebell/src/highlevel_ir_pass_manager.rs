use crate::highlevel_ir_pass::HighlevelIrPass;

use crate::highlevel_ir_pass_executor::HighlevelIrPassExecutor;

use crate::highlevel_ir::HighlevelIr;

struct HighlevelIrPassManager {
    passes: Vec<Box<dyn HighlevelIrPass>>,
}

impl HighlevelIrPassManager {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn add_pass(&mut self, pass: Box<dyn HighlevelIrPass>) {
        self.passes.push(pass);
    }

    pub fn run(&mut self, ir: &mut HighlevelIr) {
        for pass in &mut self.passes {
            ir.visit(pass);
        }
    }
}
