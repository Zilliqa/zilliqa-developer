use std::collections::HashMap;

use primitive_types::U256;

use crate::{
    constants::NAMESPACE_SEPARATOR, intermediate_representation::name_generator::NameGenerator,
};

/// Struct representing the type information of a symbol.
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub symbol_name: String,
    pub typename: String,
    pub return_type: Option<String>,
    pub arguments: Vec<String>,
    pub constructor: bool,
}

/// Implementation of TypeInfo struct.
impl TypeInfo {
    /// Checks if the TypeInfo is a function.
    pub fn is_function(&self) -> bool {
        match self.return_type {
            Some(_) => true,
            None => false,
        }
    }
    /// Checks if the TypeInfo is a constructor.
    pub fn is_constructor(&self) -> bool {
        self.constructor
    }
}

/// Struct representing the state layout entry.
#[derive(Debug, Clone)]
pub struct StateLayoutEntry {
    pub address_offset: U256,
    pub size: u64,
    pub initializer: U256,
}

/// Struct representing the symbol table.
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub aliases: HashMap<String, String>,
    pub type_of_table: HashMap<String, Box<TypeInfo>>,
    pub name_generator: NameGenerator,
    pub state_layout: HashMap<String, StateLayoutEntry>,
}

/// Trait for constructing a new symbol table.
pub trait SymbolTableConstructor {
    fn new_symbol_table(&self) -> SymbolTable;
}

/// Implementation of SymbolTable struct.
impl SymbolTable {
    /// Checks if the given name is a state.
    pub fn is_state(&self, name: &String) -> bool {
        self.state_layout.get(name).is_some()
    }

    /// Resolves the qualified name of a symbol.
    pub fn resolve_qualified_name(
        &mut self,
        basename: &String,
        current_namespace: &Option<String>,
    ) -> Option<String> {
        match &current_namespace {
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

                    if let Some(_) = self.typename_of(full_name) {
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

        if let Some(_) = self.typename_of(lookup) {
            return Some(lookup.to_string());
        }

        None
    }

    /// Returns a mutable reference to the name generator.
    pub fn get_name_generator(&mut self) -> &mut NameGenerator {
        &mut self.name_generator
    }

    /// Creates a plain typename.
    pub fn create_plain_typename(&self, typename: &str) -> Box<TypeInfo> {
        Box::new(TypeInfo {
            symbol_name: "".to_string(),
            typename: typename.to_string(),
            return_type: None,
            arguments: Vec::new(),
            constructor: false,
        })
    }

    /// Returns the type of a symbol.
    pub fn type_of(&self, name: &str, namespace: &Option<String>) -> Option<Box<TypeInfo>> {
        if let Some(namespace) = &namespace {
            // Split the namespace into parts
            let parts: Vec<&str> = namespace.split("::").collect();

            // Iterate over the parts from most specific to least specific
            for i in (0..=parts.len()).rev() {
                let qualified_name = format!("{}::{}", parts[0..i].join("::"), name);
                if let Some(value) = self.type_of_table.get(&qualified_name) {
                    return Some(value.clone());
                }
            }
        }

        self.type_of_table.get(name).cloned()
    }

    /// Returns the typename of a symbol.
    pub fn typename_of(&self, name: &str) -> Option<String> {
        if let Some(ti) = self.type_of_table.get(name) {
            Some(ti.typename.clone())
        } else {
            None
        }
    }

    /// Checks if a symbol is a function.
    pub fn is_function(&self, name: &str) -> bool {
        if let Some(ti) = self.type_of_table.get(name) {
            ti.is_function()
        } else {
            false
        }
    }

    /// Declares a function or constructor type.
    pub fn declare_function_or_constructor_type(
        &mut self,
        symbol: &str,
        arguments: &Vec<String>,
        return_type: &str,
        constructor: bool,
    ) -> Result<String, String> {
        let mut signature: String = "(".to_string();
        for arg in arguments.iter() {
            signature.push_str(&arg);
        }
        signature.push_str(") -> ");
        signature.push_str(return_type);

        let typeinfo = Box::new(TypeInfo {
            symbol_name: symbol.to_string(),
            typename: signature.clone(),
            return_type: Some(return_type.to_string()),
            arguments: Vec::new(),
            constructor,
        });

        // TODO: Consider whether it would be cleaner with an alias
        self.type_of_table
            .insert(symbol.to_string(), typeinfo.clone());

        self.type_of_table.insert(signature, typeinfo);
        Ok(symbol.to_string())
    }

    /// Declares a function type.
    pub fn declare_function_type(
        &mut self,
        symbol: &str,
        arguments: &Vec<String>,
        return_type: &str,
    ) -> Result<String, String> {
        self.declare_function_or_constructor_type(symbol, arguments, return_type, false)
    }

    /// Declares a constructor.
    pub fn declare_constructor(
        &mut self,
        symbol: &str,
        arguments: &Vec<String>,
        return_type: &str,
    ) -> Result<String, String> {
        self.declare_function_or_constructor_type(symbol, arguments, return_type, true)
    }

    /// Declares the type of a symbol.
    pub fn declare_type_of(&mut self, symbol: &str, typename: &str) -> Result<String, String> {
        let typeinfo = self.create_plain_typename(typename);

        self.type_of_table.insert(symbol.to_string(), typeinfo);

        Ok(symbol.to_string())
    }

    /// Declares a special variable.
    pub fn declare_special_variable(
        &mut self,
        name: &str,
        typename: &str,
    ) -> Result<String, String> {
        self.declare_type_of(name, typename)
    }

    /// Declares a type.
    pub fn declare_type(&mut self, symbol: &str) -> Result<String, String> {
        self.declare_type_of(symbol, symbol)
    }

    /// Declares an alias for a symbol.
    pub fn declare_alias(&mut self, alias: &str, symbol: &str) -> Result<String, String> {
        self.aliases.insert(alias.to_string(), symbol.to_string());
        Ok(symbol.to_string())
    }
}
