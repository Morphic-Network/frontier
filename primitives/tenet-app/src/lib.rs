#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::{H256, U256, Bytes};
use sp_std::{vec::Vec};
use ethereum::{AccessList, TransactionV2 as Transaction, ReceiptV3 as Receipt, Log};
use rlp::{Encodable, RlpStream};
extern crate alloc;

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
impl TxInput {
    pub fn hash(&self) -> H256 {
      let encoded = rlp::encode(self);
		let mut out = alloc::vec![0; 1 + encoded.len()];
		out[0] = 2;
		out[1..].copy_from_slice(&encoded);
		// H256::from_slice(Keccak256::digest(&out).as_slice());
        H256::from(sp_io::hashing::keccak_256(&out))
    }
}
impl Encodable for TxInput {
	fn rlp_append(&self, s: &mut RlpStream) {
		s.begin_list(9)
			.append(&self.chain_id)
			.append(&self.nonce)
			.append(&self.max_priority_fee_per_gas)
			.append(&self.max_fee_per_gas)
			.append(&self.value)
			.append(&self.input.to_vec())
			.append_list(&self.access_list)
			.append(&U256::from_big_endian(&self.r[..]))
			.append(&U256::from_big_endian(&self.s[..]));
	}
}

pub struct TxOutput {
	pub status_code: u8,
	pub used_gas: U256,
	pub logs: Vec<Log>,
}
impl TxOutput {
    pub fn hash(&self) -> H256 {
      let encoded = rlp::encode(self);
		let mut out = alloc::vec![0; 1 + encoded.len()];
		out[0] = 2;
		out[1..].copy_from_slice(&encoded);
		// H256::from_slice(Keccak256::digest(&out).as_slice());
        H256::from(sp_io::hashing::keccak_256(&out))
    }
}
impl Encodable for TxOutput {
	fn rlp_append(&self, s: &mut RlpStream) {
		s.begin_list(2)
			.append(&self.status_code)
			.append(&self.used_gas);
			// .append(&self.logs); // TODO fix
	}
}
pub struct TenetApi{}
impl TenetApi {
	pub fn generate_input_hash(transaction: &Transaction) -> H256 {
		// let tx: TransactionData = transaction.into();
		// let tenet_app_input = match tx {
		// 	Transaction::Legacy(t) => TxInput {
		// 		chain_id: 0,
		// 		nonce: t.nonce,
		// 		max_priority_fee_per_gas: U256::zero(),
		// 		max_fee_per_gas: U256::zero(),
		// 		value: t.value,
		// 		input: sp_core::Bytes(t.input.clone()),
		// 		access_list: Vec::new(),
		// 		r: H256::zero(),
		// 		s: H256::zero(),
		// 	},
		// 	Transaction::EIP2930(t) => TxInput {
		// 		chain_id: t.chain_id,
		// 		nonce: t.nonce,
		// 		max_priority_fee_per_gas: U256::zero(),
		// 		max_fee_per_gas: U256::zero(),
		// 		value: t.value,
		// 		input: sp_core::Bytes(t.input.clone()),
		// 		access_list: t.access_list.clone(),
		// 		r: t.r,
		// 		s: t.s,
		// 	},
		// 	Transaction::EIP1559(t) => TxInput {
		// 		chain_id: t.chain_id,
		// 		nonce: t.nonce,
		// 		max_priority_fee_per_gas: t.max_priority_fee_per_gas,
		// 		max_fee_per_gas: t.max_fee_per_gas,
		// 		value: t.value,
		// 		input: sp_core::Bytes(t.input.clone()),
		// 		access_list: t.access_list.clone(),
		// 		r: t.r,
		// 		s: t.s,
		// 	},
		// };
        // tenet_app_input.hash()
		transaction.hash()
	}

    pub fn generate_output_hash(receipt: &Receipt) -> H256 {
        let tenet_app_output = match receipt {
			Receipt::Legacy(t) => TxOutput {
                status_code: t.status_code,
                used_gas: t.used_gas,
                logs: t.logs.clone(),
			},
			Receipt::EIP2930(t) => TxOutput {
                status_code: t.status_code,
                used_gas: t.used_gas,
                logs: t.logs.clone(),
			},
			Receipt::EIP1559(t) => TxOutput {
                status_code: t.status_code,
                used_gas: t.used_gas,
                logs: t.logs.clone(),
			},
		};
        tenet_app_output.hash()
    }
}
