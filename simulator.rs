//! Core simulation logic for Ondo GM JIT trades.
//!
//! This module provides the main entry points for:
//! 1. Checking if a transaction is a GM trade that needs bundle simulation
//! 2. Building mock mint transactions for bundle simulation

use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    message::{Message, VersionedMessage},
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    constants::{is_portfolio_token, jupiter_order_engine_program_id},
    mint_instruction::{
        build_mock_mint_gm_instruction, build_mock_mint_gm_instruction_with_ata,
        build_mock_mint_portfolio_instruction, build_mock_mint_portfolio_instruction_with_ata,
        build_upgraded_mock_mint_gm_instruction,
    },
    types::MintLayout,
    parser::{is_jupiter_fill_instruction, parse_fill_for_gm_trade},
    types::{GmCheckResult, GmSimulatorError, GmTradeInfo},
};

/// Check if a transaction should use GM bundle simulation.
///
/// A transaction qualifies for GM bundle simulation if:
/// 1. It has exactly one instruction
/// 2. That instruction is a Jupiter Order Engine fill
/// 3. The maker is one of the 3 authorized Ondo GM solvers
/// 4. The taker is receiving a GM token (output_mint is a GM token)
///
/// # Arguments
///
/// * `transaction` - The transaction to check
///
/// # Returns
///
/// * `Ok(GmCheckResult)` with `use_gm_bundle_sim = true` and trade info if this is a GM trade
/// * `Ok(GmCheckResult)` with `use_gm_bundle_sim = false` if not a GM trade
/// * `Err` if the transaction is malformed or has an unauthorized maker
///
/// # Example
///
/// ```ignore
/// use ondo_gm_simulator::check_gm_trade;
///
/// let result = check_gm_trade(&transaction)?;
/// if result.use_gm_bundle_sim {
///     let info = result.trade_info.unwrap();
///     println!("GM trade: {} {} to {}", info.gm_token_amount, info.gm_token_symbol, info.taker);
/// }
/// ```
pub fn check_gm_trade(transaction: &Transaction) -> Result<GmCheckResult, GmSimulatorError> {
    check_gm_trade_message(&transaction.message)
}

/// Check if a message should use GM bundle simulation.
///
/// Same as `check_gm_trade` but operates on a `Message` instead of `Transaction`.
///
/// Note: GM trades typically include additional instructions like `createAssociatedTokenAccountIdempotent`
/// to ensure the taker's ATA exists. We search for the Jupiter fill instruction among all instructions.
pub fn check_gm_trade_message(message: &Message) -> Result<GmCheckResult, GmSimulatorError> {
    let account_keys = &message.account_keys;
    let jupiter_program_id = jupiter_order_engine_program_id();

    // Check 1: Must have at least one instruction
    if message.instructions.is_empty() {
        return Err(GmSimulatorError::EmptyTransaction);
    }

    // Check 2: Find Jupiter Order Engine fill instruction
    // Note: Transaction may contain other instructions like createAssociatedTokenAccountIdempotent
    let fill_instruction = message
        .instructions
        .iter()
        .find(|ix| is_jupiter_fill_instruction(ix, &jupiter_program_id, account_keys));

    let Some(instruction) = fill_instruction else {
        return Ok(GmCheckResult::not_gm_trade());
    };

    // Check 3 & 4: Parse and validate (maker must be authorized, output must be GM token)
    match parse_fill_for_gm_trade(instruction, account_keys)? {
        Some(trade_info) => Ok(GmCheckResult::gm_trade(trade_info)),
        None => Ok(GmCheckResult::not_gm_trade()),
    }
}

/// Check if a versioned transaction should use GM bundle simulation.
///
/// This function supports both legacy and v0 transactions. For v0 transactions
/// with address lookup tables, only the static account keys are checked.
///
/// # Arguments
///
/// * `transaction` - The versioned transaction to check
///
/// # Returns
///
/// * `Ok(GmCheckResult)` with `use_gm_bundle_sim = true` and trade info if this is a GM trade
/// * `Ok(GmCheckResult)` with `use_gm_bundle_sim = false` if not a GM trade
/// * `Err` if the transaction is malformed or has an unauthorized maker
pub fn check_gm_trade_versioned(
    transaction: &VersionedTransaction,
) -> Result<GmCheckResult, GmSimulatorError> {
    check_gm_trade_versioned_message(&transaction.message)
}

/// Check if a versioned message should use GM bundle simulation.
///
/// Same as `check_gm_trade_versioned` but operates on a `VersionedMessage` instead of `VersionedTransaction`.
///
/// Note: For V0 messages with address lookup tables, this function only checks the static
/// account keys. If the Jupiter fill instruction references accounts from lookup tables,
/// the check may not work correctly. In practice, the critical accounts (taker, maker,
/// output_mint) are typically in the static keys.
pub fn check_gm_trade_versioned_message(
    message: &VersionedMessage,
) -> Result<GmCheckResult, GmSimulatorError> {
    match message {
        VersionedMessage::Legacy(legacy_msg) => check_gm_trade_message(legacy_msg),
        VersionedMessage::V0(v0_msg) => {
            let account_keys = &v0_msg.account_keys;
            let jupiter_program_id = jupiter_order_engine_program_id();

            // Check 1: Must have at least one instruction
            if v0_msg.instructions.is_empty() {
                return Err(GmSimulatorError::EmptyTransaction);
            }

            // Check 2: Find Jupiter Order Engine fill instruction
            let fill_instruction = v0_msg
                .instructions
                .iter()
                .find(|ix| is_jupiter_fill_instruction(ix, &jupiter_program_id, account_keys));

            let Some(instruction) = fill_instruction else {
                return Ok(GmCheckResult::not_gm_trade());
            };

            // Check 3 & 4: Parse and validate (maker must be authorized, output must be GM token)
            match parse_fill_for_gm_trade(instruction, account_keys)? {
                Some(trade_info) => Ok(GmCheckResult::gm_trade(trade_info)),
                None => Ok(GmCheckResult::not_gm_trade()),
            }
        }
    }
}

/// Build a mock mint transaction for bundle simulation.
///
/// Given GM trade info, this builds an unsigned transaction containing:
/// 1. A `createAssociatedTokenAccountIdempotent` instruction to ensure the taker's ATA exists
/// 2. A `mint_gm` instruction that will mint the required GM tokens to the solver's token account
///
/// This matches the pattern used by actual solver transactions.
///
/// # Arguments
///
/// * `trade_info` - The GM trade info from `check_gm_trade`
/// * `_recent_blockhash` - A recent blockhash for the transaction (currently unused)
///
/// # Returns
///
/// An unsigned `Transaction` that can be simulated in a bundle with the
/// original Jupiter fill transaction.
///
/// # Simulation Order
///
/// When simulating as a bundle:
/// 1. First: The mock mint transaction (creates taker ATA if needed + mints GM tokens to solver)
/// 2. Second: The original Jupiter fill transaction (creates taker ATA if needed + swaps tokens with user)
///
/// # Example
///
/// ```ignore
/// use ondo_gm_simulator::{check_gm_trade, build_mock_mint_transaction};
///
/// let result = check_gm_trade(&fill_transaction)?;
/// if result.use_gm_bundle_sim {
///     let trade_info = result.trade_info.unwrap();
///     let mock_mint_tx = build_mock_mint_transaction(&trade_info, recent_blockhash);
///
///     // Simulate as bundle: [mock_mint_tx, fill_transaction]
///     let bundle = vec![mock_mint_tx, fill_transaction];
///     simulate_bundle(&bundle)?;
/// }
/// ```
pub fn build_mock_mint_transaction(
    trade_info: &GmTradeInfo,
    recent_blockhash: Hash,
) -> Transaction {
    build_mock_mint_transaction_with_layout(trade_info, recent_blockhash, MintLayout::Legacy)
}

/// Build the mock mint transaction for a specific GM program `mint_gm` layout.
///
/// Use this when simulating the bundle on your own nodes across the GM program
/// upgrade: build with `MintLayout::Legacy` (what `build_mock_mint_transaction`
/// does) and, if the mock mint transaction fails, rebuild with
/// `MintLayout::Upgraded` and simulate once more. `simulate_as_bundle` does
/// this for you.
///
/// Portfolio tokens only exist on the upgraded program, so they always get the
/// same `mint_portfolio` transaction whichever layout is requested.
///
/// # Example
///
/// ```ignore
/// use gm_solana_simulator::{build_mock_mint_transaction_with_layout, MintLayout};
///
/// let mock_mint = build_mock_mint_transaction_with_layout(&trade_info, blockhash, MintLayout::Legacy);
/// let mut result = simulate_bundle(&[mock_mint, fill_transaction.clone()])?;
///
/// // Transaction 0 (the mock mint) failed: the other program version is live
/// if result.transaction_results[0].err.is_some() {
///     let mock_mint = build_mock_mint_transaction_with_layout(&trade_info, blockhash, MintLayout::Upgraded);
///     result = simulate_bundle(&[mock_mint, fill_transaction])?;
/// }
/// ```
pub fn build_mock_mint_transaction_with_layout(
    trade_info: &GmTradeInfo,
    recent_blockhash: Hash,
    layout: MintLayout,
) -> Transaction {
    use spl_associated_token_account::instruction::create_associated_token_account_idempotent;

    // Pick the mint instruction the targeted program version accepts. Portfolio
    // tokens fall through to `build_mock_mint_instruction`, which always builds
    // `mint_portfolio`.
    let mint_ix = match layout {
        MintLayout::Upgraded if !is_portfolio_token(&trade_info.gm_token_mint) => {
            build_upgraded_mock_mint_gm_instruction(
                &trade_info.gm_token_mint,
                &trade_info.maker, // Mint to the solver (maker)
                trade_info.gm_token_amount,
            )
        }
        _ => build_mock_mint_instruction(trade_info),
    };

    let token_program = crate::constants::token_2022_program_id();
    let usdc_mint = crate::constants::usdc_mint();
    let minter = crate::constants::admin_minter();

    // Build instructions in order:
    // 1. Create taker's GM ATA (idempotent - won't fail if it already exists)
    let create_taker_gm_ata_ix = create_associated_token_account_idempotent(
        &minter,                   // payer
        &trade_info.taker,         // wallet
        &trade_info.gm_token_mint, // mint
        &token_program,            // token program (Token-2022)
    );

    // 2. Create maker's GM ATA (idempotent - won't fail if it already exists)
    let create_maker_gm_ata_ix = create_associated_token_account_idempotent(
        &minter,                   // payer
        &trade_info.maker,         // wallet
        &trade_info.gm_token_mint, // mint
        &token_program,            // token program (Token-2022)
    );

    // 3. Create taker's USDC ATA (idempotent - needed for Jupiter fill to send USDC)
    let create_taker_usdc_ata_ix = create_associated_token_account_idempotent(
        &minter,           // payer
        &trade_info.taker, // wallet
        &usdc_mint,        // USDC mint
        &crate::constants::spl_token_program_id(),  // token program (SPL Token)
    );

    // 4. Create maker's USDC ATA (idempotent - needed for Jupiter fill to receive USDC)
    let create_maker_usdc_ata_ix = create_associated_token_account_idempotent(
        &minter,           // payer
        &trade_info.maker, // wallet
        &usdc_mint,        // USDC mint
        &crate::constants::spl_token_program_id(),  // token program (SPL Token)
    );

    // 5. Mint GM tokens to solver (maker) - `mint_ix`, chosen above
    let message = Message::new_with_blockhash(
        &[
            create_taker_gm_ata_ix,
            create_maker_gm_ata_ix,
            create_taker_usdc_ata_ix,
            create_maker_usdc_ata_ix,
            mint_ix,
        ],
        Some(&minter),
        &recent_blockhash,
    );
    Transaction::new_unsigned(message)
}

