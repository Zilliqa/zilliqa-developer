use crate::constants::NAMESPACE_SEPARATOR;
use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::Instruction;
use crate::highlevel_ir::{
    ConcreteFunction, ConcreteType, EnumValue, FunctionBlock, FunctionBody, FunctionKind,
    HighlevelIr, IrIdentifier, IrIndentifierKind, Operation, Tuple, VariableDeclaration, Variant,
};
use crate::highlevel_ir_pass::HighlevelIrPass;
use crate::highlevel_ir_pass_executor::HighlevelIrPassExecutor;
use crate::symbol_table::SymbolTable;

pub struct AnnotateBaseTypes<'symtab> {
    symbol_table: &'symtab mut SymbolTable,
    previous_namespaces: Vec<String>,
    namespace: Option<String>,
}

impl<'symtab> AnnotateBaseTypes<'symtab> {
    pub fn new(symbol_table: &'symtab mut SymbolTable) -> Self {
        AnnotateBaseTypes {
            symbol_table,
            previous_namespaces: Vec::new(),
            namespace: None,
        }
    }

    pub fn type_of(&self, symbol: &IrIdentifier) -> Option<String> {
        if let Some(name) = &symbol.resolved {
            self.symbol_table.type_of.get(name).cloned()
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

impl<'symtab> HighlevelIrPass for AnnotateBaseTypes<'symtab> {
    fn visit_concrete_type(
        &mut self,
        _mode: TreeTraversalMode,
        _con_type: &mut ConcreteType,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_enum_value(
        &mut self,
        _mode: TreeTraversalMode,
        _enum_value: &mut EnumValue,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_tuple(
        &mut self,
        _mode: TreeTraversalMode,
        _tuple: &mut Tuple,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_variant(
        &mut self,
        _mode: TreeTraversalMode,
        _variant: &mut Variant,
    ) -> Result<TraversalResult, String> {
        // Pass through deliberate
        Ok(TraversalResult::Continue)
    }

    fn visit_variable_declaration(
        &mut self,
        _mode: TreeTraversalMode,
        var_dec: &mut VariableDeclaration,
    ) -> Result<TraversalResult, String> {
        println!("Variable declaration: {:#?}", var_dec);
        println!("");
        if let Some(typename) = &var_dec.typename.resolved {
            let _ = var_dec.name.visit(self)?;
            var_dec.name.type_reference = Some(typename.clone());

            if let Some(symbol) = &var_dec.name.resolved {
                // TODO: Check that symbol is unique
                self.symbol_table
                    .type_of
                    .insert(symbol.clone(), typename.clone());

                Ok(TraversalResult::SkipChildren)
            } else {
                println!("Error in {:#?}", var_dec);
                Err(format!(
                    "Could not resolve symbol for {}",
                    var_dec.name.unresolved
                ))
            }
        } else {
            Err(format!(
                "Could not resolve type for {}",
                var_dec.name.unresolved
            ))
        }
    }

    fn visit_concrete_function(
        &mut self,
        _mode: TreeTraversalMode,
        fnc: &mut ConcreteFunction,
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
        fnc.name.visit(self)?;

        self.push_namespace(fnc.name.unresolved.clone());

        for arg in fnc.arguments.iter_mut() {
            arg.visit(self)?;
        }

        fnc.body.visit(self)?;

        self.pop_namespace();
        self.pop_namespace();

        // TODO: collect type of function
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_symbol_kind(
        &mut self,
        _mode: TreeTraversalMode,
        _kind: &mut IrIndentifierKind,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_symbol_name(
        &mut self,
        _mode: TreeTraversalMode,
        symbol: &mut IrIdentifier,
    ) -> Result<TraversalResult, String> {
        match symbol.kind {
            IrIndentifierKind::TypeName => (),
            _ => {
                if let Some(ns) = &self.namespace {
                    symbol.resolved = Some(
                        format!("{}{}{}", ns, NAMESPACE_SEPARATOR, symbol.unresolved).to_string(),
                    );
                }
            }
        }
        symbol.type_reference = self.type_of(symbol);
        Ok(TraversalResult::Continue)
    }

    fn visit_highlevel_ir(
        &mut self,
        _mode: TreeTraversalMode,
        _highlevel_ir: &mut HighlevelIr,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_function_body(
        &mut self,
        _mode: TreeTraversalMode,
        _function_body: &mut FunctionBody,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_function_kind(
        &mut self,
        _mode: TreeTraversalMode,
        _function_kind: &mut FunctionKind,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_operation(
        &mut self,
        _mode: TreeTraversalMode,
        _operation: &mut Operation,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_instruction(
        &mut self,
        _mode: TreeTraversalMode,
        instr: &mut Instruction,
    ) -> Result<TraversalResult, String> {
        if let Some(_) = instr.result_type {
            return Ok(TraversalResult::SkipChildren);
        }

        let typename = match &mut instr.operation {
            Operation::Jump(_) => "Void".to_string(),
            Operation::ConditionalJump {
                expression,
                on_success,
                on_failure,
            } => {
                expression.visit(self)?;
                on_success.visit(self)?;
                on_failure.visit(self)?;
                "Void".to_string()
            }
            Operation::MemLoad => "TODO".to_string(),
            Operation::MemStore => "TODO".to_string(),
            Operation::IsEqual { left, right } => {
                left.visit(self)?;
                right.visit(self)?;
                "Int8".to_string()
            }
            Operation::CallExternalFunction { name, arguments } => {
                name.visit(self)?;
                for arg in arguments.iter_mut() {
                    arg.visit(self)?;
                }

                "TODO-lookup".to_string()
            }
            Operation::CallFunction { name, arguments } => {
                name.visit(self)?;
                for arg in arguments.iter_mut() {
                    arg.visit(self)?;
                }

                "TODO-lookup".to_string()
            }
            Operation::CallStaticFunction {
                name,
                owner: _,
                arguments,
            } => {
                name.visit(self)?;
                for arg in arguments.iter_mut() {
                    arg.visit(self)?;
                }

                "TODO-lookup".to_string()
            }
            Operation::CallMemberFunction {
                name,
                owner: _,
                arguments,
            } => {
                name.visit(self)?;
                for arg in arguments.iter_mut() {
                    arg.visit(self)?;
                }

                "TODO-lookup".to_string()
            }
            Operation::ResolveSymbol { symbol } => {
                symbol.visit(self)?;
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
                typename.visit(self)?;

                println!("Visiting literal {} {:?} ", data, typename);
                match &typename.type_reference {
                    Some(t) => t.clone(),
                    None => {
                        println!("Returning error??");
                        return Err(format!(
                            "Unable to determine type for literal {} {}",
                            typename.unresolved, data
                        ));
                    }
                }
            }
            Operation::AcceptTransfer => "Void".to_string(),
            Operation::PhiNode(inputs) => {
                let mut type_name = None;
                for input in inputs.iter_mut() {
                    input.visit(self)?;
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
                    "Void".to_string() // TODO: specify somewhere
                }
            }
        };

        println!(" ?? SHould set: {:?}", instr.ssa_name);
        if let Some(ssa) = &mut instr.ssa_name {
            ssa.visit(self)?;
            if let Some(symbol_name) = &ssa.resolved {
                println!(" => Setting type reference: {:?} ", ssa);
                // TODO: Check whether symbol exists
                self.symbol_table
                    .type_of
                    .insert(symbol_name.to_string(), typename.clone());
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

    fn visit_function_block(
        &mut self,
        _mode: TreeTraversalMode,
        _block: &mut FunctionBlock,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
}
