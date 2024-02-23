use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    mem,
};

use scilla_parser::ast::{TraversalResult, TreeTraversalMode};

use crate::intermediate_representation::{
    pass::IrPass,
    pass_executor::PassExecutor,
    primitives::{
        CaseClause, ConcreteFunction, ConcreteType, ContractField, EnumValue, FunctionBlock,
        FunctionBody, FunctionKind, Instruction, IntermediateRepresentation, IrIdentifier,
        IrIndentifierKind, Operation, Tuple, VariableDeclaration, Variant,
    },
    symbol_table::SymbolTable,
};

pub struct DeduceBlockDependencies {
    blocks: HashMap<String, FunctionBlock>,

    defined_names: BTreeSet<String>,
    used_names: BTreeSet<String>,
    listed_jump_to: BTreeSet<String>,
}

impl DeduceBlockDependencies {
    pub fn new() -> Self {
        DeduceBlockDependencies {
            blocks: HashMap::new(),
            defined_names: BTreeSet::new(),
            used_names: BTreeSet::new(),
            listed_jump_to: BTreeSet::new(),
        }
    }
}

// TODO: Rename to AnnotateTypesDeclarations

impl IrPass for DeduceBlockDependencies {
    fn initiate(&mut self) {}
    fn finalize(&mut self) {}

