// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::sync::Arc;

use jsonrpsee::core::RpcResult as Result;
// Substrate
use sc_network::NetworkService;
use sc_network_common::{service::NetworkPeers, ExHashT};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
// Frontier
use fc_rpc_core::{types::PoC, TenetApiServer};
use fp_rpc::EthereumRuntimeRPCApi;

use ethereum_types::H256;

use crate::internal_err;

/// Tenet API implementation.
pub struct Tenet<B: BlockT, C, H: ExHashT> {
	client: Arc<C>,
	network: Arc<NetworkService<B, H>>,
}

impl<B: BlockT, C, H: ExHashT> Tenet<B, C, H> {
	pub fn new(
		client: Arc<C>,
		network: Arc<NetworkService<B, H>>,
	) -> Self {
		Self {
			client,
			network,
		}
	}
}

impl<B, C, H: ExHashT> TenetApiServer for Tenet<B, C, H>
where
	B: BlockT,
	C: ProvideRuntimeApi<B>,
	C::Api: EthereumRuntimeRPCApi<B>,
	C: HeaderBackend<B> + 'static,
{
	fn get_poc(&self, tx_id: H256) -> Result<PoC> {
		// TransactionPoc::<T>::get(transaction.hash());
		Ok(PoC{test: true})
	}
}
