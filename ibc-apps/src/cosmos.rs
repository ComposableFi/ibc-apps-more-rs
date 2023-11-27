//! like parts of https://github.com/cosmos/cosmos-rust/tree/main/cosmrs
//! but with no_std support
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