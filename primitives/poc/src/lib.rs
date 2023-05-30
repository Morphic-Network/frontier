#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::{vec::Vec, collections::vec_deque::VecDeque};
use sp_core::{H256, ecdsa::Signature};
use rlp::{Encodable, RlpStream};

#[derive(Clone)]
pub struct PoC {
	io_hash_list: Vec<IOHash>,
	pub sign: Signature,
}
impl Encodable for PoC {
	fn rlp_append(&self, s: &mut RlpStream) {
		// TODO
	}
}


#[derive(Clone)]
pub struct IOHash {
	pub input_hash: H256,
	pub output_hash: H256,
}

pub fn generate_poc(private_key: H256, chain_id: u64, io_list: &Vec<IOHash>) -> PoC {
	let root = generate_root(io_list);
	PoC{
		io_hash_list: io_list.clone(), 
		sign: sp_io::crypto::secp256k1_ecdsa_sign(private_key.as_fixed_bytes(), root.as_fixed_bytes()).unwrap()
	}
}

fn generate_root(io_list: &Vec<IOHash>) -> H256 {
	let mut hash_queue = VecDeque::new();
	for io in io_list {
		hash_queue.push_back(io.input_hash);
		hash_queue.push_back(io.output_hash);
	}
	while hash_queue.len() > 1 {
		let hash1 = hash_queue.pop_front().unwrap();
		let hash2 = hash_queue.pop_front().unwrap();
		let combined_hash: Vec<u8> = [&hash1.to_fixed_bytes().to_vec()[..], &hash2.to_fixed_bytes().to_vec()[..]].concat();
		hash_queue.push_back(H256::from(sp_io::hashing::keccak_256(&combined_hash)));
	}
	hash_queue.pop_front().unwrap()
}