/// Build a mock mint instruction for bundle simulation.
///
/// This is a lower-level API that returns just the instruction if you
/// need more control over transaction construction.
///
/// # Arguments
///
/// * `trade_info` - The GM trade info from `check_gm_trade`
///
/// # Returns
///
/// An `Instruction` that mints GM tokens to the solver's token account
/// (`mint_portfolio` for portfolio tokens, `mint_gm` otherwise).
pub fn build_mock_mint_instruction(trade_info: &GmTradeInfo) -> Instruction {
    let build = if is_portfolio_token(&trade_info.gm_token_mint) {
        build_mock_mint_portfolio_instruction
    } else {
        build_mock_mint_gm_instruction
    };
    build(
        &trade_info.gm_token_mint,
        &trade_info.maker, // Mint to the solver (maker)
        trade_info.gm_token_amount,
    )
}

/// Build a mock mint instruction using the maker's output ATA directly.
///
/// Use this if you want to specify the exact destination token account
/// rather than deriving it from the maker's wallet.
///
/// # Arguments
///
/// * `trade_info` - The GM trade info from `check_gm_trade`
///
/// # Returns
///
/// An `Instruction` that mints GM tokens to the maker's output ATA.
pub fn build_mock_mint_instruction_to_ata(trade_info: &GmTradeInfo) -> Instruction {
    let build = if is_portfolio_token(&trade_info.gm_token_mint) {
        build_mock_mint_portfolio_instruction_with_ata
    } else {
        build_mock_mint_gm_instruction_with_ata
    };
    build(
        &trade_info.gm_token_mint,
        &trade_info.maker_output_account,
        &trade_info.maker, // Pass maker as the destination owner
        trade_info.gm_token_amount,
    )
}

/// Convenience function to check a transaction and build the mock mint if needed.
///
/// # Arguments
///
/// * `transaction` - The transaction to check
/// * `recent_blockhash` - A recent blockhash for the mock mint transaction
///
/// # Returns
///
/// * `Ok(Some(Transaction))` - A mock mint transaction if this is a GM trade
/// * `Ok(None)` - If this is not a GM trade
/// * `Err` - If there's an error parsing or validating
///
/// # Example
///
/// ```ignore
/// use ondo_gm_simulator::maybe_build_mock_mint;
///
/// match maybe_build_mock_mint(&fill_transaction, recent_blockhash)? {
///     Some(mock_mint_tx) => {
///         // Simulate as bundle: [mock_mint_tx, fill_transaction]
///     }
///     None => {
///         // Use normal single-transaction simulation
///     }
/// }
/// ```
pub fn maybe_build_mock_mint(
    transaction: &Transaction,
    recent_blockhash: Hash,
) -> Result<Option<Transaction>, GmSimulatorError> {
    let result = check_gm_trade(transaction)?;

    if let Some(trade_info) = result.trade_info {
        Ok(Some(build_mock_mint_transaction(
            &trade_info,
            recent_blockhash,
        )))
    } else {
        Ok(None)
    }
}

/// Simulate a bundle of transactions using Jito's simulateBundle RPC method.
///
/// This function sends the transactions to a Jito-enabled RPC endpoint for bundle simulation,
/// and extracts balance changes for the taker account from the Jupiter RFQ fill transaction.
///
/// # Arguments
///
/// * `transactions` - Vector of transactions to simulate as a bundle (typically [mock_mint_tx, fill_tx])
/// * `trade_info` - The GM trade info containing taker and token information
/// * `rpc_url` - The Jito-enabled RPC URL to use for simulation
///
/// # GM Program Upgrade Compatibility
///
/// `build_mock_mint_transaction` emits the `mint_gm` account layout of the
/// currently deployed program, which the upgraded program rejects (and vice
/// versa). If the first transaction is exactly what `build_mock_mint_transaction`
/// produces and it fails, the bundle is re-simulated once with the upgraded
/// layout (GM tokens only; portfolio tokens always use the upgraded
/// `mint_portfolio`). This costs an extra RPC call per simulation only once the
/// upgrade is live, until the default layout is flipped. If both layouts fail,
/// the result of the default layout is returned.
///
/// # Returns
///
/// A `BundleSimulationResult` containing:
/// - `success`: Whether the simulation succeeded
/// - `error`: Error message if simulation failed
/// - `taker_balance_changes`: Balance changes for the taker's token accounts
/// - `logs`: Optional simulation logs
///
/// # Example
///
/// ```ignore
/// use ondo_gm_simulator::{check_gm_trade, build_mock_mint_transaction, simulate_as_bundle};
///
/// let result = check_gm_trade(&fill_transaction)?;
/// if result.use_gm_bundle_sim {
///     let trade_info = result.trade_info.unwrap();
///     let mock_mint_tx = build_mock_mint_transaction(&trade_info, recent_blockhash);
///
///     let sim_result = simulate_as_bundle(
///         vec![mock_mint_tx, fill_transaction],
///         &trade_info,
///         "https://your-jito-rpc.com",
///     )?;
///
///     for change in &sim_result.taker_balance_changes {
///         println!("{}: {} ({})", change.symbol.as_deref().unwrap_or("?"), change.change_display(), change.change);
///     }
/// }
/// ```
pub fn simulate_as_bundle(
    transactions: Vec<Transaction>,
    trade_info: &crate::types::GmTradeInfo,
    rpc_url: &str,
) -> Result<crate::types::BundleSimulationResult, GmSimulatorError> {
    use crate::types::BundleSimulationResult;
    use crate::constants::{get_gm_token_symbol, usdc_mint};

    // Derive the taker's token accounts for pre/post balance checking
    // For the fill transaction (second tx), we want to track:
    // - Taker's input token account (USDC for BUY, GM for SELL)
    // - Taker's output token account (GM for BUY, USDC for SELL)
    let taker_usdc_ata = spl_associated_token_account::get_associated_token_address(
        &trade_info.taker,
        &usdc_mint(),
    );
    let taker_gm_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &trade_info.taker,
        &trade_info.gm_token_mint,
        &crate::constants::token_2022_program_id(),
    );
    let tracked_accounts = [taker_usdc_ata, taker_gm_ata];

    let mut json = request_simulate_bundle(&transactions, &tracked_accounts, rpc_url)?;

    // The GM program upgrade changes the `mint_gm` account layout. If our mock
    // mint was rejected, retry once with the upgraded layout so simulations
    // keep working whichever program version is deployed. The upgraded mint is
    // only swapped in when the caller passed our unmodified mock mint, so a
    // custom first transaction is never rewritten. Portfolio tokens already use
    // the upgraded `mint_portfolio`, so there is nothing to fall back to.
    if mock_mint_failed(&json) && !is_portfolio_token(&trade_info.gm_token_mint) {
        if let Some(mock_mint_tx) = transactions.first() {
            let blockhash = mock_mint_tx.message.recent_blockhash;
            if *mock_mint_tx == build_mock_mint_transaction(trade_info, blockhash) {
                let mut upgraded_txs = transactions.clone();
                upgraded_txs[0] =
                    build_mock_mint_transaction_with_layout(trade_info, blockhash, MintLayout::Upgraded);
                let upgraded_json = request_simulate_bundle(&upgraded_txs, &tracked_accounts, rpc_url)?;
                if !mock_mint_failed(&upgraded_json) {
                    json = upgraded_json;
                }
            }
        }
    }

    // Check for RPC-level errors
    if let Some(error) = json.get("error") {
        return Ok(BundleSimulationResult {
            success: false,
            error: Some(format!("RPC error: {}", error)),
            taker_balance_changes: vec![],
            logs: None,
        });
    }

    // Parse the result
    let result = json.get("result").ok_or_else(|| {
        GmSimulatorError::InstructionParseError("Missing result in response".to_string())
    })?;

    let value = result.get("value").ok_or_else(|| {
        GmSimulatorError::InstructionParseError("Missing value in result".to_string())
    })?;

    // Check transaction results
    let tx_results = value
        .get("transactionResults")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            GmSimulatorError::InstructionParseError("Missing transactionResults".to_string())
        })?;

    // Check if the fill transaction (index 1) succeeded
    let fill_result = tx_results.get(1).ok_or_else(|| {
        GmSimulatorError::InstructionParseError("Missing fill transaction result".to_string())
    })?;

    let fill_error = fill_result.get("err");
    let success = fill_error.map_or(true, |v| v.is_null());

    // Collect logs from the fill transaction
    let logs = fill_result
        .get("logs")
        .and_then(|l| l.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        });

    // Extract balance changes from pre/post execution accounts
    let mut taker_balance_changes = Vec::new();

    // Get pre-execution accounts for the fill tx
    let pre_accounts = fill_result
        .get("preExecutionAccounts")
        .and_then(|v| v.as_array());

    // Get post-execution accounts for the fill tx
    let post_accounts = fill_result
        .get("postExecutionAccounts")
        .and_then(|v| v.as_array());

    if let (Some(pre), Some(post)) = (pre_accounts, post_accounts) {
        // Process USDC balance change (index 0)
        if let (Some(pre_usdc), Some(post_usdc)) = (pre.get(0), post.get(0)) {
            if let Some(change) = parse_token_balance_change(
                pre_usdc,
                post_usdc,
                &usdc_mint(),
                Some("USDC".to_string()),
                &trade_info.taker,
                &taker_usdc_ata,
                6, // USDC has 6 decimals
            ) {
                taker_balance_changes.push(change);
            }
        }

        // Process GM token balance change (index 1)
        if let (Some(pre_gm), Some(post_gm)) = (pre.get(1), post.get(1)) {
            if let Some(change) = parse_token_balance_change(
                pre_gm,
                post_gm,
                &trade_info.gm_token_mint,
                Some(get_gm_token_symbol(&trade_info.gm_token_mint)
                    .unwrap_or("GM")
                    .to_string()),
                &trade_info.taker,
                &taker_gm_ata,
                9, // GM tokens have 9 decimals
            ) {
                taker_balance_changes.push(change);
            }
        }
    }

    Ok(BundleSimulationResult {
        success,
        error: if success {
            None
        } else {
            Some(format!("Fill transaction failed: {:?}", fill_error))
        },
        taker_balance_changes,
        logs,
    })
}

