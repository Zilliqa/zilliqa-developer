use crate::highlevel_ir::HighlevelIr;
use crate::highlevel_ir_pass::HighlevelIrPass;
use crate::highlevel_ir_pass_executor::HighlevelIrPassExecutor;
use crate::intermediate_name_generator::IntermediateNameGenerator;
use crate::symbol_table::SymbolTable;

use crate::passes::annotate_base_types::AnnotateBaseTypes;
use crate::passes::collect_type_definitions::CollectTypeDefinitionsPass;
use std::rc::Rc;

struct HighlevelIrPassManager {
    passes: Vec<Box<dyn HighlevelIrPass>>,
}

impl HighlevelIrPassManager {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn default_pipeline() -> Self {
        let mut ret = Self::new();

        ret.passes.push(Box::new(CollectTypeDefinitionsPass::new()));
        ret.passes.push(Box::new(AnnotateBaseTypes::new()));

        ret
    }

    pub fn add_pass(&mut self, pass: Box<dyn HighlevelIrPass>) {
        self.passes.push(pass);
    }

    pub fn run(&mut self, ir: &mut HighlevelIr) {
        for pass in &mut self.passes {
            ir.run_pass(pass.as_mut());
        }
    }
}
