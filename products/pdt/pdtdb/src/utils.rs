// Here
use anyhow::Result;
use base64::Engine;
use pdtlib::proto::ByteArray;
use primitive_types::H160;
use sha2::{Digest, Sha256};
use sha3::Keccak256;

#[derive(Clone, Debug)]
pub struct ProcessCoordinates {
    /// How many machines are processing this dataset?
    pub nr_machines: i64,
    /// How many blocks in a batch?
    pub batch_blks: i64,
    /// What is the id of the machine currently running?
    pub machine_id: i64,
    /// A name for this machine, to print in logs.
    pub client_id: String,
}

impl ProcessCoordinates {
    pub fn with_machine_id(&self, machine_id: i64) -> Self {
        Self {
            machine_id,
            ..self.clone()
        }
    }

    pub fn with_client_id(&self, client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            ..self.clone()
        }
    }
}

pub enum API {
    Ethereum,
    Zilliqa,
}

/// address_from_public_key() but generate hex.
pub fn maybe_hex_address_from_public_key(pubkey: &[u8], api: API) -> Option<String> {
    address_from_public_key(pubkey, api)
        .map(|x| hex::encode(x.as_bytes()))
        .ok()
}

/// Address from public key.
/// Given a pubkey (without the leading 0x), produce an address (without leading 0x)
/// Following the code in zilliqa-js/crypto/util.ts:getAddressFromPublicKey()
pub fn address_from_public_key(pubkey: &[u8], api: API) -> Result<H160> {
    match api {
        API::Ethereum => {
            let mut hasher = Keccak256::new();
            hasher.update(pubkey);
            let result = hasher.finalize();
            Ok(H160::from_slice(&result[12..]))
        }
        API::Zilliqa => {
            let mut hasher = Sha256::new();
            hasher.update(pubkey);
            let result = hasher.finalize();
            // Lop off the first 12 bytes.
            Ok(H160::from_slice(&result[12..]))
        }
    }
}

pub fn u128_string_from_storage(val: &ByteArray) -> Option<String> {
    u128_from_storage(val).map(|x| x.to_string()).ok()
}

pub fn u128_from_storage(val: &ByteArray) -> Result<u128> {
    let the_bytes: [u8; 16] = val.data[0..].try_into()?;
    Ok(u128::from_be_bytes(the_bytes))
}

pub fn encode_u8(y: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(y)
}

pub fn decode_u8(x: String) -> Vec<u8> {
    base64::engine::general_purpose::STANDARD
        .decode(x)
        .expect("base64-encoding should be decodeable")
}

#[test]
fn check_address_from_pubkey() {
    let zilliqa_data_points: Vec<(&str, &str)> = vec![
        (
            "0246E7178DC8253201101E18FD6F6EB9972451D121FC57AA2A06DD5C111E58DC6A",
            "9BFEC715a6bD658fCb62B0f8cc9BFa2ADE71434A",
        ),
        (
            "02c261017f4299f0e60d33035d7fbc4b85cbaa11cd7127e0a420b331cd805e517a",
            "acd9339df14af808af1f46a3edb7466590199ee6",
        ),
    ];

    let ethereum_data_points: Vec<(&str, &str)> = vec![(
        "0293386b38b31fe37175eb42ddde0147b8154c584d80f9a33cf9d1558a82455358",
        "0x2d3ec573a07b101656847aa109be7ba5ed8dfe5d",
    )];
    for (pubkey, addr) in zilliqa_data_points {
        let dec_addr =
            address_from_public_key(&hex::decode(pubkey).unwrap(), API::Zilliqa).unwrap();
        let hex_addr = hex::encode(dec_addr);
        assert_eq!(hex_addr.to_lowercase(), addr.to_lowercase());
    }

    for (pubkey, addr) in ethereum_data_points {
        let dec_addr =
            address_from_public_key(&hex::decode(pubkey).unwrap(), API::Ethereum).unwrap();
        let hex_addr = hex::encode(dec_addr);
        assert_eq!(hex_addr.to_lowercase(), addr.to_lowercase());
    }
}
