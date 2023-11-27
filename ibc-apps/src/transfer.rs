// We define the response as a prost message to be able to decode the protobuf data.
#[derive(Clone, PartialEq, Eq, prost::Message)]
pub struct MsgTransferResponse {
	#[prost(uint64, tag = "1")]
	pub sequence: u64,
}