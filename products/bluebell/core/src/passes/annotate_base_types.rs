use std::mem;

use scilla_parser::{
    ast::{TraversalResult, TreeTraversalMode},
    parser::lexer::SourcePosition,
};

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
        symbol_table::{SymbolTable, TypeInfo},
    },
};

pub struct AnnotateBaseTypes {
    previous_namespaces: Vec<String>,
    namespace: Option<String>,
    current_block: Option<FunctionBlock>,
}

impl AnnotateBaseTypes {
    pub fn new() -> Self {
        AnnotateBaseTypes {
            previous_namespaces: Vec::new(),
            namespace: None,
            current_block: None,
        }
    }

    // TODO: Make Symbol table member
    pub fn typename_of(
        &self,
        symbol: &IrIdentifier,
        symbol_table: &mut SymbolTable,
    ) -> Option<String> {
        if let Some(name) = &symbol.resolved {
            symbol_table.typename_of(name)
        } else {
            None
        }
    }

    // TODO: Make Symbol table member
    pub fn type_of(
        &self,
        symbol: &IrIdentifier,
        symbol_table: &mut SymbolTable,
    ) -> Option<Box<TypeInfo>> {
        if let Some(name) = &symbol.resolved {
            symbol_table.type_of(name, &self.namespace)
        } else {
            None
        }
    }

    pub fn push_namespace(&mut self, namespace: String) {
        let namespace = if let Some(ns) = &self.namespace {
            self.previous_namespaces.push(ns.clone());
            format!("{}{}{}", ns, NAMESPACE_SEPARATOR, namespace)
        } else {
            namespace
        };
        self.namespace = Some(namespace);
    }

    pub fn pop_namespace(&mut self) {
        self.namespace = self.previous_namespaces.pop();
    }
}

// TODO: Rename to AnnotateTypesDeclarations

impl IrPass for AnnotateBaseTypes {
    fn initiate(&mut self) {}
    fn finalize(&mut self) {}
    fn visit_concrete_type(
        &mut self,
        _mode: TreeTraversalMode,
        _con_type: &mut ConcreteType,
        _symbol_tablee: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
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
        var_dec: &mut VariableDeclaration,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        if let Some(typename) = &var_dec.typename.resolved {
            let _ = var_dec.name.visit(self, symbol_table)?;
            var_dec.name.type_reference = Some(typename.clone());

            if let Some(symbol) = &var_dec.name.resolved {
                // TODO: Check that symbol is unique

                symbol_table.declare_type_of(&symbol, typename)?;

                Ok(TraversalResult::SkipChildren)
            } else {
                println!("Error in {:#?}", var_dec);
                Err(format!(
                    "Could not resolve symbol for {}",
                    var_dec.name.unresolved,
                ))
            }
        } else {
            Err(format!(
                "Could not resolve type for {}, type {} is not declared",
                var_dec.name.unresolved, var_dec.typename.unresolved
            ))
        }
    }

