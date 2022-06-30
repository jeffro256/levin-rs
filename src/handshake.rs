use serde::{self, Serialize, Deserialize};

use crate::bucket::Bucket;
use crate::constants;
use crate::error::Result;

pub fn create_handshake_request_bucket(my_port: u32, peer_id: u64) -> Result<Bucket<HandshakeRequestPayload>> {
	let handshake_payload = HandshakeRequestPayload::new(my_port, peer_id);
	Bucket::new_request(constants::LEVIN_COMMAND_HANDSHAKE, handshake_payload)
}

#[derive(Deserialize, Serialize, Debug)]
struct HandshakeNodeData {
	local_time: u64,
	my_port: u32,
	#[serde(with = "serde_bytes")]
	network_id: Vec<u8>,
	peer_id: u64
}

impl HandshakeNodeData {
	fn unix_now() -> u64 {
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
	}

	fn new(my_port: u32, peer_id: u64) -> Self {
		Self {
			local_time: Self::unix_now(),
			my_port: my_port,
			network_id: Vec::<u8>::from(constants::CRYPTONOTE_STAGENET_NETWORK_ID),
			peer_id: peer_id
		}
	}
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
// #[serde(with = "serde_bytes")]
struct HandshakePayloadData {
	cumulative_difficulty: u64,
	current_height: u64,
	#[serde(with = "serde_bytes")]
	top_id: Vec<u8>,
	top_version: u8
}

impl HandshakePayloadData {
	fn genesis_payload() -> Self {
		Self {
			cumulative_difficulty: 1,
			current_height: 1,
			top_id: constants::MONERO_GENESIS_HASH.into(),
			top_version: 1
		}
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HandshakeRequestPayload {
	node_data: HandshakeNodeData,
	payload_data: HandshakePayloadData
}

impl HandshakeRequestPayload {
	pub fn new(my_port: u32, peer_id: u64) -> Self {
		Self {
			node_data: HandshakeNodeData::new(my_port, peer_id),
			payload_data: HandshakePayloadData::genesis_payload()
		}
	}
}
