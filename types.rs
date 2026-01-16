//! Data types for the Ondo GM transaction simulator.

use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

/// Error types for the GM simulator
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GmSimulatorError {
    #[error("Transaction does not contain Jupiter Order Engine program")]
    NotJupiterRfq,

    #[error("Transaction must contain a Jupiter RFQ fill instruction")]
    NotJupiterFill,

    #[error("Taker is not receiving any Ondo GM tokens")]
    TakerNotReceivingGmToken,

    #[error("Maker address {0} is not an authorized Ondo GM solver")]
    UnauthorizedMaker(Pubkey),

    #[error("Failed to parse Jupiter fill instruction: {0}")]
    InstructionParseError(String),

    #[error("Invalid account index in instruction")]
    InvalidAccountIndex,

    #[error("Missing required account in transaction")]
    MissingAccount,

    #[error("Transaction has no instructions")]
    EmptyTransaction,
}

/// Information extracted from a Jupiter RFQ fill instruction
#[derive(Debug, Clone)]
pub struct GmTradeInfo {
    /// The maker (market maker/solver) pubkey
    pub maker: Pubkey,
    /// The taker (user) pubkey
    pub taker: Pubkey,
    /// The GM token mint that the taker is receiving
    pub gm_token_mint: Pubkey,
    /// The GM token symbol
    pub gm_token_symbol: String,
    /// Amount of GM tokens the taker will receive (in base units, 9 decimals)
    pub gm_token_amount: u64,
    /// Maker's output token account (where tokens come from)
    pub maker_output_account: Pubkey,
    /// Unix timestamp when the quote expires
    pub expire_at: i64,
}

/// Result of checking whether a transaction is a GM trade
#[derive(Debug, Clone)]
pub struct GmCheckResult {
    /// Whether this transaction should use GM bundle simulation
    pub use_gm_bundle_sim: bool,
    /// Trade info if this is a GM trade
    pub trade_info: Option<GmTradeInfo>,
}

impl GmCheckResult {
    /// Create a result indicating this is not a GM trade
    pub fn not_gm_trade() -> Self {
        Self {
            use_gm_bundle_sim: false,
            trade_info: None,
        }
    }

    /// Create a result indicating this is a GM trade
    pub fn gm_trade(info: GmTradeInfo) -> Self {
        Self {
            use_gm_bundle_sim: true,
            trade_info: Some(info),
        }
    }
}
