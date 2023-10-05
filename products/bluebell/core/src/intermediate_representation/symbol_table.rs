use crate::constants::NAMESPACE_SEPARATOR;
use crate::intermediate_representation::name_generator::NameGenerator;
use primitive_types::U256;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub return_type: Option<String>,
    pub arguments: Vec<String>,
    pub constructor: bool,
}

impl TypeInfo {
    pub fn is_function(&self) -> bool {
        match self.return_type {
            Some(_) => true,
            None => false,
        }
    }
    pub fn is_constructor(&self) -> bool {
        self.constructor
    }
}

#[derive(Debug, Clone)]
pub struct StateLayoutEntry {
    pub address_offset: U256,
    pub size: u64,
    pub initializer: U256,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub aliases: HashMap<String, String>,
    pub type_of_table: HashMap<String, Box<TypeInfo>>,
    pub name_generator: NameGenerator,
    pub state_layout: HashMap<String, StateLayoutEntry>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let type_of_table = HashMap::new();

        let mut ret = SymbolTable {
            aliases: HashMap::new(),
            type_of_table,
            name_generator: NameGenerator::new(),
            state_layout: HashMap::new(),
        };

        // TODO: Get types from RuntimeModule
        // TODO: Deal with potential errors
        let _ = ret.declare_type("Int8");
        let _ = ret.declare_type("Int16");
        let _ = ret.declare_type("Int32");
        let _ = ret.declare_type("Int64");
        let _ = ret.declare_type("Uint8");
        let _ = ret.declare_type("Uint16");
        let _ = ret.declare_type("Uint32");
        let _ = ret.declare_type("Uint64");
        let _ = ret.declare_type("String");
        let _ = ret.declare_type("ByStr20");

        let _ = ret.declare_special_variable("_sender", "ByStr20");

        ret
    }

    pub fn is_state(&self, name: &String) -> bool {
        self.state_layout.get(name).is_some()
    }

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

    pub fn get_name_generator(&mut self) -> &mut NameGenerator {
        &mut self.name_generator
    }

    pub fn create_plain_typename(&self, typename: &str) -> Box<TypeInfo> {
        Box::new(TypeInfo {
            name: typename.to_string(),
            return_type: None,
            arguments: Vec::new(),
            constructor: false,
        })
    }

    pub fn type_of(&self, name: &str) -> Option<Box<TypeInfo>> {
        self.type_of_table.get(name).cloned()
    }

    pub fn typename_of(&self, name: &str) -> Option<String> {
        if let Some(ti) = self.type_of_table.get(name) {
            Some(ti.name.clone())
        } else {
            None
        }
    }

    pub fn is_function(&self, name: &str) -> bool {
        if let Some(ti) = self.type_of_table.get(name) {
            ti.is_function()
        } else {
            false
        }
    }

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
            name: signature.clone(),
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

    pub fn declare_function_type(
        &mut self,
        symbol: &str,
        arguments: &Vec<String>,
        return_type: &str,
    ) -> Result<String, String> {
        self.declare_function_or_constructor_type(symbol, arguments, return_type, false)
    }

    pub fn declare_constructor(
        &mut self,
        symbol: &str,
        arguments: &Vec<String>,
        return_type: &str,
    ) -> Result<String, String> {
        self.declare_function_or_constructor_type(symbol, arguments, return_type, true)
    }

    pub fn declare_type_of(&mut self, symbol: &str, typename: &str) -> Result<String, String> {
        let typeinfo = self.create_plain_typename(typename);

        self.type_of_table.insert(symbol.to_string(), typeinfo);

        Ok(symbol.to_string())
    }

    pub fn declare_special_variable(
        &mut self,
        name: &str,
        typename: &str,
    ) -> Result<String, String> {
        self.declare_type_of(name, typename)
    }

    pub fn declare_type(&mut self, symbol: &str) -> Result<String, String> {
        self.declare_type_of(symbol, symbol)
    }

    pub fn declare_alias(&mut self, alias: &str, symbol: &str) -> Result<String, String> {
        self.aliases.insert(alias.to_string(), symbol.to_string());
        Ok(symbol.to_string())
    }
}
