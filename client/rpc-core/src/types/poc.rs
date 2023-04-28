use ethereum_types::{H256, H160};
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct PoC {
	pub test: bool
}

// pub struct PoC {
// 	io_hash_list: Vec<IOHash>,
// 	pub sign: Vec<u8>,
// 	tee_node_address: H160,
// }

// pub struct IOHash {
// 	pub input_hash: H256,
// 	pub output_hash: H256,
// }