// ! taken from ZQ2

use std::str::FromStr;

use anyhow::{anyhow, Result};
use ethers::{
    core::k256,
    signers::{LocalWallet, WalletError},
    utils::hex,
};

/// The secret key type used as the basis of all cryptography in the node.
/// Any of the `NodePublicKey` or `TransactionPublicKey`s, or a libp2p identity, can be derived
/// from this.
#[derive(Debug, Clone, Copy)]
pub struct SecretKey {
    bytes: [u8; 32],
}

impl SecretKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<SecretKey> {
        let bytes: [u8; 32] = bytes.try_into()?;

        if bytes == [0; 32] {
            return Err(anyhow!("bytes are all zero"));
        }

        Ok(SecretKey { bytes })
    }

    pub fn from_hex(s: &str) -> Result<SecretKey> {
        let bytes_vec = hex::decode(s)?;
        Self::from_bytes(&bytes_vec)
    }

    fn as_ecdsa(&self) -> k256::ecdsa::SigningKey {
        // `SigningKey::from_bytes` can fail for two reasons:
        // 1. The bytes represent a zero integer. However, we validate this is not the case on construction.
        // 2. The bytes represent an integer less than the curve's modulus. However for ECDSA, the curve's order is
        //    equal to its modulus, so this is impossible.
        // Therefore, it is safe to unwrap here.
        k256::ecdsa::SigningKey::from_bytes(&self.bytes.into()).unwrap()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    pub fn as_wallet(&self) -> Result<LocalWallet, WalletError> {
        LocalWallet::from_str(self.to_hex().as_str())
    }

    pub fn to_libp2p_keypair(&self) -> libp2p::identity::Keypair {
        let keypair: libp2p::identity::ed25519::Keypair = libp2p::identity::ed25519::SecretKey::try_from_bytes(self.bytes)
            .expect("`SecretKey::from_bytes` returns an `Err` only when the length is not 32, we know the length is 32")
            .into();
        keypair.into()
    }
}
