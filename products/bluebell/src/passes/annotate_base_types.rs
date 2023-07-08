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
use crate::symbol_table::TypeInfo;
use std::mem;

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
    pub fn typename_of(&self, symbol: &IrIdentifier, symbol_table: &mut SymbolTable) -> Option<String> {
        if let Some(name) = &symbol.resolved {
            symbol_table.typename_of(name)
        } else {
            None
        }
    }

    // TODO: Make Symbol table member
    pub fn type_of(&self, symbol: &IrIdentifier, symbol_table: &mut SymbolTable) -> Option<Box<TypeInfo>> {
        if let Some(name) = &symbol.resolved {
            symbol_table.type_of(name)
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

impl HighlevelIrPass for AnnotateBaseTypes {
    fn visit_concrete_type(
        &mut self,
        _mode: TreeTraversalMode,
        _con_type: &mut ConcreteType,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_enum_value(
        &mut self,
        _mode: TreeTraversalMode,
        _enum_value: &mut EnumValue,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_tuple(
        &mut self,
        _mode: TreeTraversalMode,
        _tuple: &mut Tuple,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_variant(
        &mut self,
        _mode: TreeTraversalMode,
        _variant: &mut Variant,
        symbol_table: &mut SymbolTable,
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
        symbol_table: &mut SymbolTable,
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
                    symbol.type_reference = Some(typeinfo.name.clone());

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

                    let mut intermediate_symbol =
                        symbol_table.name_generator.new_intermediate();

                    symbol.kind = IrIndentifierKind::StaticFunctionName;
                    let mut constructor_call = Box::new(Instruction {
                        ssa_name: Some(intermediate_symbol.clone()),
                        result_type: Some(IrIdentifier {
                            unresolved: return_type.clone(),
                            resolved: Some(return_type.clone()),
                            type_reference: Some(return_type),
                            kind: IrIndentifierKind::TypeName,
                            is_definition: false,
                        }),
                        operation: Operation::CallStaticFunction {
                            name: symbol.clone(),
                            owner: None, // TODO:
                            arguments: Vec::new(),
                        },
                    });

                    constructor_call.visit(self, symbol_table)?;

                    if let Some(current_block) = &mut self.current_block {
                        current_block.instructions.push(constructor_call);
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

        match symbol.kind {
            IrIndentifierKind::BlockLabel => (),
            IrIndentifierKind::FunctionName
            | IrIndentifierKind::TransitionName
            | IrIndentifierKind::ProcedureName
            | IrIndentifierKind::VirtualRegister
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

    fn visit_highlevel_ir(
        &mut self,
        _mode: TreeTraversalMode,
        _highlevel_ir: &mut HighlevelIr,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_function_body(
        &mut self,
        _mode: TreeTraversalMode,
        _function_body: &mut FunctionBody,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_function_kind(
        &mut self,
        _mode: TreeTraversalMode,
        _function_kind: &mut FunctionKind,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_operation(
        &mut self,
        _mode: TreeTraversalMode,
        _operation: &mut Operation,
        symbol_table: &mut SymbolTable,
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
            Operation::Noop => "Void".to_string(),
            Operation::Jump(_) => "Void".to_string(),
            Operation::ConditionalJump {
                expression,
                on_success,
                on_failure,
            } => {
                expression.visit(self, symbol_table)?;
                on_success.visit(self, symbol_table)?;
                on_failure.visit(self, symbol_table)?;
                "Void".to_string()
            }
            Operation::MemLoad => "TODO".to_string(),
            Operation::MemStore => "TODO".to_string(),
            Operation::IsEqual { left, right } => {
                left.visit(self, symbol_table)?;
                right.visit(self, symbol_table)?;
                //  panic!("Failed");
                "Int8".to_string()
            }
            Operation::CallExternalFunction { name, arguments } => {
                name.visit(self, symbol_table)?;
                for arg in arguments.iter_mut() {
                    arg.visit(self, symbol_table)?;
                }

                "TODO-lookup".to_string()
            }
            Operation::CallFunction { name, arguments } => {
                name.visit(self, symbol_table)?;
                for arg in arguments.iter_mut() {
                    arg.visit(self, symbol_table)?;
                }

                "TODO-lookup".to_string()
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
                    let function_typeinfo = symbol_table.type_of(function_type);

                    if let Some(function_typeinfo) = function_typeinfo {
                        function_typeinfo.return_type.expect("").clone()
                    } else {
                        return Err("Unable to determine of {:?}".to_string());
                    }
                } else {
                    return Err("Unable to determine return type of {:?}".to_string());
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

                "TODO-lookup".to_string()
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
            Operation::AcceptTransfer => "Void".to_string(),
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
                    "Void".to_string() // TODO: specify somewhere
                }
            }
        };

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

    fn visit_function_block(
        &mut self,
        _mode: TreeTraversalMode,
        block: &mut FunctionBlock,
        symbol_table: &mut SymbolTable,
    ) -> Result<TraversalResult, String> {
        self.current_block = Some(FunctionBlock {
            name: block.name.clone(),
            instructions: Vec::new(),
            terminated: block.terminated,
        });

        for instr in block.instructions.iter_mut() {
            instr.visit(self, symbol_table)?;
            if let Some(ref mut new_block) = &mut self.current_block {
                new_block.instructions.push(instr.clone());
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
