use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    mem,
};

use scilla_parser::{
    ast::{TraversalResult, TreeTraversalMode},
    parser::lexer::SourcePosition,
};

use crate::intermediate_representation::{
    pass::IrPass,
    primitives::{
        CaseClause, ConcreteFunction, ConcreteType, ContractField, EnumValue, FunctionBlock,
        FunctionBody, FunctionKind, Instruction, IntermediateRepresentation, IrIdentifier,
        IrIndentifierKind, Operation, Tuple, VariableDeclaration, Variant,
    },
    symbol_table::SymbolTable,
};

pub struct BalanceBlockArguments {
    blocks: HashMap<String, FunctionBlock>,
}

impl BalanceBlockArguments {
    pub fn new() -> Self {
        BalanceBlockArguments {
            blocks: HashMap::new(),
        }
    }
}

// TODO: Rename to AnnotateTypesDeclarations

impl IrPass for BalanceBlockArguments {
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
        _mode: TreeTraversalMode,
        fnc: &mut ConcreteFunction,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let mut name_stack: VecDeque<String> = VecDeque::new();
        // Creating a copy of all blocks
        for block in fnc.body.blocks.iter() {
            let name = match &block.name.resolved {
                Some(n) => n,
                None => panic!("Failed to resolve name of block"),
            };

            if block.enters_from.is_empty() {
                name_stack.push_back(name.clone());
            }
            self.blocks.insert(name.to_string(), *block.clone());
        }

        while !name_stack.is_empty() {
            let next_name = name_stack.pop_front().unwrap();

            let block = match self.blocks.get(&next_name) {
                Some(b) => b.clone(),
                None => panic!("Unable to find block with name {}", next_name),
            };

            for name in &block.exits_to {
                name_stack.push_back(name.to_string());
            }

            // Combuting the common (combined) args
            let mut args_required: BTreeSet<String> = BTreeSet::new();
            for (_, set) in &block.jump_required_arguments {
                for arg in set.iter() {
                    args_required.insert(arg.to_string());
                }
            }

            let empty_set = BTreeSet::new();
            for name in &block.exits_to {
                let set = block
                    .jump_required_arguments
                    .get(name)
                    .unwrap_or(&empty_set);
                if !set.eq(&args_required) {
                    if let Some(block) = self.blocks.get_mut(name) {
                        for arg in &args_required {
                            if !set.contains(arg) {
                                let ir_identifier = IrIdentifier {
                                    unresolved: arg.clone(),
                                    resolved: Some(arg.clone()),
                                    type_reference: None,
                                    kind: IrIndentifierKind::VirtualRegister,
                                    is_definition: false,
                                    source_location: (
                                        // TODO:
                                        SourcePosition::invalid_position(),
                                        SourcePosition::invalid_position(),
                                    ),
                                };

                                let op = Operation::TerminatingRef(ir_identifier);
                                block.instructions.push_front(Box::new(Instruction {
                                    ssa_name: None,
                                    result_type: None,
                                    operation: op,
                                    source_location: (
                                        SourcePosition::invalid_position(),
                                        SourcePosition::invalid_position(),
                                    ),
                                }));
                            }
                        }
                    }
                }
            }
        }

        // Updating blocks
        for block in fnc.body.blocks.iter_mut() {
            let name = match &block.name.resolved {
                Some(n) => n,
                None => panic!("Failed to resolve name of block"),
            };

            if let Some(ref mut new_block) = self.blocks.get_mut(name) {
                //let mut new_block = Box::new(new_block);
                mem::swap(&mut block.instructions, &mut new_block.instructions);
            }
        }
        Ok(TraversalResult::SkipChildren)
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
        _symbol: &mut IrIdentifier,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
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
        _mode: TreeTraversalMode,
        _function_body: &mut FunctionBody,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
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
        _mode: TreeTraversalMode,
        _operation: &mut Operation,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
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
        _block: &mut FunctionBlock,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
}