/// Send a Jito `simulateBundle` request and return the raw JSON-RPC response.
///
/// Pre/post execution snapshots of `tracked_accounts` are requested for the
/// fill transaction (index 1) only.
fn request_simulate_bundle(
    transactions: &[Transaction],
    tracked_accounts: &[solana_sdk::pubkey::Pubkey],
    rpc_url: &str,
) -> Result<serde_json::Value, GmSimulatorError> {
    use base64::Engine;

    // Encode transactions as base64
    let encoded_txs: Vec<String> = transactions
        .iter()
        .map(|tx| {
            base64::engine::general_purpose::STANDARD.encode(
                bincode::serialize(tx).expect("Failed to serialize transaction"),
            )
        })
        .collect();

    let addresses: Vec<String> = tracked_accounts.iter().map(|a| a.to_string()).collect();

    // Build the Jito simulateBundle request with pre/post execution account configs
    // We want post-execution accounts for the fill transaction (index 1)
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "simulateBundle",
        "params": [
            {
                "encodedTransactions": encoded_txs
            },
            {
                "preExecutionAccountsConfigs": [
                    null,  // Don't need pre for mock mint
                    { "addresses": addresses }
                ],
                "postExecutionAccountsConfigs": [
                    null,  // Don't need post for mock mint
                    { "addresses": addresses }
                ],
                "replaceRecentBlockhash": true,
                "skipSigVerify": true,
                "simulationBank": {
                    "commitment": {
                        "commitment": "processed"
                    }
                }
            }
        ]
    });

    // Send the request
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .map_err(|e| GmSimulatorError::InstructionParseError(format!("HTTP request failed: {}", e)))?;

    let response_text = response
        .text()
        .map_err(|e| GmSimulatorError::InstructionParseError(format!("Failed to read response: {}", e)))?;

    serde_json::from_str(&response_text)
        .map_err(|e| GmSimulatorError::InstructionParseError(format!("Failed to parse JSON: {}", e)))
}

/// Whether the first transaction in the bundle (the mock mint) failed.
///
/// When a bundle transaction fails, Jito stops executing and only returns
/// results up to and including the failed one, with the error in `err`.
fn mock_mint_failed(json: &serde_json::Value) -> bool {
    json.pointer("/result/value/transactionResults/0/err")
        .map_or(false, |err| !err.is_null())
}

/// Helper function to parse token balance change from Jito response
fn parse_token_balance_change(
    pre_account: &serde_json::Value,
    post_account: &serde_json::Value,
    mint: &solana_sdk::pubkey::Pubkey,
    symbol: Option<String>,
    owner: &solana_sdk::pubkey::Pubkey,
    token_account: &solana_sdk::pubkey::Pubkey,
    decimals: u8,
) -> Option<crate::types::BalanceChange> {
    // Parse pre-balance from the account data
    let pre_balance = parse_token_account_balance(pre_account).unwrap_or(0);
    let post_balance = parse_token_account_balance(post_account).unwrap_or(0);

    let change = post_balance as i128 - pre_balance as i128;

    // Only return if there was a change or we have valid data
    if pre_balance != 0 || post_balance != 0 || change != 0 {
        Some(crate::types::BalanceChange {
            mint: *mint,
            symbol,
            owner: *owner,
            token_account: *token_account,
            pre_balance,
            post_balance,
            change,
            decimals,
        })
    } else {
        None
    }
}

