use std::collections::HashMap;

pub struct SymbolTable {
    pub aliases: HashMap<String, String>,
    pub type_of: HashMap<String, String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut type_of = HashMap::new();

        // TODO: Get types from RuntimeModule
        type_of.insert("Int8".to_string(), "Int8".to_string());
        type_of.insert("Int16".to_string(), "Int16".to_string());
        type_of.insert("Int32".to_string(), "Int32".to_string());
        type_of.insert("Int64".to_string(), "Int64".to_string());
        type_of.insert("Uint8".to_string(), "Uint8".to_string());
        type_of.insert("Uint16".to_string(), "Uint16".to_string());
        type_of.insert("Uint32".to_string(), "Uint32".to_string());
        type_of.insert("Uint64".to_string(), "Uint64".to_string());

        SymbolTable {
            aliases: HashMap::new(),
            type_of,
        }
    }
}
