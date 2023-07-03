use crate::constants::NAMESPACE_SEPARATOR;
use crate::constants::{TraversalResult, TreeTraversalMode};
use crate::highlevel_ir::Instruction;
use crate::highlevel_ir::{
    ConcreteFunction, ConcreteType, EnumValue, FunctionBlock, FunctionBody, FunctionKind,
    HighlevelIr, IrIdentifier, IrIndentifierKind, IrLowering, Operation, Tuple,
    VariableDeclaration, Variant,
};
use crate::highlevel_ir_pass::HighlevelIrPass;
use crate::highlevel_ir_pass_executor::HighlevelIrPassExecutor;
use std::collections::HashMap;

pub struct HighlevelIrTypeCollection {
    namespace_stack: Vec<String>,
    current_namespace: Option<String>,
    current_type: Option<String>,

    aliases: HashMap<String, String>,
    type_of: HashMap<String, String>,
}

impl HighlevelIrTypeCollection {
    pub fn new() -> Self {
        let mut type_of = HashMap::new();

        type_of.insert("Int32".to_string(), "Int32".to_string());
        type_of.insert("Uint32".to_string(), "Uint32".to_string());

        HighlevelIrTypeCollection {
            namespace_stack: Vec::new(),
            current_namespace: None,
            current_type: None,
            aliases: HashMap::new(),
            type_of,
        }
    }

    fn resolve_qualified_name(&self, basename: &String) -> Option<String> {
        match &self.current_namespace {
            None => (),
            Some(namespace) => {
                let mut namespaces = namespace.split(NAMESPACE_SEPARATOR).collect::<Vec<&str>>();

                while !namespaces.is_empty() {
                    let full_name = format!(
                        "{}{}{}",
                        namespaces.join(NAMESPACE_SEPARATOR),
                        NAMESPACE_SEPARATOR,
                        basename
                    );

                    let full_name = if let Some(aliased_name) = self.aliases.get(&full_name) {
                        aliased_name
                    } else {
                        &full_name
                    };

                    if let Some(_) = self.type_of.get(full_name) {
                        return Some(full_name.to_string());
                    }

                    // Remove the last level of the namespace
                    namespaces.pop();
                }
            }
        }

        let lookup = if let Some(aliased_name) = self.aliases.get(basename) {
            aliased_name
        } else {
            basename
        };

        if let Some(_) = self.type_of.get(lookup) {
            return Some(lookup.to_string());
        }

        None
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

impl HighlevelIrPass for HighlevelIrTypeCollection {
    fn visit_concrete_type(
        &mut self,
        mode: TreeTraversalMode,
        con_type: &mut ConcreteType,
    ) -> Result<TraversalResult, String> {
        match con_type {
            ConcreteType::Tuple {
                name,
                namespace,
                data_layout,
            } => {
                let _ = namespace.visit(self)?;
                self.push_namespace(namespace.qualified_name()?);

                let _ = name.visit(self)?;
                let qualified_name = name.qualified_name()?;
                self.type_of
                    .insert(qualified_name.clone(), qualified_name.clone());

                // Backgwards compatibility support
                // TODO: Enable and disable this with flag
                self.aliases
                    .insert(name.unresolved.clone(), qualified_name.clone());

                self.current_type = Some(qualified_name);
                let _ = data_layout.visit(self)?;

                self.current_type = None;

                self.pop_namespace();
            }
            ConcreteType::Variant {
                name,
                namespace,
                data_layout,
            } => {
                let _ = namespace.visit(self)?;
                self.push_namespace(namespace.qualified_name()?);

                let _ = name.visit(self)?;
                let qualified_name = name.qualified_name()?;
                self.type_of
                    .insert(qualified_name.clone(), qualified_name.clone());

                // Backgwards compatibility support
                // TODO: Enable and disable this with flag
                self.aliases
                    .insert(name.unresolved.clone(), qualified_name.clone());

                self.current_type = Some(qualified_name);
                let _ = data_layout.visit(self)?;

                self.current_type = None;
                self.pop_namespace();
            }
        }
        Ok(TraversalResult::SkipChildren)
    }

    fn visit_enum_value(
        &mut self,
        mode: TreeTraversalMode,
        enum_value: &mut EnumValue,
    ) -> Result<TraversalResult, String> {
        if let Some(typescope) = &self.current_type {
            self.push_namespace(typescope.to_string());
            let _ = enum_value.name.visit(self)?;
            self.pop_namespace();

            // TODO: Work out whehter we should attempt to resolve the type right away?
            if let Some(data) = &mut enum_value.data {
                let _ = data.visit(self)?;
            }

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
        mode: TreeTraversalMode,
        tuple: &mut Tuple,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_variant(
        &mut self,
        mode: TreeTraversalMode,
        variant: &mut Variant,
    ) -> Result<TraversalResult, String> {
        // Pass through deliberate
        Ok(TraversalResult::Continue)
    }

    fn visit_variable_declaration(
        &mut self,
        _mode: TreeTraversalMode,
        _var_dec: &mut VariableDeclaration,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_concrete_function(
        &mut self,
        _mode: TreeTraversalMode,
        _fnc: &mut ConcreteFunction,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
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
        mode: TreeTraversalMode,
        symbol: &mut IrIdentifier,
    ) -> Result<TraversalResult, String> {
        match symbol.kind {
            IrIndentifierKind::BlockLabel | IrIndentifierKind::Namespace => {
                symbol.resolved = Some(symbol.unresolved.clone());
            }
            _ => {
                if symbol.is_definition {
                    if let Some(namespace) = &self.current_namespace {
                        symbol.resolved = Some(
                            format!("{}{}{}", namespace, NAMESPACE_SEPARATOR, symbol.unresolved)
                                .to_string(),
                        );
                    }
                } else if let Some(resolved_name) = self.resolve_qualified_name(&symbol.unresolved)
                {
                    symbol.resolved = Some(resolved_name);
                }
            }
        }

        Ok(TraversalResult::SkipChildren)
    }

    fn visit_highlevel_ir(
        &mut self,
        mode: TreeTraversalMode,
        _highlevel_ir: &mut HighlevelIr,
    ) -> Result<TraversalResult, String> {
        match mode {
            TreeTraversalMode::Enter => (),
            TreeTraversalMode::Exit => {
                println!("Types: {:#?}\n\n", self.type_of);
                println!("Aliases: {:#?}\n\n", self.aliases);
            }
        }
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
        _instr: &mut Instruction,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }

    fn visit_function_block(
        &mut self,
        _mode: TreeTraversalMode,
        _block: &mut FunctionBlock,
    ) -> Result<TraversalResult, String> {
        Ok(TraversalResult::Continue)
    }
}
