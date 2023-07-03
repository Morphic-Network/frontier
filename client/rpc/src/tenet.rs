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

use std::{sync::Arc, marker::PhantomData};

use fc_storage::OverrideHandle;
use jsonrpsee::core::RpcResult as Result;
use sc_client_api::{StorageProvider, Backend};
// Substrate
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
// Frontier
use fp_rpc::EthereumRuntimeRPCApi;

use ethereum_types::H256;

use crate::{internal_err, frontier_backend_client, EthBlockDataCacheTask};

/// Tenet API implementation.
pub struct Tenet<B: BlockT, C, BE> {
	client: Arc<C>,
	overrides: Arc<OverrideHandle<B>>,
	backend: Arc<fc_db::Backend<B>>,
	_marker: PhantomData<BE>,
}

impl<B: BlockT, C, BE> Tenet<B, C, BE> {
	pub fn new(
		client: Arc<C>,
		overrides: Arc<OverrideHandle<B>>,
		backend: Arc<fc_db::Backend<B>>,
	) -> Self {
		Self {
			client,
			overrides,
			backend,
			_marker: PhantomData,
		}
	}
}

impl<B, C, BE> Tenet<B, C, BE>
where
	B: BlockT,
	C: ProvideRuntimeApi<B>,
	C::Api: EthereumRuntimeRPCApi<B>,
	C: HeaderBackend<B> + StorageProvider<B, BE> + 'static,
	BE: Backend<B> + 'static,
{
	pub async fn get_poc(&self, tx_id: H256) -> Result<Option<Vec<u8>>> {
		let client = Arc::clone(&self.client);
		let overrides = Arc::clone(&self.overrides);
		let backend = Arc::clone(&self.backend);

		let (hash, index) = match frontier_backend_client::load_transactions::<B, C>(
			client.as_ref(),
			backend.as_ref(),
			tx_id,
			true,
		)
		.map_err(|err| internal_err(format!("{:?}", err)))?
		{
			Some((hash, index)) => (hash, index as usize),
			None => return Ok(None),
		};

		let substrate_hash = match frontier_backend_client::load_hash::<B, C>(
			client.as_ref(),
			backend.as_ref(),
			hash,
		)
		.map_err(|err| internal_err(format!("{:?}", err)))?
		{
			Some(hash) => hash,
			_ => return Ok(None),
		};

		let schema = fc_storage::onchain_storage_schema(client.as_ref(), substrate_hash);
		let handler = overrides
			.schemas
			.get(&schema)
			.unwrap_or(&overrides.fallback);

	 Ok(handler.current_pocs(substrate_hash, tx_id))
	}
}
