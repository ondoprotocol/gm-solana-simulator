//! # Ondo GM Simulator
//!
//! A Rust library for handling Ondo Global Markets (GM) JIT trades via Jupiter RFQ
//! in wallet transaction simulation.
//!
//! ## Problem
//!
//! Ondo GM tokens are minted just-in-time (JIT) when a swap occurs. This means
//! standard transaction simulation fails because the solver doesn't have the GM
//! tokens until the mint instruction executes in the Jito bundle.
//!
//! ## Solution
//!
//! This crate provides functionality to:
//! 1. **Detect** if a transaction is a GM trade that needs special handling
//! 2. **Generate** a mock mint transaction that can be simulated as a bundle
//!
//! By simulating the mock mint transaction first, followed by the Jupiter fill
//! transaction, the simulation succeeds and shows accurate balance changes.
//!
//! ## Usage
//!
//! ```ignore
//! use ondo_gm_simulator::{check_gm_trade, build_mock_mint_transaction};
//! use solana_sdk::transaction::Transaction;
//!
//! fn simulate_transaction(tx: &Transaction, recent_blockhash: Hash) -> SimulationResult {
//!     // Check if this is a GM trade
//!     match check_gm_trade(tx) {
//!         Ok(result) if result.use_gm_bundle_sim => {
//!             let trade_info = result.trade_info.unwrap();
//!             
//!             // Build mock mint transaction
//!             let mock_mint_tx = build_mock_mint_transaction(&trade_info, recent_blockhash);
//!             
//!             // Simulate as bundle: [mock_mint_tx, original_tx]
//!             simulate_bundle(vec![mock_mint_tx, tx.clone()])
//!         }
//!         Ok(_) => {
//!             // Not a GM trade, use normal simulation
//!             simulate_single(tx)
//!         }
//!         Err(GmSimulatorError::UnauthorizedMaker(maker)) => {
//!             // GM token but unauthorized maker - reject or warn
//!             SimulationResult::error(format!("Unauthorized maker: {}", maker))
//!         }
//!         Err(e) => {
//!             // Other error, fall back to normal simulation
//!             simulate_single(tx)
//!         }
//!     }
//! }
//! ```
//!
//! ## Detection Criteria
//!
//! A transaction is considered a GM trade if it contains a **Jupiter Order Engine fill**
//! instruction where:
//! 1. The **maker** is an authorized Ondo GM solver
//! 2. The **taker receives** a GM token (output_mint is a GM token)
//!
//! The transaction may contain other instructions (e.g., ATA creates) - we search
//! through all instructions to find the Jupiter fill.
//!
//! ## Authorized Solvers
//!
//! Only these three solver addresses are authorized:
//! - `DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds`
//! - `2Cq2RNFFxxPXL7teNQAji1beA2vFbBDYW5BGPBFvoN9m`
//! - `9BB7Tt5uE5VdRsxA5XRqrjwNaq8XtgAUQW8czA6ymUPG`
//!
//! ## Simulation Minter
//!
//! The mock mint uses the admin minter account for simulation:
//! - `4pfyfezvwjBrsHtJpXPPKsqH9cphwSDDb7s63KzkVEqF`
//!
//! This account doesn't require attestations, making it suitable for simulation.
//!
//! ## Important Notes
//!
//! - The `mint_gm` instruction discriminator and account layout should be verified
//!   against the actual on-chain IDL at program `XzTT4XB8m7sLD2xi6snefSasaswsKCxx5Tifjondogm`
//! - GM tokens use Token-2022 (not SPL Token)
//! - All GM tokens have 9 decimal places

pub mod constants;
pub mod discriminator;
pub mod mint_instruction;
pub mod parser;
pub mod simulator;
pub mod types;

// Re-export main public API
pub use constants::{
    get_gm_token_symbol, is_authorized_solver, is_gm_token, jupiter_order_engine_program_id,
    ondo_gm_program_id, admin_minter, token_2022_program_id, usdc_mint,
    AUTHORIZED_SOLVERS, GM_TOKENS, JUPITER_ORDER_ENGINE_PROGRAM_ID, ONDO_GM_PROGRAM_ID,
    ADMIN_MINTER, TOKEN_2022_PROGRAM_ID, USDC_MINT,
};
pub use discriminator::instruction_discriminator;
pub use mint_instruction::{
    build_mock_mint_gm_instruction, build_mock_mint_gm_instruction_with_ata, get_gm_token_ata,
};
pub use simulator::{
    build_mock_mint_instruction, build_mock_mint_instruction_to_ata, build_mock_mint_transaction,
    check_gm_trade, check_gm_trade_message, check_gm_trade_versioned,
    check_gm_trade_versioned_message, maybe_build_mock_mint,
};
pub use types::{GmCheckResult, GmSimulatorError, GmTradeInfo};
