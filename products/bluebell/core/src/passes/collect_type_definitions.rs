use scilla_parser::ast::{TraversalResult, TreeTraversalMode};

use crate::{
    constants::NAMESPACE_SEPARATOR,
    intermediate_representation::{
        pass::IrPass,
        pass_executor::PassExecutor,
        primitives::{
            CaseClause, ConcreteFunction, ConcreteType, ContractField, EnumValue, FunctionBlock,
            FunctionBody, FunctionKind, Instruction, IntermediateRepresentation, IrIdentifier,
            IrIndentifierKind, Operation, Tuple, VariableDeclaration, Variant,
        },
        symbol_table::SymbolTable,
    },
};

pub struct CollectTypeDefinitionsPass {
    namespace_stack: Vec<String>,
    current_namespace: Option<String>,
    current_type: Option<String>,
}

impl CollectTypeDefinitionsPass {
    pub fn new() -> Self {
        CollectTypeDefinitionsPass {
            namespace_stack: Vec::new(),
            current_namespace: None,
            current_type: None,
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

impl IrPass for CollectTypeDefinitionsPass {
    fn initiate(&mut self) {}
    fn finalize(&mut self) {}

    fn visit_concrete_type(
        &mut self,
        _mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match con_type {
            ConcreteType::Tuple {
                name,
                namespace,
                data_layout,
            } => {
                let _ = namespace.visit(self, symbol_table)?;
                self.push_namespace(namespace.qualified_name()?);

                let _ = name.visit(self, symbol_table)?;
                let qualified_name = name.qualified_name()?;

                symbol_table.declare_type(&qualified_name)?;

                // Backgwards compatibility support
                // TODO: Enable and disable this with flag

                self.current_type = Some(qualified_name);
                let _ = data_layout.visit(self, symbol_table)?;

                self.current_type = None;

                self.pop_namespace();
            }
            ConcreteType::Variant {
                name,
                namespace,
                data_layout,
            } => {
                let _ = namespace.visit(self, symbol_table)?;
                self.push_namespace(namespace.qualified_name()?);

                let _ = name.visit(self, symbol_table)?;
                let qualified_name = name.qualified_name()?;

                symbol_table.declare_type(&qualified_name)?;

                // Backgwards compatibility support
                // TODO: Enable and disable this with flag
                // TODO: Check error
                let _ = symbol_table.declare_alias(&name.unresolved, &qualified_name);

                self.current_type = Some(qualified_name);
                let _ = data_layout.visit(self, symbol_table)?;

                self.current_type = None;
                self.pop_namespace();
            }
        }
        Ok(TraversalResult::SkipChildren)
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

        self.pop_namespace();
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_enum_value(
        &mut self,
        _mode: TreeTraversalMode,
        enum_value: &mut EnumValue,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        if let Some(return_type) = self.current_type.clone() {
            self.push_namespace(return_type.clone().to_string());
            let _ = enum_value.name.visit(self, symbol_table)?;

            let resolved_name = if let Some(resolved_name) = &enum_value.name.resolved {
                resolved_name.to_string()
            } else {
                return Err(format!(
                    "Could not resolve symbol for {}",
                    enum_value.name.unresolved
                ));
            };

            // Creating alias for legacy reasons
            symbol_table
                .aliases
                .insert(enum_value.name.unresolved.clone(), resolved_name.clone());

            self.pop_namespace();

            // TODO: Work out whehter we should attempt to resolve the type right away?
            let mut arguments: Vec<String> = Vec::new();
            if let Some(data) = &mut enum_value.data {
                let _ = data.visit(self, symbol_table)?;
                if let Some(resolved_type) = &data.resolved {
                    arguments.push(resolved_type.to_string());
                }
            }

            symbol_table.declare_constructor(&resolved_name, &arguments, &return_type)?;

            // TODO: Set the constructor function signature and alias

            Ok(TraversalResult::SkipChildren)
        } else {
            Err(
                "Internal error: Unable to determine the type which is currently being defined."
                    .to_string(),
            )
        }
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
        fnc: &mut ConcreteFunction,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let _ = fnc.namespace.visit(self, symbol_table)?;
        self.push_namespace(fnc.namespace.qualified_name()?);

        let _ = fnc.name.visit(self, symbol_table)?;
        let qualified_name = fnc.name.qualified_name()?;

        let mut args_types: Vec<String> = Vec::new();
        for arg in fnc.arguments.iter_mut() {
            arg.visit(self, symbol_table)?;
            args_types.push(arg.typename.qualified_name()?);
        }

        // TODO: Get return type from body if not set in the definition
        fnc.body.visit(self, symbol_table)?;

        // Declaring
        let qualified_name = format!("{}::<{}>", qualified_name, args_types.join(","));
        let return_type = "TODO";
        symbol_table.declare_function_type(&qualified_name, &args_types, return_type)?;

        self.current_type = None;
        self.pop_namespace();

        Ok(TraversalResult::SkipChildren)
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
        symbol: &mut IrIdentifier,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match symbol.kind {
            IrIndentifierKind::BlockLabel | IrIndentifierKind::Namespace => {
                symbol.resolved = Some(symbol.unresolved.clone());
            }
            _ => {
                if symbol.is_definition {
                    if let Some(namespace) = &self.current_namespace {
                        let typename =
                            format!("{}{}{}", namespace, NAMESPACE_SEPARATOR, symbol.unresolved)
                                .to_string();
                        symbol.resolved = Some(typename.clone());
                    } else {
                        symbol.resolved = Some(symbol.unresolved.clone());
                    }
                } else if let Some(resolved_name) =
                    symbol_table.resolve_qualified_name(&symbol.unresolved, &self.current_namespace)
                {
                    // TODO: Consider whether this is needed.
                    // It appears that currently this is only triggered
                    // by builtin type defintions which really ought to have
                    // is_definition = true
                    symbol.resolved = Some(resolved_name);
                }
            }
        }

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_primitives(
        &mut self,
        mode: TreeTraversalMode,
        _primitives: &mut IntermediateRepresentation,
        _symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => (),
            TreeTraversalMode::Exit => {
                panic!("Not handled.");
            }
        }
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