/// Parse token balance from a Jito account response
fn parse_token_account_balance(account: &serde_json::Value) -> Option<u64> {
    // Jito returns account data in base64 format
    // Token account data layout: mint (32) + owner (32) + amount (8) + ...
    use base64::Engine;

    let data_str = account.get("data")?.as_array()?.get(0)?.as_str()?;
    let data = base64::engine::general_purpose::STANDARD.decode(data_str).ok()?;

    // Token account amount is at bytes 64-72 (after mint and owner)
    if data.len() >= 72 {
        let amount_bytes: [u8; 8] = data[64..72].try_into().ok()?;
        Some(u64::from_le_bytes(amount_bytes))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::usdc_mint;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
    };
    use std::str::FromStr;

    fn create_mock_jupiter_fill(
        maker: &Pubkey,
        taker: &Pubkey,
        input_mint: &Pubkey,
        output_mint: &Pubkey,
        input_amount: u64,
        output_amount: u64,
    ) -> Instruction {
        let jupiter_program_id = jupiter_order_engine_program_id();

        // Build instruction data: discriminator + input_amount + output_amount + expire_at
        let fill_discriminator = crate::instruction_discriminator("fill");
        let mut data = fill_discriminator.to_vec();
        data.extend_from_slice(&input_amount.to_le_bytes());
        data.extend_from_slice(&output_amount.to_le_bytes());
        // Add a mock expire_at timestamp (e.g., 1 hour from now in unix time)
        let expire_at: i64 = 1704067200; // Mock timestamp
        data.extend_from_slice(&expire_at.to_le_bytes());

        let taker_input_ata = Pubkey::new_unique();
        let maker_input_ata = Pubkey::new_unique();
        let taker_output_ata = Pubkey::new_unique();
        let maker_output_ata = Pubkey::new_unique();

        // Account order matches actual Jupiter RFQ fill layout:
        // taker, maker, taker_input_ata, maker_input_ata, taker_output_ata, maker_output_ata,
        // input_mint, input_token_program, output_mint
        Instruction {
            program_id: jupiter_program_id,
            accounts: vec![
                AccountMeta::new(*taker, true),                // 0: taker
                AccountMeta::new(*maker, true),                // 1: maker
                AccountMeta::new(taker_input_ata, false),      // 2: taker_input_ata
                AccountMeta::new(maker_input_ata, false),      // 3: maker_input_ata
                AccountMeta::new(taker_output_ata, false),     // 4: taker_output_ata
                AccountMeta::new(maker_output_ata, false),     // 5: maker_output_ata
                AccountMeta::new_readonly(*input_mint, false), // 6: input_mint
                AccountMeta::new_readonly(crate::constants::token_2022_program_id(), false), // 7: input_token_program
                AccountMeta::new_readonly(*output_mint, false), // 8: output_mint
            ],
            data,
        }
    }

    #[test]
    fn test_check_gm_trade_buy() {
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let user = Keypair::new();
        let usdc = usdc_mint();
        let aapl = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();

        let ix = create_mock_jupiter_fill(
            &solver,
            &user.pubkey(),
            &usdc,
            &aapl,
            200_000_000,
            1_500_000_000,
        );

        let message = Message::new(&[ix], Some(&user.pubkey()));
        let result = check_gm_trade_message(&message).unwrap();

        assert!(result.use_gm_bundle_sim);
        let info = result.trade_info.unwrap();
        assert_eq!(info.maker, solver);
        assert_eq!(info.taker, user.pubkey());
        assert_eq!(info.gm_token_mint, aapl);
        assert_eq!(info.gm_token_symbol, "AAPLon");
        assert_eq!(info.gm_token_amount, 1_500_000_000);
        assert_eq!(info.expire_at, 1704067200); // Verify expire_at is parsed
    }

    #[test]
    fn test_check_gm_trade_unauthorized_maker() {
        let unauthorized_maker = Pubkey::new_unique();
        let user = Keypair::new();
        let usdc = usdc_mint();
        let aapl = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();

        let ix = create_mock_jupiter_fill(
            &unauthorized_maker,
            &user.pubkey(),
            &usdc,
            &aapl,
            200_000_000,
            1_500_000_000,
        );

        let message = Message::new(&[ix], Some(&user.pubkey()));
        let result = check_gm_trade_message(&message);

        assert!(matches!(
            result,
            Err(GmSimulatorError::UnauthorizedMaker(_))
        ));
    }

    #[test]
    fn test_check_gm_trade_not_gm_token() {
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let user = Keypair::new();
        let usdc = usdc_mint();
        let random_token = Pubkey::new_unique();

        let ix = create_mock_jupiter_fill(
            &solver,
            &user.pubkey(),
            &usdc,
            &random_token,
            200_000_000,
            1_000_000_000,
        );

        let message = Message::new(&[ix], Some(&user.pubkey()));
        let result = check_gm_trade_message(&message).unwrap();

        assert!(!result.use_gm_bundle_sim);
        assert!(result.trade_info.is_none());
    }

    #[test]
    fn test_check_gm_trade_with_create_ata() {
        use spl_associated_token_account::instruction::create_associated_token_account_idempotent;

        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let user = Keypair::new();
        let usdc = usdc_mint();
        let aapl = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();

        // Create ATA instruction (typically included by solvers)
        let create_ata_ix = create_associated_token_account_idempotent(
            &solver,
            &user.pubkey(),
            &aapl,
            &crate::constants::token_2022_program_id(),
        );

        // Jupiter fill instruction
        let fill_ix = create_mock_jupiter_fill(
            &solver,
            &user.pubkey(),
            &usdc,
            &aapl,
            200_000_000,
            1_500_000_000,
        );

        // Transaction with both instructions (realistic scenario)
        let message = Message::new(&[create_ata_ix, fill_ix], Some(&user.pubkey()));
        let result = check_gm_trade_message(&message).unwrap();

        // Should still detect as GM trade despite multiple instructions
        assert!(result.use_gm_bundle_sim);
        let info = result.trade_info.unwrap();
        assert_eq!(info.gm_token_mint, aapl);
        assert_eq!(info.gm_token_amount, 1_500_000_000);
    }

    #[test]
    fn test_check_gm_trade_multiple_fills() {
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let user = Keypair::new();
        let usdc = usdc_mint();
        let aapl = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();

        let ix1 = create_mock_jupiter_fill(
            &solver,
            &user.pubkey(),
            &usdc,
            &aapl,
            200_000_000,
            1_500_000_000,
        );
        let ix2 = create_mock_jupiter_fill(
            &solver,
            &user.pubkey(),
            &usdc,
            &aapl,
            100_000_000,
            750_000_000,
        );

        let message = Message::new(&[ix1, ix2], Some(&user.pubkey()));
        let result = check_gm_trade_message(&message).unwrap();

        // With multiple fill instructions, we detect the first one as a GM trade
        // This is an edge case - in practice, transactions typically have one fill
        assert!(result.use_gm_bundle_sim);
    }

    #[test]
    fn test_check_gm_trade_portfolio_buy() {
        let maker = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let taker = Keypair::new().pubkey();
        let brain = Pubkey::from_str("CPcYDPoNoD42Yqvv2kKfB93RqL73vxnXEoRRgqqondo").unwrap();

        let fill_ix = create_mock_jupiter_fill(&maker, &taker, &usdc_mint(), &brain, 100_000_000, 2_000_000_000);
        let tx = Transaction::new_unsigned(Message::new(&[fill_ix], Some(&taker)));

        let result = check_gm_trade(&tx).unwrap();
        assert!(result.use_gm_bundle_sim);
        let trade_info = result.trade_info.unwrap();
        assert_eq!(trade_info.gm_token_symbol, "BRAINon");

        // The mock mint must use mint_portfolio, not mint_gm
        let mock_tx = build_mock_mint_transaction(&trade_info, Hash::default());
        let mint_ix = &mock_tx.message.instructions[4];
        assert_eq!(mint_ix.data[..8], crate::instruction_discriminator("mint_portfolio"));
        assert_eq!(build_mock_mint_instruction(&trade_info).data[..8], crate::instruction_discriminator("mint_portfolio"));
        assert_eq!(build_mock_mint_instruction_to_ata(&trade_info).data[..8], crate::instruction_discriminator("mint_portfolio"));

        // Portfolio tokens have one layout, so the requested GM layout is irrelevant
        assert_eq!(
            build_mock_mint_transaction_with_layout(&trade_info, Hash::default(), MintLayout::Upgraded),
            mock_tx,
        );
    }

    #[test]
    fn test_upgraded_mock_mint_transaction_only_swaps_mint() {
        let trade_info = GmTradeInfo {
            maker: Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap(),
            taker: Pubkey::new_unique(),
            gm_token_mint: Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap(),
            gm_token_symbol: "AAPLon".to_string(),
            gm_token_amount: 1_500_000_000,
            maker_output_account: Pubkey::new_unique(),
            expire_at: 1704067200,
        };
        let blockhash = Hash::new_unique();

        let current = build_mock_mint_transaction(&trade_info, blockhash);
        let legacy = build_mock_mint_transaction_with_layout(&trade_info, blockhash, MintLayout::Legacy);
        let upgraded = build_mock_mint_transaction_with_layout(&trade_info, blockhash, MintLayout::Upgraded);

        // The default stays the deployed layout until the upgrade is live
        assert_eq!(current, legacy);

        // Same fee payer, blockhash and ATA set-up; only the mint_gm instruction differs
        let mint_accounts = |tx: &Transaction| tx.message.instructions[4].accounts.len();
        assert_eq!(upgraded.message.account_keys[0], current.message.account_keys[0]);
        assert_eq!(upgraded.message.recent_blockhash, blockhash);
        assert_eq!(upgraded.message.instructions.len(), 5);
        assert_eq!(mint_accounts(&current), 12);
        assert_eq!(mint_accounts(&upgraded), 14);
        assert_ne!(upgraded, current);
    }

    /// Response shapes captured from Jito `simulateBundle` on mainnet.
    #[test]
    fn test_mock_mint_failed() {
        // Mock mint rejected: Jito stops after tx 0 and reports its error
        let mint_failed = serde_json::json!({
            "result": { "value": {
                "summary": { "failed": {
                    "error": { "TransactionFailure": [[0], "Error processing Instruction 4: custom program error: 0xbc4"] },
                    "tx_signature": "1111111111111111111111111111111111111111111111111111111111111111"
                }},
                "transactionResults": [{ "err": { "InstructionError": [4, { "Custom": 3012 }] }, "logs": [] }]
            }}
        });
        assert!(mock_mint_failed(&mint_failed));

        // Mock mint succeeded, fill failed: not a layout problem, no fallback
        let fill_failed = serde_json::json!({
            "result": { "value": {
                "transactionResults": [{ "err": null }, { "err": { "InstructionError": [1, { "Custom": 1 }] } }]
            }}
        });
        assert!(!mock_mint_failed(&fill_failed));

        let succeeded = serde_json::json!({
            "result": { "value": {
                "summary": "succeeded",
                "transactionResults": [{ "err": null }, { "err": null }]
            }}
        });
        assert!(!mock_mint_failed(&succeeded));

        // RPC-level error is surfaced as-is, never retried
        let rpc_error = serde_json::json!({ "error": { "code": -32602, "message": "bad params" } });
        assert!(!mock_mint_failed(&rpc_error));
    }

    /// Serve `responses` in order from a local JSON-RPC stub and return its URL
    /// plus a channel carrying each request body received.
    fn spawn_rpc_stub(
        responses: Vec<serde_json::Value>,
    ) -> (String, std::sync::mpsc::Receiver<serde_json::Value>) {
        use std::io::{BufRead, BufReader, Read, Write};

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for response in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());

                // Read the headers up to the blank line, then exactly Content-Length body bytes
                let mut content_length = 0;
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line == "\r\n" {
                        break;
                    }
                    if let Some(len) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                        content_length = len.trim().parse().unwrap();
                    }
                }
                let mut body = vec![0; content_length];
                reader.read_exact(&mut body).unwrap();
                tx.send(serde_json::from_slice(&body).unwrap()).unwrap();

                let response = response.to_string();
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    response.len(),
                    response
                )
                .unwrap();
            }
        });
        (url, rx)
    }

    /// Exercises the `simulate_as_bundle` layout fallback against a local stub,
    /// so the retry is covered without a Jito RPC.
    #[test]
    fn test_simulate_as_bundle_layout_fallback() {
        let mint_failed = serde_json::json!({ "result": { "value": {
            "transactionResults": [{ "err": { "InstructionError": [4, { "Custom": 3012 }] } }]
        }}});
        let succeeded = |tag: &str| serde_json::json!({ "result": { "value": {
            "transactionResults": [{ "err": null }, { "err": null, "logs": [tag] }]
        }}});

        let gm_trade = GmTradeInfo {
            maker: Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap(),
            taker: Pubkey::new_unique(),
            gm_token_mint: Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap(),
            gm_token_symbol: "AAPLon".to_string(),
            gm_token_amount: 1_500_000_000,
            maker_output_account: Pubkey::new_unique(),
            expire_at: 1704067200,
        };
        let portfolio_trade = GmTradeInfo {
            gm_token_mint: Pubkey::from_str("CPcYDPoNoD42Yqvv2kKfB93RqL73vxnXEoRRgqqondo").unwrap(),
            gm_token_symbol: "BRAINon".to_string(),
            ..gm_trade.clone()
        };
        let blockhash = Hash::new_unique();
        let fill_tx = Transaction::new_unsigned(Message::new(&[], Some(&gm_trade.taker)));

        // Run one simulation; return the result and the mint account count of
        // each mock mint the stub received, in request order.
        let run = |trade_info: &GmTradeInfo, mock_mint: Transaction, responses: Vec<serde_json::Value>| {
            use base64::Engine;
            let (url, requests) = spawn_rpc_stub(responses);
            let result = simulate_as_bundle(vec![mock_mint, fill_tx.clone()], trade_info, &url);
            let mint_accounts: Vec<usize> = requests
                .try_iter()
                .map(|body| {
                    let encoded = body["params"][0]["encodedTransactions"][0].as_str().unwrap().to_string();
                    let bytes = base64::engine::general_purpose::STANDARD.decode(encoded).unwrap();
                    let tx: Transaction = bincode::deserialize(&bytes).unwrap();
                    tx.message.instructions[4].accounts.len()
                })
                .collect();
            (result, mint_accounts)
        };
        let default_mint = |trade_info: &GmTradeInfo| build_mock_mint_transaction(trade_info, blockhash);

        // Default (deployed) layout accepted: a single request, no retry
        let (result, mints) = run(&gm_trade, default_mint(&gm_trade), vec![succeeded("default")]);
        assert_eq!(mints, vec![12]);
        assert_eq!(result.unwrap().logs, Some(vec!["default".to_string()]));

        // Default layout rejected: retried once with the upgraded layout, whose result wins
        let (result, mints) = run(&gm_trade, default_mint(&gm_trade), vec![mint_failed.clone(), succeeded("upgraded")]);
        assert_eq!(mints, vec![12, 14]);
        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(result.logs, Some(vec!["upgraded".to_string()]));

        // Both layouts rejected: the default layout's response is surfaced
        let (result, mints) = run(&gm_trade, default_mint(&gm_trade), vec![mint_failed.clone(), mint_failed.clone()]);
        assert_eq!(mints, vec![12, 14]);
        assert!(matches!(result, Err(GmSimulatorError::InstructionParseError(e)) if e.contains("Missing fill")));

        // A caller-built first transaction is never rewritten
        let custom = build_mock_mint_transaction_with_layout(&gm_trade, blockhash, MintLayout::Upgraded);
        let (_, mints) = run(&gm_trade, custom, vec![mint_failed.clone()]);
        assert_eq!(mints, vec![14]);

        // Portfolio tokens have no other layout to retry with
        let (_, mints) = run(&portfolio_trade, default_mint(&portfolio_trade), vec![mint_failed]);
        assert_eq!(mints, vec![14]);
    }

    #[test]
    fn test_build_mock_mint_transaction() {
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let aapl = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();

        let trade_info = GmTradeInfo {
            maker: solver,
            taker: Pubkey::new_unique(),
            gm_token_mint: aapl,
            gm_token_symbol: "AAPLon".to_string(),
            gm_token_amount: 1_500_000_000,
            maker_output_account: Pubkey::new_unique(),
            expire_at: 1704067200,
        };

        let mock_tx = build_mock_mint_transaction(&trade_info, Hash::default());

        // Verify the transaction structure
        // Should have 5 instructions: create taker GM ATA + create maker GM ATA + create taker USDC ATA + create maker USDC ATA + mint
        assert_eq!(mock_tx.message.instructions.len(), 5);
        // Transaction has 1 signature slot (for the minter/fee payer), but it's not signed yet
        assert_eq!(mock_tx.signatures.len(), 1);
        // All signatures should be default (all zeros) since it's unsigned
        assert!(mock_tx
            .signatures
            .iter()
            .all(|sig| sig.as_ref().iter().all(|&b| b == 0)));
    }

    /// Comprehensive test with hardcoded transactions for both BUY and SELL scenarios.
    ///
    /// Run with: `RPC_URL=<your_rpc> cargo test test_from_scratch -- --ignored --nocapture`
    ///
    /// This test combines the best features from test_jito_bundle_simulation_from_scratch,
    /// test_hardcoded_buy, and test_hardcoded_sell:
    /// - Tests both BUY (USDC → GM) and SELL (GM → USDC) scenarios
    /// - Uses proper Jito bundle simulation types and helpers
    /// - Includes detailed debug output and ATA derivation checks
    /// - Verifies on-chain account states
    /// - Shows proper detection logic for both trade types
    /// - Demonstrates that BUY needs bundle simulation, SELL doesn't
    #[test]
    #[ignore]
    fn test_from_scratch() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::commitment_config::CommitmentConfig;
        use solana_sdk::instruction::{AccountMeta, Instruction};
        use std::str::FromStr;

        println!("{}", "=".repeat(80));
        println!("COMPREHENSIVE FROM-SCRATCH TEST (BUY AND SELL)");
        println!("{}", "=".repeat(80));

        // Initialize RPC client
        let rpc_url = std::env::var("RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        println!("Using RPC: {}", rpc_url);

        let client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());

        // Get fresh blockhash
        println!("\nFetching fresh blockhash...");
        let fresh_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get fresh blockhash");
        println!("✓ Got fresh blockhash: {}", fresh_blockhash);

        // Common accounts
        let taker = Pubkey::from_str("7z86y3WYofAiuxhQvYV2U6ZQMQ7jLxncuyV9U7D8PwYV").unwrap(); // Funded with USDC
        let maker = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap(); // Real solver
        let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let aapl_mint = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();

        let future_expire = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 3600;

        // ===========================================
        // TEST 1: BUY Transaction (USDC → GM)
        // ===========================================
        println!("\n{}", "=".repeat(80));
        println!("TEST 1: BUY TRANSACTION (USDC → GM) - REQUIRES BUNDLE SIMULATION");
        println!("{}", "=".repeat(80));

        // Derive ATAs
        let taker_usdc_ata = spl_associated_token_account::get_associated_token_address(&taker, &usdc_mint);
        let taker_aapl_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
            &taker, &aapl_mint, &spl_token_2022::id(),
        );
        let maker_usdc_ata = spl_associated_token_account::get_associated_token_address(&maker, &usdc_mint);
        let maker_aapl_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
            &maker, &aapl_mint, &spl_token_2022::id(),
        );

        println!("\nAccounts (BUY):");
        println!("  Taker:           {}", taker);
        println!("  Maker:           {}", maker);
        println!("  Taker USDC ATA:  {}", taker_usdc_ata);
        println!("  Maker USDC ATA:  {}", maker_usdc_ata);
        println!("  Taker AAPLon ATA: {}", taker_aapl_ata);
        println!("  Maker AAPLon ATA: {}", maker_aapl_ata);

        // Check on-chain account states
        println!("\nChecking on-chain account states...");
        print!("  Taker USDC ATA: ");
        match client.get_token_account_balance(&taker_usdc_ata) {
            Ok(bal) => println!("EXISTS, balance: {}", bal.ui_amount_string),
            Err(_) => println!("DOES NOT EXIST (will be created by mock mint)"),
        }
        print!("  Maker GM ATA:   ");
        match client.get_account(&maker_aapl_ata) {
            Ok(_) => println!("EXISTS"),
            Err(_) => println!("DOES NOT EXIST (will be created by mock mint)"),
        }

        // Build Jupiter fill instruction for BUY
        let mut buy_data = vec![0xa8, 0x60, 0xb7, 0xa3, 0x5c, 0x0a, 0x28, 0xa0]; // fill discriminator
        buy_data.extend_from_slice(&1000000u64.to_le_bytes()); // 1 USDC
        buy_data.extend_from_slice(&3880411u64.to_le_bytes()); // 3.880411 AAPLon
        buy_data.extend_from_slice(&future_expire.to_le_bytes());

        println!("\nBuilding BUY transaction:");
        println!("  Input: 1 USDC");
        println!("  Output: 3.880411 AAPLon");
        println!("  Expire at: {} (future)", future_expire);

        let buy_fill_ix = Instruction {
            program_id: jupiter_order_engine_program_id(),
            accounts: vec![
                AccountMeta::new(taker, true),
                AccountMeta::new(maker, true),
                AccountMeta::new(taker_usdc_ata, false),
                AccountMeta::new(maker_usdc_ata, false),
                AccountMeta::new(taker_aapl_ata, false),
                AccountMeta::new(maker_aapl_ata, false),
                AccountMeta::new_readonly(usdc_mint, false),
                AccountMeta::new_readonly(crate::constants::spl_token_program_id(), false),
                AccountMeta::new_readonly(aapl_mint, false),
                AccountMeta::new_readonly(crate::constants::token_2022_program_id(), false),
                AccountMeta::new_readonly(solana_system_interface::program::id(), false),
            ],
            data: buy_data,
        };

        let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &taker, &taker, &aapl_mint, &spl_token_2022::id(),
        );

        let buy_message = Message::new_with_blockhash(&[create_ata_ix, buy_fill_ix], Some(&taker), &fresh_blockhash);
        let buy_tx = Transaction::new_unsigned(buy_message);

        println!("✓ BUY transaction built");

        // Test GM trade detection
        println!("\nTesting GM trade detection for BUY...");
        let buy_result = check_gm_trade(&buy_tx).expect("Failed to check GM trade");

        if buy_result.use_gm_bundle_sim {
            println!("✓ Correctly identified as GM BUY trade (bundle simulation required)");
            let trade_info = buy_result.trade_info.expect("Expected trade info");

            // Build and simulate bundle
            println!("\nBuilding mock mint transaction...");
            let mock_mint_tx = build_mock_mint_transaction(&trade_info, fresh_blockhash);
            println!("✓ Mock mint transaction built ({} instructions)", mock_mint_tx.message.instructions.len());

            println!("\nSimulating bundle with Jito using simulate_as_bundle...");
            match simulate_as_bundle(vec![mock_mint_tx, buy_tx], &trade_info, &rpc_url) {
                Ok(sim_result) => {
                    if sim_result.success {
                        println!("  ✓ Bundle simulation succeeded");
                    } else {
                        println!("  ✗ Bundle simulation failed: {:?}", sim_result.error);
                    }

                    // Print taker balance changes
                    println!("\n  Taker Balance Changes:");
                    if sim_result.taker_balance_changes.is_empty() {
                        println!("    (no balance changes detected)");
                    } else {
                        for change in &sim_result.taker_balance_changes {
                            let symbol = change.symbol.as_deref().unwrap_or("?");
                            let sign = if change.change >= 0 { "+" } else { "" };
                            println!(
                                "    {} {}: {}{:.6} (raw: {}{})",
                                change.token_account,
                                symbol,
                                sign,
                                change.change_display(),
                                sign,
                                change.change
                            );
                            println!(
                                "      Pre-balance:  {} ({:.6} {})",
                                change.pre_balance,
                                change.pre_balance as f64 / 10f64.powi(change.decimals as i32),
                                symbol
                            );
                            println!(
                                "      Post-balance: {} ({:.6} {})",
                                change.post_balance,
                                change.post_balance as f64 / 10f64.powi(change.decimals as i32),
                                symbol
                            );
                        }
                    }

                    // Print logs if available
                    if let Some(logs) = &sim_result.logs {
                        println!("\n    Logs ({} entries):", logs.len());
                        for log in logs.iter().take(10) {
                            println!("      {}", log);
                        }
                        if logs.len() > 10 {
                            println!("      ... and {} more", logs.len() - 10);
                        }
                    }
                }
                Err(e) => println!("  ✗ simulate_as_bundle failed: {:?}", e),
            }
        } else {
            panic!("BUY transaction not identified as requiring bundle simulation");
        }

        // ===========================================
        // TEST 2: SELL Transaction (GM → USDC)
        // ===========================================
        println!("\n{}", "=".repeat(80));
        println!("TEST 2: SELL TRANSACTION (GM → USDC) - NO BUNDLE SIMULATION REQUIRED");
        println!("{}", "=".repeat(80));

        // For SELL, taker has GM tokens and wants USDC
        let sell_taker_gm_ata = Pubkey::new_unique(); // Simulated GM ATA
        let sell_taker_usdc_ata = Pubkey::new_unique();
        let sell_maker_gm_ata = Pubkey::new_unique();
        let sell_maker_usdc_ata = Pubkey::new_unique();

        println!("\nBuilding SELL transaction:");
        println!("  Input: 0.007 AAPLon");
        println!("  Output: 0.001801 USDC");

        let mut sell_data = vec![0xa8, 0x60, 0xb7, 0xa3, 0x5c, 0x0a, 0x28, 0xa0]; // fill discriminator
        sell_data.extend_from_slice(&7000000u64.to_le_bytes()); // 0.007 AAPLon
        sell_data.extend_from_slice(&1801u64.to_le_bytes()); // 0.001801 USDC
        sell_data.extend_from_slice(&future_expire.to_le_bytes());

        let sell_fill_ix = Instruction {
            program_id: jupiter_order_engine_program_id(),
            accounts: vec![
                AccountMeta::new(taker, true),
                AccountMeta::new(maker, true),
                AccountMeta::new(sell_taker_gm_ata, false),   // Taker input (GM)
                AccountMeta::new(sell_maker_usdc_ata, false), // Maker input (USDC - has it already)
                AccountMeta::new(sell_maker_gm_ata, false),   // Maker output (GM)
                AccountMeta::new(sell_taker_usdc_ata, false), // Taker output (USDC)
                AccountMeta::new_readonly(aapl_mint, false),  // Input mint (GM)
                AccountMeta::new_readonly(crate::constants::token_2022_program_id(), false),
                AccountMeta::new_readonly(usdc_mint, false),  // Output mint (USDC)
                AccountMeta::new_readonly(crate::constants::spl_token_program_id(), false),
                AccountMeta::new_readonly(solana_system_interface::program::id(), false),
            ],
            data: sell_data,
        };

        let sell_message = Message::new_with_blockhash(&[sell_fill_ix], Some(&taker), &fresh_blockhash);
        let sell_tx = Transaction::new_unsigned(sell_message);

        println!("✓ SELL transaction built");

        // Test GM trade detection
        println!("\nTesting GM trade detection for SELL...");
        let sell_result = check_gm_trade(&sell_tx).expect("Failed to check GM trade");

        if !sell_result.use_gm_bundle_sim {
            println!("✓ Correctly identified as GM SELL trade (bundle simulation NOT required)");
            println!("  Reason: Solver already has USDC, no minting needed");
        } else {
            panic!("SELL transaction incorrectly identified as requiring bundle simulation");
        }

        println!("\n{}", "=".repeat(80));
        println!("✓ FROM-SCRATCH TEST COMPLETED SUCCESSFULLY");
        println!("  • BUY transaction correctly requires bundle simulation");
        println!("  • SELL transaction correctly does NOT require bundle simulation");
        println!("  • All detection logic working properly");
        println!("{}", "=".repeat(80));
    }

    /// Comprehensive test that fetches a real mainnet transaction and simulates it.
    ///
    /// Run with: `TX_HASH=<hash> RPC_URL=<rpc> cargo test test_mainnet -- --ignored --nocapture`
    ///
    /// This test combines the best features from test_jito_bundle_simulation and
    /// test_mainnet_transaction:
    /// - Fetches real transactions from mainnet using TX_HASH env var
    /// - Handles both BUY and SELL transactions appropriately
    /// - Uses proper Jito bundle simulation types and helpers
    /// - Includes extensive debug output and trade analysis
    /// - Updates expire_at to prevent expiration errors
    /// - Shows detailed detection criteria and reasoning
    #[test]
    #[ignore]
    fn test_mainnet() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::commitment_config::CommitmentConfig;
        use solana_sdk::signature::Signature;
        use solana_transaction_status::UiTransactionEncoding;

        println!("{}", "=".repeat(80));
        println!("COMPREHENSIVE MAINNET TRANSACTION TEST");
        println!("{}", "=".repeat(80));

        // Use environment variable or default to a known GM trade transaction
        let tx_hash = std::env::var("TX_HASH")
            .unwrap_or_else(|_| {
                println!("No TX_HASH provided, using default. Set TX_HASH=<hash> to test a specific transaction.");
                "YOUR_DEFAULT_TX_HASH_HERE".to_string()
            });

        println!("Testing with transaction: {}", tx_hash);

        // Initialize RPC client for mainnet
        let rpc_url = std::env::var("RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        println!("Using RPC: {}", rpc_url);

        let client = RpcClient::new_with_commitment(rpc_url.clone(), CommitmentConfig::confirmed());

        // Parse the transaction signature
        let signature = Signature::from_str(&tx_hash).expect("Invalid transaction signature");

        // Fetch the transaction from mainnet
        println!("\nFetching transaction from mainnet...");
        let config = solana_client::rpc_config::RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        };
        let tx_with_meta = client
            .get_transaction_with_config(&signature, config)
            .expect("Failed to fetch transaction from mainnet");

        // Extract the transaction - it comes as base64 encoded
        let encoded_tx = tx_with_meta.transaction.transaction;
        let tx_data = match encoded_tx {
            solana_transaction_status::EncodedTransaction::Binary(data, encoding) => {
                use base64::Engine;
                match encoding {
                    solana_transaction_status::TransactionBinaryEncoding::Base58 => {
                        bs58::decode(&data)
                            .into_vec()
                            .expect("Failed to decode base58")
                    }
                    solana_transaction_status::TransactionBinaryEncoding::Base64 => {
                        base64::engine::general_purpose::STANDARD
                            .decode(&data)
                            .expect("Failed to decode base64")
                    }
                }
            }
            _ => panic!("Transaction not in binary format"),
        };

        // Try to deserialize as VersionedTransaction first (v0 transactions)
        let mut original_tx: Transaction = if let Ok(versioned_tx) =
            bincode::deserialize::<solana_sdk::transaction::VersionedTransaction>(&tx_data)
        {
            // Convert VersionedTransaction to legacy Transaction for our API
            match versioned_tx.message {
                solana_sdk::message::VersionedMessage::Legacy(legacy_msg) => {
                    let mut tx = Transaction::new_unsigned(legacy_msg);
                    tx.signatures = versioned_tx.signatures;
                    tx
                }
                solana_sdk::message::VersionedMessage::V0(v0_msg) => {
                    // Convert v0 message to legacy format
                    let legacy_msg = Message {
                        header: v0_msg.header,
                        account_keys: v0_msg.account_keys,
                        recent_blockhash: v0_msg.recent_blockhash,
                        instructions: v0_msg.instructions,
                    };
                    let mut tx = Transaction::new_unsigned(legacy_msg);
                    tx.signatures = versioned_tx.signatures;
                    tx
                }
            }
        } else {
            // Fall back to legacy transaction deserialization
            bincode::deserialize(&tx_data).expect("Failed to deserialize transaction")
        };

        println!("✓ Fetched transaction successfully");
        println!("  Instructions: {}", original_tx.message.instructions.len());
        println!("  Signatures: {}", original_tx.signatures.len());

        // Debug: print all program IDs and accounts in the transaction
        println!("\nTransaction Analysis:");
        println!("  Programs:");
        for (i, instruction) in original_tx.message.instructions.iter().enumerate() {
            let program_id =
                original_tx.message.account_keys[instruction.program_id_index as usize];
            println!("    Instruction {}: {}", i, program_id);
            if program_id == jupiter_order_engine_program_id() {
                println!("      ✓ Jupiter Order Engine fill instruction found!");
                println!("      Accounts in this instruction:");
                for (j, account_idx) in instruction.accounts.iter().enumerate() {
                    let account = original_tx.message.account_keys[*account_idx as usize];
                    println!("        Account {}: {}", j, account);
                }
                println!("      Data length: {} bytes", instruction.data.len());
                if instruction.data.len() >= 8 {
                    println!("      Discriminator: {:02x?}", &instruction.data[0..8]);
                }

                // Show detailed analysis
                if instruction.accounts.len() >= 9 {
                    use crate::constants::{
                        get_gm_token_symbol, is_authorized_solver, is_gm_token,
                    };

                    let taker = original_tx.message.account_keys[instruction.accounts[0] as usize];
                    let maker = original_tx.message.account_keys[instruction.accounts[1] as usize];
                    let input_mint =
                        original_tx.message.account_keys[instruction.accounts[6] as usize];
                    let output_mint =
                        original_tx.message.account_keys[instruction.accounts[8] as usize];

                    println!("\n      Trade Analysis:");
                    println!("        Taker (user): {}", taker);
                    println!(
                        "        Maker (solver): {} {}",
                        maker,
                        if is_authorized_solver(&maker) {
                            "✓ AUTHORIZED"
                        } else {
                            "✗ UNAUTHORIZED"
                        }
                    );

                    let input_is_gm = is_gm_token(&input_mint);
                    let output_is_gm = is_gm_token(&output_mint);

                    println!(
                        "        Input mint: {} {}",
                        input_mint,
                        if input_is_gm {
                            format!("({})", get_gm_token_symbol(&input_mint).unwrap_or("GM"))
                        } else if input_mint.to_string()
                            == "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
                        {
                            "(USDC)".to_string()
                        } else {
                            "(Unknown)".to_string()
                        }
                    );

                    println!(
                        "        Output mint: {} {}",
                        output_mint,
                        if output_is_gm {
                            format!("({})", get_gm_token_symbol(&output_mint).unwrap_or("GM"))
                        } else if output_mint.to_string()
                            == "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
                        {
                            "(USDC)".to_string()
                        } else {
                            "(Unknown)".to_string()
                        }
                    );

                    println!("\n      Detection Criteria:");
                    println!(
                        "        {} Is GM trade (GM token involved)",
                        if input_is_gm || output_is_gm { "✓" } else { "✗" }
                    );
                    println!(
                        "        {} Maker is authorized",
                        if is_authorized_solver(&maker) {
                            "✓"
                        } else {
                            "✗"
                        }
                    );
                    println!(
                        "        {} Taker receives GM token (output)",
                        if output_is_gm { "✓" } else { "✗" }
                    );

                    if input_is_gm && !output_is_gm {
                        println!("\n      Trade Type: SELL (GM → USDC)");
                        println!("      Bundle Simulation: NOT REQUIRED");
                        println!("      Reason: Solver already has USDC, no minting needed");
                    } else if !input_is_gm && output_is_gm {
                        println!("\n      Trade Type: BUY (USDC → GM)");
                        println!("      Bundle Simulation: REQUIRED");
                        println!("      Reason: Solver needs GM tokens minted JIT");
                    }
                }
            }
        }

        // Strip the signatures to create an unsigned transaction
        println!("\nStripping signatures...");
        for sig in &mut original_tx.signatures {
            *sig = solana_sdk::signature::Signature::default();
        }
        println!("✓ Signatures stripped");

        // Test: Verify check_gm_trade identifies this correctly
        println!("\nChecking GM trade detection...");
        let result = check_gm_trade(&original_tx);

        match &result {
            Ok(check_result) if !check_result.use_gm_bundle_sim => {
                println!("\n  Result: GM trade detected, but bundle simulation NOT required");
                println!(
                    "         (SELL transactions don't need minting - solver already has USDC)"
                );
            }
            Ok(_) => {
                println!("\n  Result: GM trade detected, bundle simulation REQUIRED");
                println!("         (BUY transactions need minting - solver needs GM tokens)");
            }
            Err(e) => {
                println!("\n  ✗ Error checking GM trade: {:?}", e);
            }
        }

        let result = result.expect("Failed to check GM trade");

        if result.use_gm_bundle_sim {
            // This is a BUY transaction - proceed with full bundle simulation test
            let trade_info = result.trade_info.expect("Expected trade info");
            println!("\n✓ GM BUY Trade Confirmed:");
            println!("  Maker (solver): {}", trade_info.maker);
            println!("  Taker (user): {}", trade_info.taker);
            println!(
                "  GM Token: {} ({})",
                trade_info.gm_token_symbol, trade_info.gm_token_mint
            );
            println!("  Amount: {}", trade_info.gm_token_amount);
            println!("  Expire At: {}", trade_info.expire_at);

            // Build the mock mint transaction
            println!("\nBuilding mock mint transaction...");
            let mock_mint_tx = build_mock_mint_transaction(&trade_info, Hash::default());
            println!("✓ Mock mint transaction built");
            println!(
                "  Instructions: {}",
                mock_mint_tx.message.instructions.len()
            );

            // Simulate the bundle on mainnet
            println!("\nSimulating bundle on mainnet...");
            println!("  Simulating: [mock_mint_tx, original_fill_tx]");

            // Get a fresh blockhash for simulation
            println!("\n  Fetching fresh blockhash for simulation...");
            let fresh_blockhash = match client.get_latest_blockhash() {
                Ok(hash) => {
                    println!("  ✓ Got fresh blockhash: {}", hash);
                    hash
                }
                Err(e) => {
                    println!("  ✗ Failed to get blockhash: {:?}", e);
                    println!("  Cannot proceed with simulation");
                    Hash::default()
                }
            };

            // Reconstruct transactions with fresh blockhash
            let mock_mint_tx_fresh = {
                let mut msg = mock_mint_tx.message.clone();
                msg.recent_blockhash = fresh_blockhash;
                Transaction::new_unsigned(msg)
            };

            let original_tx_fresh = {
                let mut msg = original_tx.message.clone();
                msg.recent_blockhash = fresh_blockhash;

                // Update the expire_at field in the Jupiter fill instruction to prevent expiration errors
                for instruction in &mut msg.instructions {
                    let program_id = msg.account_keys[instruction.program_id_index as usize];
                    if program_id == jupiter_order_engine_program_id() {
                        if instruction.data.len() >= 32 {
                            // Set expire_at to 1 hour from now
                            let future_expire = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs() as i64
                                + 3600;
                            instruction.data[24..32].copy_from_slice(&future_expire.to_le_bytes());
                            println!("  Updated expire_at to: {}", future_expire);
                        }
                    }
                }

                Transaction::new_unsigned(msg)
            };

            // Use Jito bundle simulation via simulate_as_bundle
            println!("\n  Using Jito bundle simulation via simulate_as_bundle...");

            match simulate_as_bundle(
                vec![mock_mint_tx_fresh, original_tx_fresh],
                &trade_info,
                &rpc_url,
            ) {
                Ok(sim_result) => {
                    if sim_result.success {
                        println!("  ✓ Bundle simulation succeeded");
                    } else {
                        println!("  ✗ Bundle simulation failed: {:?}", sim_result.error);
                    }

                    // Print taker balance changes
                    println!("\n  Taker Balance Changes:");
                    if sim_result.taker_balance_changes.is_empty() {
                        println!("    (no balance changes detected)");
                    } else {
                        for change in &sim_result.taker_balance_changes {
                            let symbol = change.symbol.as_deref().unwrap_or("?");
                            let sign = if change.change >= 0 { "+" } else { "" };
                            println!(
                                "    {} {}: {}{:.6} (raw: {}{})",
                                change.token_account,
                                symbol,
                                sign,
                                change.change_display(),
                                sign,
                                change.change
                            );
                            println!(
                                "      Pre-balance:  {} ({:.6} {})",
                                change.pre_balance,
                                change.pre_balance as f64 / 10f64.powi(change.decimals as i32),
                                symbol
                            );
                            println!(
                                "      Post-balance: {} ({:.6} {})",
                                change.post_balance,
                                change.post_balance as f64 / 10f64.powi(change.decimals as i32),
                                symbol
                            );
                        }
                    }

                    // Print logs if available
                    if let Some(logs) = &sim_result.logs {
                        println!("\n    Logs ({} entries):", logs.len());
                        for log in logs.iter().take(10) {
                            println!("      {}", log);
                        }
                        if logs.len() > 10 {
                            println!("      ... and {} more", logs.len() - 10);
                        }
                    }
                }
                Err(e) => println!("  ✗ simulate_as_bundle failed: {:?}", e),
            }

            println!("\n{}", "=".repeat(80));
            println!("✓ MAINNET BUY TRANSACTION TEST COMPLETED");
            println!("  • Transaction fetched from mainnet");
            println!("  • GM trade detection working properly");
            println!("  • Mock mint transaction built");
            println!("  • Bundle simulation completed with balance changes");
            println!("{}", "=".repeat(80));
        } else {
            // This is a SELL transaction or non-GM trade - no bundle simulation needed
            println!("\n✓ Transaction validated - no bundle simulation required");
            println!("  This transaction can use standard single-transaction simulation.");

            println!("\n{}", "=".repeat(80));
            println!("✓ MAINNET SELL TRANSACTION TEST COMPLETED");
            println!("  • Transaction fetched from mainnet");
            println!("  • Correctly identified as not requiring bundle simulation");
            println!("  • SELL transactions don't need minting");
            println!("{}", "=".repeat(80));
        }
    }

    /// Test a base64-encoded transaction payload from a file.
    ///
    /// This test reads a base64-encoded transaction from `./payload` file and:
    /// 1. Decodes and deserializes the transaction
    /// 2. Checks if it's a GM trade
    /// 3. If yes, builds mock mint and simulates the bundle
    ///
    /// Run with: `RPC_URL=<your_rpc> cargo test test_payload_file -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn test_payload_file() {
        use base64::Engine;

        println!("{}", "=".repeat(80));
        println!("PAYLOAD FILE TEST");
        println!("{}", "=".repeat(80));

        // Read the payload file
        let payload_path = std::env::var("PAYLOAD_FILE").unwrap_or_else(|_| "payload".to_string());
        println!("Reading payload from: {}", payload_path);

        let base64_payload = std::fs::read_to_string(&payload_path)
            .expect("Failed to read payload file")
            .trim()
            .to_string();

        println!("Payload length: {} bytes (base64)", base64_payload.len());

        // Decode base64
        let tx_data = base64::engine::general_purpose::STANDARD
            .decode(&base64_payload)
            .expect("Failed to decode base64 payload");
        println!("Decoded transaction: {} bytes", tx_data.len());

        // Try to deserialize as VersionedTransaction first
        let versioned_tx: VersionedTransaction =
            bincode::deserialize(&tx_data).expect("Failed to deserialize as VersionedTransaction");

        println!(
            "Transaction type: {}",
            match &versioned_tx.message {
                solana_sdk::message::VersionedMessage::Legacy(_) => "Legacy",
                solana_sdk::message::VersionedMessage::V0(_) => "V0 (versioned)",
            }
        );

        // Check if it's a GM trade using versioned check
        println!("\nChecking GM trade detection...");
        let result = check_gm_trade_versioned(&versioned_tx);

        match &result {
            Ok(check_result) if check_result.use_gm_bundle_sim => {
                println!("✓ GM BUY trade detected - bundle simulation REQUIRED");
            }
            Ok(check_result) if !check_result.use_gm_bundle_sim => {
                println!("✓ GM SELL trade or non-GM trade - bundle simulation NOT required");
            }
            Ok(_) => {}
            Err(e) => {
                println!("✗ Error checking GM trade: {:?}", e);
                panic!("GM trade check failed");
            }
        }

        let result = result.expect("Failed to check GM trade");

        if result.use_gm_bundle_sim {
            let trade_info = result.trade_info.expect("Expected trade info");
            println!("\n✓ GM BUY Trade Details:");
            println!("  Maker (solver): {}", trade_info.maker);
            println!("  Taker (user): {}", trade_info.taker);
            println!(
                "  GM Token: {} ({})",
                trade_info.gm_token_symbol, trade_info.gm_token_mint
            );
            println!(
                "  Amount: {} ({:.6} {})",
                trade_info.gm_token_amount,
                trade_info.gm_token_amount as f64 / 1_000_000_000.0,
                trade_info.gm_token_symbol
            );
            println!("  Expire At: {}", trade_info.expire_at);

            // Initialize RPC client
            let rpc_url = std::env::var("RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
            println!("\nUsing RPC: {}", rpc_url);

            // Convert versioned tx to legacy for simulation (EXACT payload - no modifications)
            let original_tx: Transaction = match versioned_tx.message {
                solana_sdk::message::VersionedMessage::Legacy(legacy_msg) => {
                    let mut tx = Transaction::new_unsigned(legacy_msg);
                    tx.signatures = versioned_tx.signatures;
                    tx
                }
                solana_sdk::message::VersionedMessage::V0(v0_msg) => {
                    // Convert V0 to legacy (note: this loses lookup table info)
                    let legacy_msg = Message {
                        header: v0_msg.header,
                        account_keys: v0_msg.account_keys,
                        recent_blockhash: v0_msg.recent_blockhash,
                        instructions: v0_msg.instructions,
                    };
                    let mut tx = Transaction::new_unsigned(legacy_msg);
                    tx.signatures = versioned_tx.signatures;
                    tx
                }
            };

            // Use the EXACT blockhash from the original transaction
            let original_blockhash = original_tx.message.recent_blockhash;
            println!("\nUsing EXACT transaction payload (no modifications):");
            println!("  Blockhash from payload: {}", original_blockhash);
            println!("  Expire At from payload: {}", trade_info.expire_at);

            // Build the mock mint transaction with the SAME blockhash as the original
            println!("\nBuilding mock mint transaction...");
            let mock_mint_tx = build_mock_mint_transaction(&trade_info, original_blockhash);
            println!(
                "✓ Mock mint transaction built ({} instructions)",
                mock_mint_tx.message.instructions.len()
            );

            // Simulate the bundle with EXACT original transaction
            println!("\nSimulating bundle via Jito...");
            println!("  Bundle: [mock_mint_tx, original_fill_tx (unchanged)]");

            match simulate_as_bundle(
                vec![mock_mint_tx, original_tx],
                &trade_info,
                &rpc_url,
            ) {
                Ok(sim_result) => {
                    if sim_result.success {
                        println!("\n✓ Bundle simulation SUCCEEDED!");
                    } else {
                        println!(
                            "\n✗ Bundle simulation failed: {:?}",
                            sim_result.error.as_deref().unwrap_or("unknown error")
                        );
                    }

                    // Print taker balance changes
                    println!("\nTaker Balance Changes:");
                    if sim_result.taker_balance_changes.is_empty() {
                        println!("  (no balance changes detected)");
                    } else {
                        for change in &sim_result.taker_balance_changes {
                            let symbol = change.symbol.as_deref().unwrap_or("?");
                            let sign = if change.change >= 0 { "+" } else { "" };
                            println!(
                                "  {} {}: {}{:.6}",
                                change.token_account,
                                symbol,
                                sign,
                                change.change_display()
                            );
                            println!(
                                "    Pre:  {:.6} {} (raw: {})",
                                change.pre_balance as f64 / 10f64.powi(change.decimals as i32),
                                symbol,
                                change.pre_balance
                            );
                            println!(
                                "    Post: {:.6} {} (raw: {})",
                                change.post_balance as f64 / 10f64.powi(change.decimals as i32),
                                symbol,
                                change.post_balance
                            );
                        }
                    }

                    // Print some logs
                    if let Some(logs) = &sim_result.logs {
                        println!("\nSimulation Logs ({} entries):", logs.len());
                        for log in logs.iter().take(15) {
                            println!("  {}", log);
                        }
                        if logs.len() > 15 {
                            println!("  ... and {} more", logs.len() - 15);
                        }
                    }
                }
                Err(e) => {
                    println!("\n✗ simulate_as_bundle error: {:?}", e);
                }
            }

            println!("\n{}", "=".repeat(80));
            println!("✓ PAYLOAD FILE TEST COMPLETED");
            println!("{}", "=".repeat(80));
        } else {
            println!("\n✓ Transaction does not require bundle simulation");
            println!("  This can use standard single-transaction simulation.");
            println!("\n{}", "=".repeat(80));
            println!("✓ PAYLOAD FILE TEST COMPLETED (NO BUNDLE SIM NEEDED)");
            println!("{}", "=".repeat(80));
        }
    }

    /// Regression test for the `mint_gm` layout fallback against a live program.
    ///
    /// Run with: `RPC_URL=<jito_rpc> cargo test test_mint_layout_fallback -- --ignored --nocapture`
    ///
    /// Reports which layout the deployed GM program accepts, then asserts that
    /// `simulate_as_bundle` succeeds and credits the taker either way. Passing on
    /// both sides of the program upgrade is the zero-downtime guarantee.
    #[test]
    #[ignore]
    fn test_mint_layout_fallback() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::instruction::{AccountMeta, Instruction};
        use crate::mint_instruction::get_gm_token_ata;

        let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be a Jito-enabled RPC");
        let blockhash = RpcClient::new(rpc_url.clone())
            .get_latest_blockhash()
            .expect("Failed to get blockhash");

        let taker = Pubkey::from_str("7z86y3WYofAiuxhQvYV2U6ZQMQ7jLxncuyV9U7D8PwYV").unwrap(); // Funded with USDC
        let maker = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap(); // Real solver
        let aapl_mint = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();
        let gm_amount = 3_880_411u64;
        let expire_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 3600;

        // BUY: 1 USDC -> AAPLon, same shape as test_from_scratch
        let mut data = vec![0xa8, 0x60, 0xb7, 0xa3, 0x5c, 0x0a, 0x28, 0xa0];
        data.extend_from_slice(&1_000_000u64.to_le_bytes());
        data.extend_from_slice(&gm_amount.to_le_bytes());
        data.extend_from_slice(&expire_at.to_le_bytes());
        let fill_ix = Instruction {
            program_id: jupiter_order_engine_program_id(),
            accounts: vec![
                AccountMeta::new(taker, true),
                AccountMeta::new(maker, true),
                AccountMeta::new(spl_associated_token_account::get_associated_token_address(&taker, &usdc_mint()), false),
                AccountMeta::new(spl_associated_token_account::get_associated_token_address(&maker, &usdc_mint()), false),
                AccountMeta::new(get_gm_token_ata(&taker, &aapl_mint), false),
                AccountMeta::new(get_gm_token_ata(&maker, &aapl_mint), false),
                AccountMeta::new_readonly(usdc_mint(), false),
                AccountMeta::new_readonly(crate::constants::spl_token_program_id(), false),
                AccountMeta::new_readonly(aapl_mint, false),
                AccountMeta::new_readonly(crate::constants::token_2022_program_id(), false),
                AccountMeta::new_readonly(solana_system_interface::program::id(), false),
            ],
            data,
        };
        let buy_tx = Transaction::new_unsigned(Message::new_with_blockhash(&[fill_ix], Some(&taker), &blockhash));

        let trade_info = check_gm_trade(&buy_tx)
            .expect("Failed to check GM trade")
            .trade_info
            .expect("Expected a GM BUY trade");
        let mock_mint_tx = build_mock_mint_transaction(&trade_info, blockhash);

        // Probe the default layout on its own so the output records which program is live
        let tracked = [get_gm_token_ata(&taker, &aapl_mint)];
        let probe = request_simulate_bundle(&[mock_mint_tx.clone(), buy_tx.clone()], &tracked, &rpc_url)
            .expect("simulateBundle request failed");
        if mock_mint_failed(&probe) {
            println!("Deployed GM program: POST-upgrade (default layout rejected, fallback required)");
        } else {
            println!("Deployed GM program: PRE-upgrade (default layout accepted)");
        }

        let result = simulate_as_bundle(vec![mock_mint_tx, buy_tx], &trade_info, &rpc_url)
            .expect("simulate_as_bundle errored");
        assert!(result.success, "Bundle simulation failed: {:?}", result.error);

        let gm_change = result
            .taker_balance_changes
            .iter()
            .find(|c| c.mint == aapl_mint)
            .expect("Missing taker GM balance change");
        assert_eq!(gm_change.change, gm_amount as i128);
        println!("✓ Taker received {} AAPLon base units", gm_change.change);
    }
}
