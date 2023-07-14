use crate::intermediate_name_generator::IntermediateNameGenerator;
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

pub struct SymbolTable {
    pub aliases: HashMap<String, String>,
    pub type_of_table: HashMap<String, Box<TypeInfo>>,
    pub name_generator: IntermediateNameGenerator,
}

impl SymbolTable {
    pub fn new() -> Self {
        let type_of_table = HashMap::new();

        let mut ret = SymbolTable {
            aliases: HashMap::new(),
            type_of_table,
            name_generator: IntermediateNameGenerator::new(),
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

        ret
    }

    pub fn get_name_generator(&mut self) -> &mut IntermediateNameGenerator {
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

    pub fn declare_type(&mut self, symbol: &str) -> Result<String, String> {
        self.declare_type_of(symbol, symbol)
    }

    pub fn declare_alias(&mut self, alias: &str, symbol: &str) -> Result<String, String> {
        self.aliases.insert(alias.to_string(), symbol.to_string());
        Ok(symbol.to_string())
    }
}
