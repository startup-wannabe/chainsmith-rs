use chainsmith_networks::{sui::Config, Config as NetworkConfig, Network, OnchainRpcProvider};
use chainsmith_primitives::{Address, Balance, BlockNumber, Result, Uint};
use eyre::WrapErr;
use sui_sdk::{SuiClient, SuiClientBuilder};
use tracing::info;

/// Configuration for the Substrate-based network.
pub struct Sui;

impl Network for Sui {
	type Config = Config;
	type Provider = SuiRpcProvider;
}

/// RPC Provider for the Sui network.
pub struct SuiRpcProvider {
	inner: SuiClient,
}

impl SuiRpcProvider {
	pub async fn get_latest_storage(&self) -> Result<SuiClient> {
		self.inner.read_api().get_
	}

	pub async fn query_storage(
		&self,
		pallet_name: &str,
		entry_name: &str,
		params: Vec<Value>,
	) -> Result<DecodedValue> {
		let query = subxt::dynamic::storage(pallet_name, entry_name, params);
		let latest_storage = self.get_latest_storage().await?;
		let result = latest_storage.fetch(&query).await?;
		result.unwrap().to_value().wrap_err("Failed to decode value")
	}
}

impl OnchainRpcProvider<Config> for SuiRpcProvider {
	async fn new(url: &str) -> Result<Self> {
		let client = SuiClientBuilder::default().build(&self).await?;
		Ok(Self { inner: client })
	}

	async fn get_block_number(&self) -> Result<u64> {
		info!(method = "get_block_number");
		let block_number = self.inner.blocks().at_latest().await?.number();
		Ok(block_number.into())
	}

	async fn get_balance(&self, address: Address) -> Result<Option<Balance>> {
		info!(method = "get_balance");
		let value = self
			.query_storage("System", "Account", vec![Value::from_bytes(address.as_bytes())])
			.await?;
		let account_data = value.at("data");
		Ok(account_data
			.at("free")
			.map(|b| {
				let balance = b.as_u128().unwrap();
				Some(Uint::from(balance))
			})
			.unwrap_or(None))
	}

	async fn get_transaction(
		&self,
		_: <Config as NetworkConfig>::TransactionQuery,
	) -> Result<Option<<Config as NetworkConfig>::Transaction>> {
		unimplemented!()
	}

	async fn get_account(
		&self,
		_address: Address,
	) -> Result<Option<<Config as NetworkConfig>::AccountData>> {
		unimplemented!()
	}

	async fn get_block_by_number(
		&self,
		_block_number: BlockNumber,
	) -> Result<Option<<Config as NetworkConfig>::BlockData>> {
		unimplemented!()
	}
}
