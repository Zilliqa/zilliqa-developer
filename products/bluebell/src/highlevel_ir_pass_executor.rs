use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::*;
use crate::highlevel_ir_pass::HighlevelIrPass;
use crate::symbol_table::SymbolTable;

pub trait HighlevelIrPassExecutor {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String>;
}

impl HighlevelIrPassExecutor for IrIndentifierKind {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_symbol_kind(TreeTraversalMode::Enter, self, symbol_table)?;

        match ret {
            TraversalResult::Continue => {
                pass.visit_symbol_kind(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for IrIdentifier {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_symbol_name(TreeTraversalMode::Enter, self, symbol_table);

        // TODO: visit children, if 'ret' is TraversalResult::Continuen
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            self.kind.visit(pass, symbol_table)?;
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;

        match children_ret {
            TraversalResult::Continue => {
                pass.visit_symbol_name(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for EnumValue {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_enum_value(TreeTraversalMode::Enter, self, symbol_table);
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            let _ = self.name.visit(pass, symbol_table)?;
            if let Some(data) = &mut self.data {
                data.visit(pass, symbol_table)
            } else {
                ret
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                pass.visit_enum_value(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Tuple {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_tuple(TreeTraversalMode::Enter, self, symbol_table);
        // visit children, if 'ret' is TraversalResult::Continue
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for field in self.fields.iter_mut() {
                field.visit(pass, symbol_table)?;
            }

            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                pass.visit_tuple(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Variant {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_variant(TreeTraversalMode::Enter, self, symbol_table);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for field in &mut self.fields {
                let result = field.visit(pass, symbol_table);
                if result != Ok(TraversalResult::Continue) {
                    return result;
                }
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        if let Ok(TraversalResult::Continue) = children_ret {
            pass.visit_variant(TreeTraversalMode::Exit, self, symbol_table)
        } else {
            Ok(TraversalResult::SkipChildren)
        }
    }
}

impl HighlevelIrPassExecutor for VariableDeclaration {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_variable_declaration(TreeTraversalMode::Enter, self, symbol_table);
        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            self.typename.visit(pass, symbol_table)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                pass.visit_variable_declaration(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Operation {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_operation(TreeTraversalMode::Enter, self, symbol_table);

        let children_ret = if let Ok(TraversalResult::Continue) = ret {
            match self {
                Operation::Jump(identifier) => identifier.visit(pass, symbol_table),
                Operation::MemLoad
                | Operation::MemStore
                | Operation::AcceptTransfer
                | Operation::PhiNode(_) => Ok(TraversalResult::Continue),
                Operation::ConditionalJump {
                    expression,
                    on_success,
                    on_failure,
                } => {
                    expression.visit(pass, symbol_table)?;
                    on_success.visit(pass, symbol_table)?;
                    on_failure.visit(pass, symbol_table)
                }
                Operation::IsEqual { left, right } => {
                    left.visit(pass, symbol_table)?;
                    right.visit(pass, symbol_table)?;
                    Ok(TraversalResult::Continue)
                }
                Operation::CallExternalFunction { name, arguments }
                | Operation::CallFunction { name, arguments } => {
                    name.visit(pass, symbol_table)?;
                    for arg in arguments {
                        arg.visit(pass, symbol_table)?;
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
                        owner.visit(pass, symbol_table)?;
                    }
                    name.visit(pass, symbol_table)?;
                    for arg in arguments {
                        arg.visit(pass, symbol_table)?;
                    }
                    Ok(TraversalResult::Continue)
                }
                Operation::ResolveSymbol { symbol } => {
                    symbol.visit(pass, symbol_table)?;
                    Ok(TraversalResult::Continue)
                }
                Operation::Literal { data: _, typename } => {
                    typename.visit(pass, symbol_table)?;
                    Ok(TraversalResult::Continue)
                }
                Operation::Noop => Ok(TraversalResult::Continue),
            }
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                pass.visit_operation(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for Instruction {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_instruction(TreeTraversalMode::Enter, self, symbol_table);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            if let Some(ssa) = &mut self.ssa_name {
                ssa.visit(pass, symbol_table)?;
            }
            if let Some(ret) = &mut self.result_type {
                ret.visit(pass, symbol_table)?;
            }

            self.operation.visit(pass, symbol_table)?;
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                pass.visit_instruction(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for FunctionBlock {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_function_block(TreeTraversalMode::Enter, self, symbol_table);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            self.name.visit(pass, symbol_table)?;
            for instr in self.instructions.iter_mut() {
                instr.visit(pass, symbol_table)?;
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        }?;
        match children_ret {
            TraversalResult::Continue => {
                pass.visit_function_block(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}
impl HighlevelIrPassExecutor for FunctionBody {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_function_body(TreeTraversalMode::Enter, self, symbol_table);
        let children_ret = if ret == Ok(TraversalResult::Continue) {
            for block in self.blocks.iter_mut() {
                block.visit(pass, symbol_table)?;
            }
            Ok(TraversalResult::Continue)
        } else {
            ret
        };
        match children_ret {
            Ok(TraversalResult::Continue) => {
                pass.visit_function_body(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => children_ret,
        }
    }
}

impl HighlevelIrPassExecutor for ConcreteType {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_concrete_type(TreeTraversalMode::Enter, self, symbol_table)?;
        let ret = if let TraversalResult::Continue = ret {
            // visit children
            match self {
                ConcreteType::Tuple {
                    name,
                    namespace,
                    data_layout,
                } => {
                    name.visit(pass, symbol_table)?;
                    namespace.visit(pass, symbol_table)?;
                    data_layout.visit(pass, symbol_table)?;
                }
                ConcreteType::Variant {
                    name,
                    namespace,
                    data_layout,
                } => {
                    name.visit(pass, symbol_table)?;
                    namespace.visit(pass, symbol_table)?;

                    data_layout.visit(pass, symbol_table)?;
                }
            }
            TraversalResult::Continue
        } else {
            ret
        };
        match ret {
            TraversalResult::Continue => {
                pass.visit_concrete_type(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for FunctionKind {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_function_kind(TreeTraversalMode::Enter, self, symbol_table)?;

        // No children

        match ret {
            TraversalResult::Continue => {
                pass.visit_function_kind(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}

impl HighlevelIrPassExecutor for ConcreteFunction {
    fn visit(
        &mut self,
        pass: &mut dyn HighlevelIrPass,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let ret = pass.visit_concrete_function(TreeTraversalMode::Enter, self, symbol_table)?;
        let children_ret = if ret == TraversalResult::Continue {
            let _ = self.name.visit(pass, symbol_table)?;
            let _ = self.namespace.visit(pass, symbol_table)?;
            let _ = self.function_kind.visit(pass, symbol_table)?;
            if let Some(_rt) = &mut self.return_type {
                // TODO: Change when rt is an IrIdentifier let _ = rt.visit(pass, symbol_table)?;
            }
            for (_i, arg) in self.arguments.iter_mut().enumerate() {
                let _ = arg.visit(pass, symbol_table)?;
            }

            let _ = self.body.visit(pass, symbol_table)?;

            TraversalResult::Continue
        } else {
            ret
        };
        match children_ret {
            TraversalResult::Continue => {
                pass.visit_concrete_function(TreeTraversalMode::Exit, self, symbol_table)
            }
            _ => Ok(TraversalResult::Continue),
        }
    }
}


impl HighlevelIr {
    pub fn run_pass(&mut self,pass: &mut dyn HighlevelIrPass) -> Result<TraversalResult, String> {
        for type_def in &mut self.type_definitions {
            type_def.visit(pass, &mut self.symbol_table)?;
        }

        for function_def in &mut self.function_definitions {
            function_def.visit(pass,  &mut self.symbol_table)?;
        }

        Ok(TraversalResult::Continue)
    }

}
