use crate::constants::TraversalResult;
use crate::highlevel_ir::*;
use crate::highlevel_ir_pass::HighlevelIrPass;

pub trait HighlevelIrPassExecutor {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String>;
}

impl HighlevelIrPassExecutor for IrIndentifierKind {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_symbol_kind(TreeTraversalMode::Enter, &mut self.clone());
        // TODO: visit children, if 'ret' is TraversalResult::Continuen
        emitter.visit_symbol_kind(TreeTraversalMode::Exit, &mut self.clone())
    }
}

impl HighlevelIrPassExecutor for IrIdentifier {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_symbol_name(TreeTraversalMode::Enter, &mut self.clone());
        // TODO: visit children, if 'ret' is TraversalResult::Continuen

        emitter.visit_symbol_name(TreeTraversalMode::Exit, &mut self.clone())
    }
}

impl HighlevelIrPassExecutor for EnumValue {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_enum_value(TreeTraversalMode::Enter, &mut self.clone());
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            if let Some(data) = &self.data {
                data.visit(emitter)
            } else {
                ret
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_enum_value(TreeTraversalMode::Exit, &mut self.clone())
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Tuple {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_tuple(TreeTraversalMode::Enter, &mut self.clone());
        // visit children, if 'ret' is TraversalResult::Continue
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.fields
                .iter()
                .map(|field| field.visit(emitter))
                .find(|r| *r == Err(String::from("Failure")))
                .unwrap_or(Ok(TraversalResult::Continue))
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_tuple(TreeTraversalMode::Exit, &mut self.clone())
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Variant {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_variant(TreeTraversalMode::Enter, &mut self.clone());
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
            emitter.visit_variant(TreeTraversalMode::Exit, &mut self.clone())
        } else {
            Ok(TraversalResult::SkipChildren)
        }
    }
}

impl HighlevelIrPassExecutor for Identifier {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_identifier(TreeTraversalMode::Enter, &mut self.clone());
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            match self {
                Identifier::ComponentName(name) => {
                    IrIdentifier::new(name.clone(), IrIndentifierKind::ComponentName).visit(emitter)
                }
                Identifier::TypeName(name) => {
                    IrIdentifier::new(name.clone(), IrIndentifierKind::TypeName).visit(emitter)
                }
                Identifier::Event(name) => {
                    IrIdentifier::new(name.clone(), IrIndentifierKind::Event).visit(emitter)
                }
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_identifier(TreeTraversalMode::Exit, &mut self.clone())
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for VariableDeclaration {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_variable_declaration(TreeTraversalMode::Enter, &mut self.clone());
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            self.typename.visit(emitter)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_variable_declaration(TreeTraversalMode::Exit, &mut self.clone())
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

use std::error::Error;
impl HighlevelIrPassExecutor for Operation {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_operation(TreeTraversalMode::Enter, self)?;
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            match self {
                Operation::Jump(identifier)
                | Operation::MemLoad
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
                Operation::IsEqual { left, right }
                | Operation::CallFunction {
                    name: left,
                    arguments: right,
                }
                | Operation::CallStaticFunction {
                    name: left,
                    owner: _,
                    arguments: right,
                }
                | Operation::CallMemberFunction {
                    name: left,
                    owner: right,
                    arguments: _,
                } => {
                    left.visit(emitter)?;
                    right.visit(emitter)
                }
                Operation::CallExternalFunction { name, arguments }
                | Operation::ResolveSymbol { symbol: name }
                | Operation::Literal {
                    data: name,
                    typename: arguments,
                } => {
                    name.visit(emitter)?;
                    for argument in arguments {
                        argument.visit(emitter)?;
                    }
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
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_instruction(TreeTraversalMode::Enter, &mut self.clone());
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.ssa_name
                .as_ref()
                .map(|name| name.visit(emitter))
                .unwrap_or(ret)
                .and_then(|r| match r {
                    TraversalResult::Continue => self
                        .result_type
                        .as_ref()
                        .map(|res| res.visit(emitter))
                        .unwrap_or(ret),
                    _ => ret,
                })
                .and_then(|r| match r {
                    TraversalResult::Continue => self.operation.visit(emitter),
                    _ => ret,
                })
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_instruction(TreeTraversalMode::Exit, &mut self.clone())
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for FunctionBlock {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_function_block(TreeTraversalMode::Enter, &mut self.clone());
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            // Visit each of the instructions
            self.instructions
                .iter()
                .map(|instruction| instruction.visit(emitter))
                .find(|r| *r == Err(String::from("Failure")))
                .unwrap_or(Ok(TraversalResult::Continue))
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                emitter.visit_function_block(TreeTraversalMode::Exit, &mut self.clone())
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl HighlevelIrPassExecutor for FunctionBody {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_function_body(TreeTraversalMode::Enter, &mut self.clone());
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.blocks
                .iter()
                .map(|block| block.visit(emitter))
                .find(|r| r.is_err())
                .unwrap_or(Ok(TraversalResult::Continue))
        } else {
            ret
        };
        match children_ret {
            Ok(TraversalResult::Continue) => {
                emitter.visit_function_body(TreeTraversalMode::Exit, &mut self.clone())
            }
            _ => children_ret,
        }
    }
}
impl HighlevelIrPassExecutor for ConcreteType {
    fn visit(&self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_concrete_type(TreeTraversalMode::Enter, &mut self.clone());
        let ret = if let Ok(TraversalResult::Continue) = ret {
            // visit children
            match self {
                ConcreteType::Tuple { data_layout, .. } => {
                    data_layout.fields.iter().try_for_each(|field| {
                        emitter.visit_symbol_name(TreeTraversalMode::Enter, field)
                    })
                }
                ConcreteType::Variant { data_layout, .. } => {
                    data_layout.fields.iter().try_for_each(|enum_val| {
                        emitter.visit_enum_value(TreeTraversalMode::Enter, enum_val)
                    })
                }
            }
        } else {
            ret
        }?;
        match ret {
            TraversalResult::Continue => {
                emitter.visit_concrete_type(TreeTraversalMode::Exit, &mut self.clone())
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl HighlevelIrPassExecutor for FunctionKind {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_function_kind(TreeTraversalMode::Enter, self);
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            // No explicit childs to visit in this implementation.
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.visit_function_kind(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for ConcreteFunction {
    fn visit(&mut self, emitter: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        let ret = emitter.visit_concrete_function(TreeTraversalMode::Enter, self)?;
        let children_ret = if ret == TraversalResult::Continue {
            // Visit the children here, example given for arguments vector
            for arg in &mut self.arguments {
                arg.visit(emitter)?;
            }
            // If you want to visit the body too, uncomment the following line
            // self.body.visit(emitter)?;
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
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
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            for type_def in &mut self.type_definitions {
                match type_def {
                    ConcreteType::Variant { name, data_layout } => {
                        name.visit(emitter)?;
                        data_layout.visit(emitter)?;
                    }
                    ConcreteType::Tuple { name, data_layout } => {
                        name.visit(emitter)?;
                        data_layout.visit(emitter)?;
                    }
                }
            }
            for function_def in &mut self.function_definitions {
                function_def.visit(emitter)?;
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => emitter.visit_highlevel_ir(TreeTraversalMode::Exit, self),
            _ => Ok(TraversalResult::Continue),
        }
    }
}
