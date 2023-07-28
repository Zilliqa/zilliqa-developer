use crate::intermediate_representation::highlevel_ir::HighlevelIr;
use crate::intermediate_representation::highlevel_ir_pass::HighlevelIrPass;
use crate::passes::annotate_base_types::AnnotateBaseTypes;
use crate::passes::collect_type_definitions::CollectTypeDefinitionsPass;

pub struct HighlevelIrPassManager {
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

    pub fn run(&mut self, ir: &mut HighlevelIr) -> Result<u32, String> {
        // TODO: Make self immutable and copy pass before running it on IR
        for pass in &mut self.passes {
            let _ = ir.run_pass(pass.as_mut());
        }
        Ok(0)
    }
}
