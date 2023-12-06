//! most common memo with PFM

use ibc_core_host_types::identifiers::{ChannelId, PortId};

use crate::types::hook::Callback;
use serde::{Deserialize, Serialize};

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
pub struct Memo<Next, Msg> {
    /// memo has at least one key, with value "wasm", than wasm hooks will try to execute it
    /// CosmWasm must be enabled on receiver chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasm: Option<Callback<Msg>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<ForwardingMemo<Next>>,
}

impl<Next, Msg> Memo<Next, Msg> {
    pub fn forward(forward: ForwardingMemo<Next>) -> Self {
        Self {
            forward: Some(forward),
            wasm: None,
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
pub struct ForwardingMemoBase {
    pub receiver: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<PortId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u8>,
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
pub struct ForwardingMemo<Next> {
    #[serde(flatten)]
    pub base: ForwardingMemoBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<Box<Next>>,
}

/// Does not types `next`, so only validates it is JSON
pub type JsonForwardingMemo = ForwardingMemo<serde_cw_value::Value>;

pub type ExactEagerForwardingMemo = ForwardingMemo<ExactForwardingMemo>;

/// just duplicate as type cannot use Self in type alias
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
pub struct ExactForwardingMemo {
    #[serde(flatten)]
    pub base: ForwardingMemoBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<Box<Self>>,
}

impl<Next> ForwardingMemo<Next> {
    pub fn new_ibc_memo(
        receiver: String,
        port: PortId,
        channel: ChannelId,
        timeout: u64,
        retries: u8,
    ) -> Self {
        Self {
            base: ForwardingMemoBase {
                receiver,
                port: Some(port),
                channel: Some(channel),
                timeout: Some(timeout),
                retries: Some(retries),
            },
            next: None,
        }
    }
}
