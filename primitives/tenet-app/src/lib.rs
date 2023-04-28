//#![cfg_attr(not(feature = "std"), no_std)]

use ethereum_types::{Bloom, BloomInput, H160, H256, H64, U256};
pub use ethereum::{
	AccessListItem, AccessList,
	BlockV2 as Block, LegacyTransactionMessage, Log, ReceiptV3 as Receipt,
	TransactionAction, TransactionV2 as Transaction,
};
use rlp::{DecoderError, Encodable, Rlp, RlpStream};
use sha3::{Digest, Keccak256};

extern crate alloc;
type Bytes = alloc::vec::Vec<u8>;

pub struct TxInput {
	chain_id: u64,
	nonce: U256,
	max_priority_fee_per_gas: U256,
	max_fee_per_gas: U256,
	value: U256,
	input: Bytes,
	access_list: AccessList,
	r: H256,
	s: H256,
}
impl Encodable for TxInput {
	fn rlp_append(&self, s: &mut RlpStream) {
		s.begin_list(9)
			.append(&self.chain_id)
			.append(&self.nonce)
			.append(&self.max_priority_fee_per_gas)
			.append(&self.max_fee_per_gas)
			.append(&self.value)
			.append(&self.input)
			.append_list(&self.access_list)
			.append(&U256::from_big_endian(&self.r[..]))
			.append(&U256::from_big_endian(&self.s[..]));
	}
}

pub struct TenetApp{}
impl TenetApp {
	pub fn generate_input_hash(transaction: &Transaction) -> H256 {
		let tenet_app_intput = match transaction {
			Transaction::Legacy(t) => TxInput {
				chain_id: 0,
				nonce: t.nonce,
				max_priority_fee_per_gas: U256::zero(),
				max_fee_per_gas: U256::zero(),
				value: t.value,
				input: t.input.clone(),
				access_list: Vec::new(),
				r: H256::zero(),
				s: H256::zero(),
			},
			Transaction::EIP2930(t) => TxInput {
				chain_id: t.chain_id,
				nonce: t.nonce,
				max_priority_fee_per_gas: U256::zero(),
				max_fee_per_gas: U256::zero(),
				value: t.value,
				input: t.input.clone(),
				access_list: t.access_list.clone(),
				r: t.r,
				s: t.s,
			},
			Transaction::EIP1559(t) => TxInput {
				chain_id: t.chain_id,
				nonce: t.nonce,
				max_priority_fee_per_gas: t.max_priority_fee_per_gas,
				max_fee_per_gas: t.max_fee_per_gas,
				value: t.value,
				input: t.input.clone(),
				access_list: t.access_list.clone(),
				r: t.r,
				s: t.s,
			},
		};
		let mut rlp_stream = RlpStream::new();
		rlp_stream.append(&tenet_app_intput);
		let encoded = rlp_stream.out();
		H256::from_slice(Keccak256::digest(&encoded).as_slice())
	}
}