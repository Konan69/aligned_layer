/// Batcher ///
pub const GAS_PRICE_INCREMENT_PERCENTAGE_PER_ITERATION: usize = 5;
pub const DEFAULT_AGGREGATOR_GAS_COST: u128 = 330_000;
pub const BATCHER_SUBMISSION_BASE_GAS_COST: u128 = 125_000;
pub const ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF: u128 = 2_000;
pub const DEFAULT_CONSTANT_GAS_COST: u128 = ((DEFAULT_AGGREGATOR_GAS_COST
    * DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER)
    / PERCENTAGE_DIVIDER)
    + BATCHER_SUBMISSION_BASE_GAS_COST;
pub const DEFAULT_MAX_FEE_PER_PROOF: u128 =
    ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * 100_000_000_000; // gas_price = 100 Gwei = 0.0000001 ether (high gas price)
pub const CONNECTION_TIMEOUT: u64 = 30; // 30 secs

// According to:
// - https://www.rfc-editor.org/rfc/rfc8949.html#section-3.1-2.10
// - https://www.rfc-editor.org/rfc/rfc8949.html#section-3-3.2
// - https://www.rfc-editor.org/rfc/rfc8949.html#section-3-3.4
// 9 bytes are the maximum overhead from aggregating data into an array in CBOR
// (it may be as little as just 1 byte, but it depends on the number of elements
// and serialization parameters).
pub const CBOR_ARRAY_MAX_OVERHEAD: usize = 9;

// % modifiers: (100% is x1, 10% is x0.1, 1000% is x10)
pub const RESPOND_TO_TASK_FEE_LIMIT_PERCENTAGE_MULTIPLIER: u128 = 250; // fee_for_aggregator -> respondToTaskFeeLimit modifier
pub const DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER: u128 = 125; // feeForAggregator modifier
pub const GAS_PRICE_PERCENTAGE_MULTIPLIER: u128 = 110; // gasPrice modifier
pub const OVERRIDE_GAS_PRICE_PERCENTAGE_MULTIPLIER: u128 = 120; // gasPrice modifier to override previous transactions
pub const PERCENTAGE_DIVIDER: u128 = 100;

/// SDK ///
/// Number of proofs we a batch for estimation.
/// This is the number of proofs in a batch of size n, where we set n = 32.
/// i.e. the user pays for the entire batch and his proof is instantly submitted.
pub const MAX_FEE_BATCH_PROOF_NUMBER: usize = 32;
/// Estimated number of proofs for batch submission.
/// This corresponds to the number of proofs to compute for a default max_fee.
pub const MAX_FEE_DEFAULT_PROOF_NUMBER: usize = 10;

/// Ethereum calls retry constants
pub const ETHEREUM_CALL_MIN_RETRY_DELAY: u64 = 500; // milliseconds
pub const ETHEREUM_CALL_MAX_RETRIES: usize = 5;
pub const ETHEREUM_CALL_BACKOFF_FACTOR: f32 = 2.0;
pub const ETHEREUM_CALL_MAX_RETRY_DELAY: u64 = 3600; // seconds

/// Ethereum transaction retry constants
pub const BUMP_MIN_RETRY_DELAY: u64 = 500; // milliseconds
pub const BUMP_MAX_RETRIES: usize = 33; // ~ 1 day
pub const BUMP_BACKOFF_FACTOR: f32 = 2.0;
pub const BUMP_MAX_RETRY_DELAY: u64 = 3600; // seconds
