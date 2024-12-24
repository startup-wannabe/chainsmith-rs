use chainsmith_primitives::BlockNumber;

/// Types for a mainnet-like Substrate-based network.
#[derive(Clone, Copy, Debug)]
pub struct Config {
	_private: (),
}

impl crate::Config for Config {
	type AccountData = ();
	type BlockData = ();
	type BlockQuery = BlockNumber;
	type Transaction = ();
	type TransactionQuery = String;
}
