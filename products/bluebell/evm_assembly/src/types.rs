use primitive_types::U256;

#[derive(Clone, Debug)]
pub enum EvmTypeValue {
    Uint32(u32),
    Uint64(u64),
    Uint256(U256),
    // Address(Address),
    // Add more types as needed
    StackReference(u32),
}

impl EvmTypeValue {
    fn pad_byte_array(bytes: Vec<u8>) -> Vec<u8> {
        let padding_size = 32 - bytes.len();
        let mut ret = vec![0; padding_size];
        ret.extend(bytes);
        ret
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        Self::pad_byte_array(self.to_bytes_unpadded())
    }

    pub fn to_bytes_unpadded(&self) -> Vec<u8> {
        match self {
            EvmTypeValue::Uint32(value) => value.to_be_bytes().to_vec(),
            EvmTypeValue::Uint64(value) => value.to_be_bytes().to_vec(),
            // EvmTypeValue::Uint256(value) => pad_byte_array(value.to_big_endian(/* &mut [u8] */).to_vec()),
            // TODO EvmTypeValue::Address(value) => pad_byte_array(value.as_bytes().to_vec()),
            // Handle other types here
            _ => panic!("Type conversion not implemented."),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EvmType {
    Uint(usize),
    Int(usize),
    Bytes(usize),
    Address,
    Bool,
    String,
}

impl EvmType {
    pub fn signature(&self) -> String {
        match self {
            EvmType::Uint(size) => format!("uint{}", size).to_string(),
            EvmType::Int(size) => format!("int{}", size).to_string(),
            EvmType::Bytes(size) => format!("bytes{}", size).to_string(),
            EvmType::Address => "address".to_string(),
            EvmType::Bool => "bool".to_string(),
            EvmType::String => "string".to_string(),
        }
    }
}
