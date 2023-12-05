//! most common memo with PFM

use ibc_core_host_types::identifiers::{ChannelId, PortId};

use crate::types::hook::Callback;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
// Encode, Decode, scale_info::TypeInfo, to be manually implemented for subset of know messages
pub struct Memo {
    /// memo has at least one key, with value "wasm", than wasm hooks will try to execute it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasm: Option<Callback>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<ForwardingMemo>,
}

impl Memo {
    pub fn forward(forward: ForwardingMemo) -> Self {
        Self {
            forward: Some(forward),
            wasm: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
// Encode, Decode, scale_info::TypeInfo, to be manually implemented for subset of know messages
pub struct ForwardingMemo {
    pub receiver: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<PortId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<Box<Memo>>,
}

impl ForwardingMemo {
    pub fn new_ibc_memo(
        receiver: String,
        port: PortId,
        channel: ChannelId,
        timeout: u64,
        retries: u8,
    ) -> Self {
        Self {
            receiver,
            port: Some(port),
            channel: Some(channel),
            timeout: Some(timeout),
            retries: Some(retries),
            next: None,
        }
    }
}
