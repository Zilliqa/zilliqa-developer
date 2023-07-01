use crate::highlevel_ir_pass::HighlevelIrPass;
use crate::constants::{TraversalResult};

pub trait HighlevelIrPassExecutor {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String>;    
}