    fn visit_concrete_type(
        &mut self,
        _mode: TreeTraversalMode,
        _con_type: &mut ConcreteType,
        _symbol_tablee: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_enum_value(
        &mut self,
        _mode: TreeTraversalMode,
        _enum_value: &mut EnumValue,
        _symbol_tablee: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_tuple(
        &mut self,
        _mode: TreeTraversalMode,
        _tuple: &mut Tuple,
        _symbol_tablee: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_variant(
        &mut self,
        _mode: TreeTraversalMode,
        _variant: &mut Variant,
        _symbol_tablee: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        // Pass through deliberate
        Ok(TraversalResult::Continue)
    }

    fn visit_variable_declaration(
        &mut self,
        _mode: TreeTraversalMode,
        _var_dec: &mut VariableDeclaration,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_contract_field(
        &mut self,
        _mode: TreeTraversalMode,
        _field: &mut ContractField,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_concrete_function(
        &mut self,
        mode: TreeTraversalMode,
        fnc: &mut ConcreteFunction,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => {
                self.blocks = HashMap::new();
            }
            TreeTraversalMode::Exit => {
                let mut block_names: Vec<String> = Vec::new();

                // Updating enters_from
                for block in fnc.body.blocks.iter_mut() {
                    let name = match &block.name.resolved {
                        Some(n) => n,
                        None => panic!("Failed to resolve name of block"),
                    };
                    block_names.push(name.clone());
                    for link_to in &block.exits_to {
                        if let Some(target) = self.blocks.get_mut(link_to) {
                            target.enters_from.insert(name.to_string());
                        } else {
                            panic!("Jump to non-existing block {} in {}", link_to, name);
                        }
                    }
                }

                // Tracking variables
                for name in &block_names {
                    let block = match self.blocks.get_mut(name) {
                        Some(b) => b.clone(),
                        _ => {
                            panic!("Unregistered block {}", name);
                        }
                    };

                    // Tracing arguments back to their origin
                    for variable in &block.block_arguments {
                        let mut used: BTreeSet<String> = BTreeSet::new();
                        let mut edge_queue: VecDeque<(String, String)> = VecDeque::new();
                        for from in &block.enters_from {
                            edge_queue.push_back((from.to_string(), name.to_string()));
                        }

                        while !edge_queue.is_empty() {
                            let (from, to) = edge_queue.pop_front().unwrap();
                            if used.contains(&from) {
                                continue;
                            }

                            used.insert(from.clone());

                            let required_block_args: BTreeSet<String> = match self.blocks.get(&to) {
                                Some(to) => to.block_arguments.clone().into_iter().collect(),
                                None => panic!("Unregistered block {}", to),
                            };

                            match self.blocks.get_mut(&from) {
                                Some(from_block) => {
                                    from_block
                                        .jump_required_arguments
                                        .insert(to, required_block_args);
                                    if !from_block.defined_ssas.contains(variable) {
                                        // If the variable is not contained in the arguments already,
                                        // we add it and continue to traverse backwards
                                        if !from_block.block_arguments.contains(variable) {
                                            from_block.block_arguments.insert(variable.to_string());

                                            for name in &block.enters_from {
                                                edge_queue.push_back((
                                                    name.to_string(),
                                                    from.to_string(),
                                                ));
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    panic!("Unregistered block {}", from);
                                }
                            };
                        }
                    }
                }

                // Updating the function blocks
                for block in fnc.body.blocks.iter_mut() {
                    let name = match &block.name.resolved {
                        Some(n) => n,
                        None => panic!("Failed to resolve name of block"),
                    };
                    if let Some(updated) = self.blocks.get_mut(name) {
                        mem::swap(&mut block.block_arguments, &mut updated.block_arguments);
                        mem::swap(&mut block.enters_from, &mut updated.enters_from);
                        mem::swap(&mut block.exits_to, &mut updated.exits_to);
                        mem::swap(&mut block.defined_ssas, &mut updated.defined_ssas);
                        mem::swap(
                            &mut block.jump_required_arguments,
                            &mut updated.jump_required_arguments,
                        );
                    } else {
                        panic!("Unregistered block {}", name);
                    }
                }
            }
        }
        Ok(TraversalResult::Continue)
    }

    fn visit_symbol_kind(
        &mut self,
        _mode: TreeTraversalMode,
        _kind: &mut IrIndentifierKind,
        _symbol_tablee: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_symbol_name(
        &mut self,
        _mode: TreeTraversalMode,
        symbol: &mut IrIdentifier,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match symbol.kind {
            IrIndentifierKind::VirtualRegister | IrIndentifierKind::VirtualRegisterIntermediate => {
                match &symbol.resolved {
                    Some(n) => self.used_names.insert(n.clone()),
                    None => panic!("Unresolved symbol name encountered"),
                };
            }
            _ => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn visit_primitives(
        &mut self,
        _mode: TreeTraversalMode,
        _primitives: &mut IntermediateRepresentation,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_function_body(
        &mut self,
        mode: TreeTraversalMode,
        _function_body: &mut FunctionBody,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => (),
            _ => (),
        };
        Ok(TraversalResult::Continue)
    }

    fn visit_function_kind(
        &mut self,
        _mode: TreeTraversalMode,
        _function_kind: &mut FunctionKind,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_operation(
        &mut self,
        mode: TreeTraversalMode,
        operation: &mut Operation,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => match &operation {
                Operation::Jump(label) => {
                    match &label.resolved {
                        Some(l) => self.listed_jump_to.insert(l.clone()),
                        None => panic!("Unresolved block label encountered"),
                    };
                    return Ok(TraversalResult::SkipChildren);
                }
                Operation::ConditionalJump {
                    expression: _,
                    on_success,
                    on_failure,
                } => {
                    match &on_success.resolved {
                        Some(l) => self.listed_jump_to.insert(l.clone()),
                        None => panic!("Unresolved block label encountered"),
                    };

                    match &on_failure.resolved {
                        Some(l) => self.listed_jump_to.insert(l.clone()),
                        None => panic!("Unresolved block label encountered"),
                    };
                    return Ok(TraversalResult::SkipChildren);
                }
                _ => (),
            },
            _ => (),
        }
        Ok(TraversalResult::Continue)
    }

    fn visit_instruction(
        &mut self,
        _mode: TreeTraversalMode,
        _instr: &mut Instruction,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_case_clause(
        &mut self,
        _mode: TreeTraversalMode,
        _con_function: &mut CaseClause,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_function_block(
        &mut self,
        _mode: TreeTraversalMode,
        block: &mut FunctionBlock,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        self.listed_jump_to = BTreeSet::new();
        self.defined_names = BTreeSet::new();
        self.used_names = BTreeSet::new();

        let name = match &block.name.resolved {
            Some(n) => n.clone(),
            None => {
                return Err("Function block does not have a resolved name".to_string());
            }
        };

        for instr in block.instructions.iter_mut() {
            match &instr.ssa_name {
                Some(id) => {
                    if id.kind == IrIndentifierKind::VirtualRegister
                        || id.kind == IrIndentifierKind::VirtualRegisterIntermediate
                    {
                        match &id.resolved {
                            Some(name) => {
                                // We only define a variable if it was not used before. If it was used before
                                // it should be registered as a block argument.
                                if !self.used_names.contains(name) {
                                    self.defined_names.insert(name.clone());
                                }
                            }
                            None => panic!("Encountered unresolved SSA name"),
                        };
                    }
                }
                None => (),
            };
            instr.visit(self, symbol_table)?;
        }

        block.block_arguments = self
            .used_names
            .difference(&self.defined_names)
            .cloned()
            .collect();
        mem::swap(&mut block.defined_ssas, &mut self.defined_names);
        mem::swap(&mut block.exits_to, &mut self.listed_jump_to);

        self.blocks.insert(name, block.clone());

        Ok(TraversalResult::SkipChildren)
    }
}
