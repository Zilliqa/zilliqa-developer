use primitive_types::U256;
use scilla_parser::ast::{TraversalResult, TreeTraversalMode};

use crate::intermediate_representation::{
    pass::IrPass,
    pass_executor::PassExecutor,
    primitives::{
        CaseClause, ConcreteFunction, ConcreteType, ContractField, EnumValue, FunctionBlock,
        FunctionBody, FunctionKind, Instruction, IntermediateRepresentation, IrIdentifier,
        IrIndentifierKind, Operation, Tuple, VariableDeclaration, Variant,
    },
    symbol_table::{StateLayoutEntry, SymbolTable},
};

pub struct StateCollector {
    namespace_stack: Vec<String>,
    current_namespace: Option<String>,
    // current_type: Option<String>,
    address_offset: u64,
}

impl StateCollector {
    pub fn new() -> Self {
        StateCollector {
            namespace_stack: Vec::new(),
            current_namespace: None,
            // current_type: None,
            address_offset: 4919, // TODO:
        }
    }

    fn push_namespace(&mut self, namespace: String) {
        self.namespace_stack.push(namespace.clone());
        self.current_namespace = Some(namespace);
    }

    fn pop_namespace(&mut self) {
        let _ = self.namespace_stack.pop();
        if let Some(namespace) = &self.namespace_stack.last() {
            self.current_namespace = Some(namespace.to_string());
        } else {
            self.current_namespace = None;
        }
    }
}

impl IrPass for StateCollector {
    fn initiate(&mut self) {}
    fn finalize(&mut self) {}

    fn visit_concrete_type(
        &mut self,
        _mode: TreeTraversalMode,
        _con_type: &mut ConcreteType,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_contract_field(
        &mut self,
        _mode: TreeTraversalMode,
        field: &mut ContractField,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let _ = field.namespace.visit(self, symbol_table)?;
        self.push_namespace(field.namespace.qualified_name()?);

        field.variable.visit(self, symbol_table)?;
        field.initializer.visit(self, symbol_table)?;

        let name = match &field.variable.name.resolved {
            Some(n) => n,
            None => {
                return Err("Failed to resolve name for contract field.".to_string());
            }
        };

        let address = U256::from(self.address_offset);
        let initializer = U256::from(0);
        self.address_offset += 1;

        let state = StateLayoutEntry {
            address_offset: address,
            size: 1, // TODO:
            initializer,
        };

        symbol_table.state_layout.insert(name.to_string(), state);
        // TODO: Register type of.

        self.pop_namespace();
        Ok(TraversalResult::Continue)
    }

    fn visit_enum_value(
        &mut self,
        _mode: TreeTraversalMode,
        _enum_value: &mut EnumValue,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_tuple(
        &mut self,
        _mode: TreeTraversalMode,
        _tuple: &mut Tuple,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_variant(
        &mut self,
        _mode: TreeTraversalMode,
        _variant: &mut Variant,
        _symbol_table: &mut SymbolTable,
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

    fn visit_concrete_function(
        &mut self,
        _mode: TreeTraversalMode,
        _fnc: &mut ConcreteFunction,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_symbol_kind(
        &mut self,
        _mode: TreeTraversalMode,
        _kind: &mut IrIndentifierKind,
        _symbol_table: &mut SymbolTable,
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

    fn visit_function_block(
        &mut self,
        _mode: TreeTraversalMode,
        _block: &mut FunctionBlock,
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
}