    fn visit_contract_field(
        &mut self,
        _mode: TreeTraversalMode,
        field: &mut ContractField,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let namespace = match &field.namespace.resolved {
            Some(ns) => ns.clone(),
            None => {
                return Err(format!(
                    "Could not determine the namespace of {}",
                    field.namespace.unresolved
                ))
            }
        };

        self.push_namespace(namespace);

        field.variable.visit(self, symbol_table)?;
        field.initializer.visit(self, symbol_table)?;

        self.pop_namespace();

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_concrete_function(
        &mut self,
        _mode: TreeTraversalMode,
        fnc: &mut ConcreteFunction,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        let namespace = match &fnc.namespace.resolved {
            Some(ns) => ns.clone(),
            None => {
                return Err(format!(
                    "Could not determine the namespace of {}",
                    fnc.name.unresolved
                ))
            }
        };

        self.push_namespace(namespace);

        fnc.name.visit(self, symbol_table)?;

        self.push_namespace(fnc.name.unresolved.clone());

        for arg in fnc.arguments.iter_mut() {
            arg.visit(self, symbol_table)?;
        }

        fnc.body.visit(self, symbol_table)?;

        self.pop_namespace();
        self.pop_namespace();

        // TODO: collect type of function
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
        symbol: &mut IrIdentifier,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        match symbol.kind {
            IrIndentifierKind::Unknown => {
                if let Some(typeinfo) = self.type_of(symbol, symbol_table) {
                    symbol.type_reference = Some(typeinfo.typename.clone());

                    // We only move constructors out of line
                    if !typeinfo.is_constructor() {
                        return Ok(TraversalResult::Continue);
                    }

                    if typeinfo.arguments.len() > 0 {
                        return Err(format!(
                            "Cannot invoke constructor of {:?} with arguments",
                            symbol
                        ));
                    }

                    let return_type = if let Some(r) = typeinfo.return_type {
                        r
                    } else {
                        return Err(format!(
                            "Internal error: Return type not defined for contructor type {:?}",
                            typeinfo
                        ));
                    };

                    let mut intermediate_symbol = symbol_table.name_generator.new_intermediate();

                    symbol.kind = IrIndentifierKind::StaticFunctionName;
                    let mut constructor_call = Box::new(Instruction {
                        ssa_name: Some(intermediate_symbol.clone()),
                        result_type: Some(IrIdentifier {
                            unresolved: return_type.clone(),
                            resolved: Some(return_type.clone()),
                            type_reference: Some(return_type),
                            kind: IrIndentifierKind::TypeName,
                            is_definition: false,
                            source_location: (
                                // TODO:
                                SourcePosition::invalid_position(),
                                SourcePosition::invalid_position(),
                            ),
                        }),
                        operation: Operation::CallStaticFunction {
                            name: symbol.clone(),
                            owner: None, // TODO:
                            arguments: Vec::new(),
                        },
                        source_location: (
                            SourcePosition::invalid_position(),
                            SourcePosition::invalid_position(),
                        ),
                    });

                    constructor_call.visit(self, symbol_table)?;

                    if let Some(current_block) = &mut self.current_block {
                        current_block.instructions.push_back(constructor_call);
                    } else {
                        return Err(
                            "Internal error: No block available to push instruction ".to_string()
                        );
                    }

                    mem::swap(&mut intermediate_symbol, symbol);
                } else {
                    return Err(format!("Unable to resolve type of {:?}", symbol));
                }
            }
            _ => (),
        }

        // Changing

        if symbol.kind == IrIndentifierKind::VirtualRegister {
            if let Some(resolved_name) =
                symbol_table.resolve_qualified_name(&symbol.unresolved, &self.namespace)
            {
                if symbol_table.is_state(&resolved_name) {
                    symbol.kind = IrIndentifierKind::State;
                }
            } else {
                // TODO: panic!("Could not resolve qualified name");
            }
        }

        // Updating type
        match symbol.kind {
            IrIndentifierKind::BlockLabel => (),
            IrIndentifierKind::FunctionName
            | IrIndentifierKind::State
            | IrIndentifierKind::TransitionName
            | IrIndentifierKind::ProcedureName => {
                if !symbol.is_definition {
                    if let Some(resolved_name) =
                        symbol_table.resolve_qualified_name(&symbol.unresolved, &self.namespace)
                    {
                        symbol.resolved = Some(resolved_name);
                    }
                }
            }
            IrIndentifierKind::VirtualRegister
            | IrIndentifierKind::VirtualRegisterIntermediate
            | IrIndentifierKind::Memory => {
                if let Some(ns) = &self.namespace {
                    symbol.resolved = Some(
                        format!("{}{}{}", ns, NAMESPACE_SEPARATOR, symbol.unresolved).to_string(),
                    );
                }
            }
            _ => (),
        }
        symbol.type_reference = self.typename_of(symbol, symbol_table);
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
        instr: &mut Instruction,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        // TODO: These types should be stored somewhere (in the symbol table maybe?)
        let typename = match &mut instr.operation {
            Operation::TerminatingRef(_identifier) => {
                "Void".to_string() // TODO: Fetch from somewhere
            }
            Operation::Noop => "Void".to_string(), // TODO: Fetch from somewhere
            Operation::Jump(_) => "Void".to_string(), // TODO: Fetch from somewhere
            Operation::ConditionalJump {
                expression,
                on_success,
                on_failure,
            } => {
                expression.visit(self, symbol_table)?;
                on_success.visit(self, symbol_table)?;
                on_failure.visit(self, symbol_table)?;
                "Void".to_string() // TODO: Fetch from somewhere
            }
            Operation::StateLoad { address } => {
                address.name.visit(self, symbol_table)?;
                let value = match &mut instr.ssa_name {
                    Some(v) => v,
                    None => panic!("Load does not assign value"),
                };
                value.visit(self, symbol_table)?;
                let symbol_name = match &value.resolved {
                    Some(r) => r.clone(),
                    None => {
                        return Err(format!(
                            "Unable resolve symbol name for for load statement {}",
                            value.unresolved
                        ))
                    }
                };

                value.type_reference = address.name.type_reference.clone();

                match &value.type_reference {
                    Some(typename) => {
                        symbol_table.declare_type_of(&symbol_name, &typename)?;
                        typename.clone()
                    }
                    None => {
                        return Err(format!(
                            "Unable to deduce type for load statement {}",
                            symbol_name
                        ))
                    }
                }
            }
            Operation::StateStore { address, value } => {
                address.name.visit(self, symbol_table)?;
                value.visit(self, symbol_table)?;

                let symbol_name = match &address.name.resolved {
                    Some(r) => r.clone(),
                    None => {
                        return Err(format!(
                            "Unable resolve symbol name for for store statement {}",
                            value.unresolved
                        ))
                    }
                };

                value.type_reference = address.name.type_reference.clone();

                match &value.type_reference {
                    Some(typename) => {
                        symbol_table.declare_type_of(&symbol_name, &typename)?;
                        typename.clone()
                    }
                    None => {
                        return Err(format!(
                            "Unable to deduce type for store statement {}",
                            symbol_name
                        ))
                    }
                }
            }
            Operation::MemLoad => "TODO".to_string(),
            Operation::MemStore => "TODO".to_string(),
            Operation::IsEqual { left, right } => {
                left.visit(self, symbol_table)?;
                right.visit(self, symbol_table)?;
                //  panic!("Failed");
                // TODO: Should return the same type as left and right
                "Uint256".to_string()
            }
            Operation::CallFunction {
                ref mut name,
                arguments,
            } // We do not distinguish between CallFunction and CallExternalFunction
            | Operation::CallExternalFunction {
                ref mut name,
                arguments,
            } => {
                name.visit(self, symbol_table)?;
                let mut argument_type_args: String = "".to_string();
                for (i, arg) in arguments.iter_mut().enumerate() {
                    arg.visit(self, symbol_table)?;
                    let type_name = match &arg.type_reference {
                        Some(t) => t,
                        None => {
                            // TODO: Fix error propagation
                            panic!(
                                "Unable to resolve type for {:?} in {:?}",
                                arg.unresolved, name
                            );
                        }
                    };
                    if i > 0 {
                        argument_type_args.push_str(",");
                    }
                    argument_type_args.push_str(type_name);
                }

                // In the event of a template function, we use the unresolved name
                // as the function may not yet exist
                let name_value = match &name.resolved {
                    Some(v) => v,
                    None => &name.unresolved
                };

                let function_type = format!("{}::<{}>", name_value, argument_type_args).to_string();

                let function_type = if let Some(typeinfo)= symbol_table.type_of(&function_type, &self.namespace) {
                    typeinfo.symbol_name
                } else {
                    panic!("Unable to find symbol {}", function_type);
                };

                name.resolved = Some(function_type.clone());


                // The value of the SSA is the return type of the function
                // TODO: To this end we need to resolve the type refernce from the resolved name
                name.type_reference = Some(function_type.clone()); // TODO: Should contain return type as this is a function pointer

                let type_info = match symbol_table.type_of(&function_type, &self.namespace) {
                    Some(v) => {
                        match v.return_type {
                            Some(r) => r,
                            None => "Void".to_string() // TODO: Get value from somewhere
                        }
                    }
                    None => {
                        println!("{:#?}", symbol_table);
                        panic!("Undeclared function {}", function_type)
                    }
                };

                type_info
            }
            Operation::CallStaticFunction {
                name,
                owner: _,
                arguments,
            } => {
                name.visit(self, symbol_table)?;
                for arg in arguments.iter_mut() {
                    arg.visit(self, symbol_table)?;
                }

                let return_type = if let Some(function_type) = &name.type_reference {
                    let function_typeinfo = symbol_table.type_of(function_type, &self.namespace);

                    if let Some(function_typeinfo) = function_typeinfo {
                        function_typeinfo.return_type.expect("").clone()
                    } else {
                        return Err(format!("Unable to determine type of {:?}", name.unresolved)
                            .to_string());
                    }
                } else {
                    return Err(format!(
                        "Unable to determine return type of {:?}",
                        name.unresolved
                    )
                    .to_string());
                };

                return_type
            }
            Operation::CallMemberFunction {
                name,
                owner: _,
                arguments,
            } => {
                name.visit(self, symbol_table)?;
                for arg in arguments.iter_mut() {
                    arg.visit(self, symbol_table)?;
                }

                unimplemented!()
            }
            Operation::ResolveSymbol { symbol } => {
                symbol.visit(self, symbol_table)?;
                match &symbol.type_reference {
                    Some(t) => t.clone(),
                    None => {
                        return Err(format!(
                            "Unable to determine type for {}",
                            symbol.unresolved
                        ));
                    }
                }
            }
            Operation::ResolveContextResource { symbol } => {
                symbol.visit(self, symbol_table)?;
                match &symbol.type_reference {
                    Some(t) => t.clone(),
                    None => {
                        return Err(format!(
                            "Unable to determine type for {}",
                            symbol.unresolved
                        ));
                    }
                }
            }
            Operation::Literal { data, typename } => {
                typename.visit(self, symbol_table)?;

                match &typename.type_reference {
                    Some(t) => t.clone(),
                    None => {
                        return Err(format!(
                            "Unable to determine type for literal {} {}",
                            typename.unresolved, data
                        ));
                    }
                }
            }
            Operation::PhiNode(inputs) => {
                let mut type_name = None;
                for input in inputs.iter_mut() {
                    input.visit(self, symbol_table)?;
                    if input.type_reference != type_name {
                        if type_name == None {
                            type_name = input.type_reference.clone();
                        } else {
                            return Err("Different paths given different return types.".to_string());
                        }
                    }
                }

                if let Some(type_name) = type_name {
                    type_name
                } else {
                    "Void".to_string() // TODO: specify somewhere // TODO: Fetch from somewhere
                }
            }
            Operation::Revert(n) | Operation::Return(n) => {
                match n {
                    Some(_) => todo!(),
                    None => "Void".to_string(), // TODO: specify somewhere // TODO: Fetch from somewhere
                }
            }
        };

        //instr.operation.type_reference = Some(typename);

        if let Some(ssa) = &mut instr.ssa_name {
            ssa.visit(self, symbol_table)?;
            if let Some(symbol_name) = &ssa.resolved {
                // TODO: Check whether symbol exists

                symbol_table.declare_type_of(symbol_name, &typename)?;

                ssa.type_reference = Some(typename);
            } else {
                return Err(format!(
                    "Unable to resolved symbol name for {}",
                    ssa.unresolved
                ));
            }
        }

        Ok(TraversalResult::SkipChildren)
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
        self.current_block = Some(*FunctionBlock::new_from_symbol(block.name.clone()));

        if let Some(ref mut new_block) = &mut self.current_block {
            new_block.terminated = block.terminated;
        }

        for instr in block.instructions.iter_mut() {
            instr.visit(self, symbol_table)?;
            if let Some(ref mut new_block) = &mut self.current_block {
                new_block.instructions.push_back(instr.clone());
            }
        }

        if let Some(ref mut new_block) = &mut self.current_block {
            mem::swap(block, new_block);
            self.current_block = None;
        } else {
            return Err(
                "Internal error: Block was undefined when returning from instruction passing"
                    .to_string(),
            );
        }
        Ok(TraversalResult::SkipChildren)
    }
}
