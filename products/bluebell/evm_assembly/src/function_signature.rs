use crate::{
    block::EvmBlock,
    types::{EvmType, EvmTypeValue},
};

pub type AssemblyBuilderFn = fn(&mut EvmBlock);

#[derive(Debug, Clone)]
pub struct EvmFunctionSignature {
    pub name: String,
    pub arguments: Vec<EvmType>,
    pub return_type: EvmType,

    pub inline_assembly_generator: Option<AssemblyBuilderFn>,
    pub external_address: Option<u32>,
}

impl EvmFunctionSignature {
    pub fn new(name: String, arguments: Vec<EvmType>, return_type: &EvmType) -> Self {
        Self {
            name,
            arguments,
            return_type: return_type.clone(),
            inline_assembly_generator: None,
            external_address: None,
        }
    }

    pub fn signature(&self) -> String {
        let mut argnames = Vec::new();
        for arg in &self.arguments {
            argnames.push(arg.signature());
        }

        format!("{}({})", self.name, argnames.join(",")).to_string()
    }

    pub fn full_signature(&self) -> String {
        let mut argnames = Vec::new();
        for arg in &self.arguments {
            argnames.push(arg.signature());
        }

        format!(
            "{}({})->{}",
            self.name,
            argnames.join(","),
            self.return_type.signature()
        )
        .to_string()
    }

    pub fn selector(&self) -> Vec<u8> {
        let signature = self.signature();
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(signature);

        let mut selector = Vec::new();
        selector.extend_from_slice(&hash[..4]);
        selector
    }

    pub fn generate_transaction_data(&self, args: Vec<EvmTypeValue>) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(self.selector());

        // Encode the arguments
        for arg in args {
            data.extend(arg.to_bytes());
        }

        data
    }
}
