use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::*;
use crate::highlevel_ir_pass::HighlevelIrPass;

pub trait HighlevelIrPassExecutor {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String>;
}

impl HighlevelIrPassExecutor for IrIndentifierKind {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_symbol_kind(TreeTraversalMode::Enter, self)?;

        match ret {
            TraversalResult::Continue => emitter.visit_symbol_kind(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for IrIdentifier {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_symbol_name(TreeTraversalMode::Enter, self);

        // TODO: visit children, if 'ret' is TraversalResult::Continuen
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            self.kind.visit(emitter)?;
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;

        match children_ret {
            TraversalResult::Continue => emitter.visit_symbol_name(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for EnumValue {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_enum_value(TreeTraversalMode::Enter, self);
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            self.name.visit(emitter);
            if let Some(data) = &mut self.data {
                data.visit(emitter)
            } else {
                ret
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.visit_enum_value(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Tuple {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_tuple(TreeTraversalMode::Enter, self);
        // visit children, if 'ret' is TraversalResult::Continue
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for field in self.fields.iter_mut() {
                field.visit(emitter)?;
            }

            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.visit_tuple(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Variant {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_variant(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for field in &mut self.fields {
                let result = field.visit(emitter);
                if result != Ok(TraversalResult::Continue) {
                    return result;
                }
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        if let Ok(TraversalResult::Continue) = children_ret {
            emitter.visit_variant(TreeTraversalMode::Exit, self)
        } else {
            Ok(TraversalResult::SkipChildren)
        }
    }
}

impl HighlevelIrPassExecutor for VariableDeclaration {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_variable_declaration(TreeTraversalMode::Enter, self);
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            self.typename.visit(emitter)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_variable_declaration(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

use std::error::Error;
impl HighlevelIrPassExecutor for Operation {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_operation(TreeTraversalMode::Enter, self);

        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            match self {
                Operation::Jump(identifier) => identifier.visit(emitter),
                Operation::MemLoad
                | Operation::MemStore
                | Operation::AcceptTransfer
                | Operation::PhiNode(_) => Ok(TraversalResult::Continue),
                Operation::ConditionalJump {
                    expression,
                    on_success,
                    on_failure,
                } => {
                    expression.visit(emitter)?;
                    on_success.visit(emitter)?;
                    on_failure.visit(emitter)
                }
                Operation::IsEqual { left, right } => {
                    left.visit(emitter)?;
                    right.visit(emitter)?;
                    Ok(TraversalResult::Continue)
                }
                Operation::CallExternalFunction { name, arguments }
                | Operation::CallFunction { name, arguments } => {
                    name.visit(emitter)?;
                    for arg in arguments {
                        arg.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
                Operation::CallStaticFunction {
                    name,
                    owner,
                    arguments,
                }
                | Operation::CallMemberFunction {
                    name,
                    owner,
                    arguments,
                } => {
                    if let Some(owner) = owner {
                        owner.visit(emitter)?;
                    }
                    name.visit(emitter)?;
                    for arg in arguments {
                        arg.visit(emitter)?;
                    }
                    Ok(TraversalResult::Continue)
                }
                Operation::ResolveSymbol { symbol } => {
                    symbol.visit(emitter)?;
                    Ok(TraversalResult::Continue)
                }
                Operation::Literal { data: _, typename } => {
                    typename.visit(emitter)?;
                    Ok(TraversalResult::Continue)
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.visit_operation(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Instruction {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_instruction(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            if let Some(ssa) = &mut self.ssa_name {
                ssa.visit(emitter)?;
            }
            if let Some(ret) = &mut self.result_type {
                ret.visit(emitter)?;
            }

            self.operation.visit(emitter)?;
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.visit_instruction(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for FunctionBlock {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_function_block(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.name.visit(emitter)?;
            for instr in self.instructions.iter_mut() {
                instr.visit(emitter)?;
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_function_block(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl HighlevelIrPassExecutor for FunctionBody {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_function_body(TreeTraversalMode::Enter, self);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for block in self.blocks.iter_mut() {
                block.visit(emitter)?;
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret {
            Ok(TraversalResult::Continue) => {
                emitter.visit_function_body(TreeTraversalMode::Exit, self)
            }
            _ => children_ret,
        }
    }
}

impl HighlevelIrPassExecutor for ConcreteType {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_concrete_type(TreeTraversalMode::Enter, self)?;
        let ret = if let TraversalResult::Continue = ret {
            // visit children
            match self {
                ConcreteType::Tuple {
                    name,
                    namespace,
                    data_layout,
                } => {
                    name.visit(emitter)?;
                    namespace.visit(emitter)?;
                    data_layout.visit(emitter)?;
                }
                ConcreteType::Variant {
                    name,
                    namespace,
                    data_layout,
                } => {
                    name.visit(emitter)?;
                    namespace.visit(emitter)?;

                    data_layout.visit(emitter)?;
                }
            }
            TraversalResult::Continue
        } else {
            ret
        };
        match ret {
            TraversalResult::Continue => emitter.visit_concrete_type(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for FunctionKind {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_function_kind(TreeTraversalMode::Enter, self)?;

        // No children

        match ret {
            TraversalResult::Continue => emitter.visit_function_kind(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for ConcreteFunction {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_concrete_function(TreeTraversalMode::Enter, self)?;
        let children_ret = if ret == TraversalResult::Continue {
            let _ = self.name.visit(emitter)?;
            let _ = self.namespace.visit(emitter)?;
            let _ = self.function_kind.visit(emitter)?;
            if let Some(rt) = &mut self.return_type {
                // TODO: Change when rt is an IrIdentifier let _ = rt.visit(emitter)?;
            }
            for (i, arg) in self.arguments.iter_mut().enumerate() {
                let _ = arg.visit(emitter)?;
            }

            let _ = self.body.visit(emitter)?;

            TraversalResult::Continue
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_concrete_function(TreeTraversalMode::Exit, self)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for HighlevelIr {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_highlevel_ir(TreeTraversalMode::Enter, self)?;
        let children_ret = if let TraversalResult::Continue = ret {
            for type_def in &mut self.type_definitions {
                type_def.visit(emitter)?;
            }

            for function_def in &mut self.function_definitions {
                function_def.visit(emitter)?;
            }

            TraversalResult::Continue
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Continue => emitter.visit_highlevel_ir(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}
