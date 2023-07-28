use crate::intermediate_representation::pass::IrPass;
use crate::intermediate_representation::primitives::IntermediateRepresentation;
use crate::passes::annotate_base_types::AnnotateBaseTypes;
use crate::passes::collect_type_definitions::CollectTypeDefinitionsPass;

pub struct PassManager {
    passes: Vec<Box<dyn IrPass>>,
}

impl PassManager {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn default_pipeline() -> Self {
        let mut ret = Self::new();

        ret.passes.push(Box::new(CollectTypeDefinitionsPass::new()));
        ret.passes.push(Box::new(AnnotateBaseTypes::new()));

        ret
    }

    pub fn add_pass(&mut self, pass: Box<dyn IrPass>) {
        self.passes.push(pass);
    }

    pub fn run(&mut self, ir: &mut IntermediateRepresentation) -> Result<u32, String> {
        // TODO: Make self immutable and copy pass before running it on IR
        for pass in &mut self.passes {
            let _ = ir.run_pass(pass.as_mut());
        }
        Ok(0)
    }
}
