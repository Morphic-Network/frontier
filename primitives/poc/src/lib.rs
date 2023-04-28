//#![cfg_attr(not(feature = "std"), no_std)]

use ethereum_types::{H160, H256};
use sha3::{Digest, Keccak256};
use libsecp256k1::Signature;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoC {
	io_hash_list: Vec<IOHash>,
	pub sign: Vec<u8>,
	tee_node_address: H160,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IOHash {
	pub input_hash: H256,
	pub output_hash: H256,
}

pub fn generate_poc(private_key: H256, chain_id: u64, io_list: &Vec<IOHash>) -> PoC {
	let raw = mix_io_hash(io_list);

	let s = libsecp256k1::sign(
		&libsecp256k1::Message::parse(H256::from_slice(Keccak256::digest(raw).as_slice()).as_fixed_bytes()),
		&libsecp256k1::SecretKey::parse_slice(&private_key[..]).unwrap(),
	);

	let secret_key = libsecp256k1::SecretKey::parse_slice(&private_key[..]).unwrap();
	let public_key = &libsecp256k1::PublicKey::from_secret_key(&secret_key).serialize()[1..65];
	let address = H160::from(H256::from_slice(sha3::Keccak256::digest(public_key).as_slice()));
	PoC {
		io_hash_list: io_list.clone(),
		sign: Vec::from(s.0.serialize()),
		tee_node_address: address,
	}
}

pub fn verify_poc(poc: &PoC, public_key: &[u8]) -> bool {
	let raw = mix_io_hash(&poc.io_hash_list);
	let mut sign: [u8; 64] = [0; 64];
	sign.copy_from_slice(&poc.sign);
	libsecp256k1::verify(
		&libsecp256k1::Message::parse(H256::from_slice(Keccak256::digest(raw).as_slice()).as_fixed_bytes()),
		&Signature::parse_standard(&sign).unwrap(),
		&libsecp256k1::PublicKey::parse_slice(public_key, Some(libsecp256k1::PublicKeyFormat::Full)).unwrap()
	)
}

fn mix_io_hash(io_list: &Vec<IOHash>) -> String {
	let mut hash_queue = Vec::new();
	for io in io_list {
		hash_queue.push(H256::from_slice(Keccak256::digest(&format!("{}{}", io.input_hash, io.output_hash)).as_slice()));
	}
	let mut raw = String::from("");
	for hash in &hash_queue {
		raw.push_str(&hash.to_string());
	}
	raw
}