use core::str::FromStr;

use primitive_types::U256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvmTypeValue {
    Uint32(u32),
    Uint64(u64),
    Uint256(U256),
    String(String),
    // Address(Address),
    // Add more types as needed
    StackReference(u32),
}

impl Serialize for EvmTypeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO: Update to match whatever is the standard for Ethereum
        match self {
            EvmTypeValue::Uint32(value) => serializer.serialize_u32(*value),
            EvmTypeValue::Uint64(value) => serializer.serialize_u64(*value),
            EvmTypeValue::Uint256(value) => {
                let s = value.to_string();
                serializer.serialize_str(&s)
            }
            EvmTypeValue::String(value) => serializer.serialize_str(value),
            EvmTypeValue::StackReference(value) => serializer.serialize_u32(*value),
        }
    }
}

impl<'de> Deserialize<'de> for EvmTypeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO: Update to match whatever is the standard for Ethereum
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::Number(num) => {
                if num.is_u64() {
                    Ok(EvmTypeValue::Uint64(num.as_u64().unwrap()))
                } else {
                    Err(serde::de::Error::custom("Invalid integer value"))
                }
            }
            Value::String(s) => Ok(EvmTypeValue::String(s)),
            _ => Err(serde::de::Error::custom("Unsupported type")),
        }
    }
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
            EvmTypeValue::String(value) => value.as_bytes().to_vec(),
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

impl FromStr for EvmType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_ascii_lowercase();

        if s_lower == "address" {
            Ok(EvmType::Address)
        } else if s_lower == "bool" {
            Ok(EvmType::Bool)
        } else if s_lower == "string" {
            Ok(EvmType::String)
        } else if s_lower.starts_with("uint") {
            s[4..]
                .parse::<usize>()
                .map(|size| EvmType::Uint(size))
                .map_err(|_| "Error parsing Uint size")
        } else if s_lower.starts_with("int") {
            s[3..]
                .parse::<usize>()
                .map(|size| EvmType::Int(size))
                .map_err(|_| "Error parsing Int size")
        } else if s_lower.starts_with("bystr") {
            s[5..]
                .parse::<usize>()
                .map(|size| EvmType::Bytes(size))
                .map_err(|_| "Error parsing Bytes size")
        } else {
            Err("Unknown type")
        }
    }
}
