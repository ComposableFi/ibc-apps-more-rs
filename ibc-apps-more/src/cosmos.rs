//! like parts of https://github.com/cosmos/cosmos-rust/tree/main/cosmrs
//! but with no_std support
use ibc_apps::transfer::types::PrefixedDenom;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// Hash creates a new address from address type and key.
// The functions should only be used by new types defining their own address function
// (eg public keys).
/// https://github.com/cosmos/cosmos-sdk/blob/main/types/address/hash.go
pub fn addess_hash(typ: &str, key: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::default();
    hasher.update(typ.as_bytes());
    let th = hasher.finalize();
    let mut hasher = Sha256::default();
    hasher.update(th);
    hasher.update(key);
    hasher.finalize().into()
}

// takes a transfer message and returns ibc/<hash of denom>
// https://ibc.cosmos.network/main/architecture/adr-001-coin-source-tracing.html
// so can infer for some chain denom on hops
pub fn hash_denom_trace(denom: &PrefixedDenom) -> String {
    let denom = denom.to_string();
    let digest = Sha256::digest(denom.as_bytes());
    ["ibc/", &hex::encode_upper(digest)].concat()
}
/// Coin defines a token with a denomination and an amount.
///
/// NOTE: The amount field is an Int which implements the custom method
/// signatures required by gogoproto.
/// modifications: adds cosmwasm_std::Coin mappin and JsonSchema
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, prost::Message, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: String,
    #[prost(string, tag = "2")]
    pub amount: String,
}

impl Coin {
    pub const PROTO_MESSAGE_URL: &str = "/cosmos.base.v1beta1.Coin";
}

#[cfg(feature = "cosmwasm")]
impl TryInto<cosmwasm_std::Coin> for Coin {
    type Error = cosmwasm_std::StdError;

    fn try_into(self) -> Result<cosmwasm_std::Coin, Self::Error> {
        Ok(cosmwasm_std::Coin {
            denom: self.denom,
            amount: self.amount.parse()?,
        })
    }
}

impl alloc::fmt::Display for Coin {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(f, "{}{}", self.amount, self.denom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_denom() {
        let pica =
            hash_denom_trace(&PrefixedDenom::from_str("transfer/channel-1/1").expect("const"));
        assert_eq!(
            pica,
            "ibc/71B5DB2263A5A5B160BBA26A307BF5441BDB330534C19A9F551F63D9CC0C3026"
        );
    }
}
