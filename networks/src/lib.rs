use std::fmt::Debug;

use chainsmith_primitives::{Address, Balance, BlockNumber, Result};

pub mod ethereum;
pub mod solana;
#[deprecated]
pub mod substrate;
pub mod sui;

/// All chain adapters are required to implement this `Network` trait.
pub trait Network {
	/// Network-specific configuration.
	type Config: Config;
	/// Implementation of the RPC Provider.
	type Provider: OnchainRpcProvider<Self::Config>;
}

/// Network-specific configuration.
pub trait Config {
	/// Account data.
	type AccountData;
	/// Block data.
	type BlockData;
	/// Parameter type for querying the block.
	type BlockQuery: Into<u64>;
	/// Transaction type.
	type Transaction: Clone + Debug + PartialEq;
	/// Parameter type for querying the transaction.
	type TransactionQuery;
}

#[allow(async_fn_in_trait)]
pub trait OnchainRpcProvider<T: Config> {
	async fn new(url: &str) -> Result<Self>
	where
		Self: Sized;

	/// Get account data by address. Returns `Config::AccountData`.
	async fn get_account(&self, address: Address) -> Result<Option<T::AccountData>>;

	/// Get latest block number. Returns `BlockNumber`.
	async fn get_block_number(&self) -> Result<BlockNumber>;

	/// Get block data by block number. Returns `Config::BlockData`.
	async fn get_block_by_number(&self, block_number: BlockNumber) -> Result<Option<T::BlockData>>;

	/// Get the balance of the account.
	async fn get_balance(&self, address: Address) -> Result<Option<Balance>>;

	/// Get the transaction by transaction hash or block hash. Returns `Config::Transaction`.
	async fn get_transaction(
		&self,
		signature: T::TransactionQuery,
	) -> Result<Option<T::Transaction>>;
}
