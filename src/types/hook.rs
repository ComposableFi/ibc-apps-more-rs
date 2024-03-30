//! osmosis hooks app

use bech32::{Bech32, Hrp};
use data_encoding::BASE32;
use ibc_core_host_types::identifiers::ChannelId;
use ibc_primitives::Signer;
use serde::{Deserialize, Serialize};

use crate::{cosmos::addess_hash, types::memo::Memo};

pub const SENDER_PREFIX: &str = "ibc-wasm-hook-intermediary";

#[derive(Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum IBCLifecycleComplete {
    #[serde(rename = "ibc_ack")]
    IBCAck {
        /// The source channel (osmosis side) of the IBC packet
        channel: ChannelId,
        /// The sequence number that the packet was sent with
        sequence: u64,
        /// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
        ack: String,
        /// Weather an ack is a success of failure according to the transfer spec
        success: bool,
    },
    #[serde(rename = "ibc_timeout")]
    IBCTimeout {
        /// The source channel (osmosis side) of the IBC packet
        channel: ChannelId,
        /// The sequence number that the packet was sent with
        sequence: u64,
    },
}

/// derives the sender address to be used when calling wasm hooks
/// https://github.com/osmosis-labs/osmosis/blob/master/x/ibc-hooks/keeper/keeper.go#L170
/// ```rust
/// use ibc_apps_more::types::hook;
/// use ibc_core_host_types::identifiers::ChannelId;
/// let channel = ChannelId::new(0);
/// let original_sender =   "juno12smx2wdlyttvyzvzg54y2vnqwq2qjatezqwqxu";
/// let hashed_sender = hook::derive_intermediate_sender(&channel, original_sender, "osmo").expect("new address");
/// assert_eq!(hashed_sender, "osmo1nt0pudh879m6enw4j6z4mvyu3vmwawjv5gr7xw6lvhdsdpn3m0qs74xdjl");
/// ```
pub fn derive_intermediate_sender(
    channel: &ChannelId,
    original_sender: &str,
    bech32_prefix: &str,
) -> Result<String, crate::types::error::HookError> {
    use data_encoding::BASE32;
    let channel_sender = alloc::format!("{channel}/{original_sender}");
    let sender_hash = addess_hash(SENDER_PREFIX, channel_sender.as_bytes());
    let mut buf = String::new();
    let hrp = Hrp::parse(bech32_prefix)?;
    bech32::encode_lower_to_fmt::<Bech32, _>(&mut buf, hrp, sender_hash.as_ref())?;
    Ok(buf)
}

/// see https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks
/// Information about which contract to call when the crosschain CW spawn finishes
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
pub struct Callback<Msg> {
    // really Addr, but it does not have scale, I guess we need to impl `type XcAddr = SS58 |
    // Bech32` with signer inside for serde
    pub contract: Signer,
    /// Is a valid JSON object. The contract will be called with this as the message.
    pub msg: Msg,
}

impl<Msg> Callback<Msg> {
    pub fn new(contract: Signer, msg: Msg) -> Self {
        Self { contract, msg }
    }

    #[cfg(feature = "cosmwasm")]
    pub fn new_cosmwasm(contract: cosmwasm_std::Addr, msg: serde_cw_value::Value) -> Self {
        Self {
            contract: contract.to_string().into(),
            msg,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
pub struct HookMemo<Next, Msg> {
    #[serde(flatten)]
    pub base: Memo<Next, Msg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibc_callback: Option<Signer>,
}

pub type LazyHookMemo = HookMemo<serde_cw_value::Value, serde_cw_value::Value>;

/// Message type for `sudo` entry_point
#[cfg(feature = "cosmwasm")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}
