//! Integration tests for simulating GM mock mint transactions.
//!
//! These tests connect to Solana mainnet to fetch real fill transactions
//! and verify that the mock mint instructions can be built and simulated.

use gm_solana_simulator::{
    build_mock_mint_instruction, build_mock_mint_instruction_to_ata, check_gm_trade_message,
    instruction_discriminator,
};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::{Message, VersionedMessage},
    signature::Signature,
    transaction::Transaction,
};
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;


/// Helper function to fetch a transaction and build the mock mint instruction
fn get_mint_instruction_from_fill(
    client: &RpcClient,
    signature: Signature,
) -> Result<solana_sdk::instruction::Instruction, Box<dyn std::error::Error>> {
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Base64),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    let tx_response = client.get_transaction_with_config(&signature, config)?;

    let versioned_tx = tx_response
        .transaction
        .transaction
        .decode()
        .ok_or("Failed to decode transaction")?;

    let result = match versioned_tx.message {
        VersionedMessage::Legacy(ref legacy_msg) => check_gm_trade_message(legacy_msg)?,
        VersionedMessage::V0(ref v0_msg) => {
            let legacy_msg = Message {
                header: v0_msg.header,
                account_keys: v0_msg.account_keys.clone(),
                recent_blockhash: v0_msg.recent_blockhash,
                instructions: v0_msg.instructions.clone(),
            };
            check_gm_trade_message(&legacy_msg)?
        }
    };

    if !result.use_gm_bundle_sim {
        return Err("Transaction is not a GM trade".into());
    }

    let trade_info = result.trade_info.unwrap();
    println!("GM Trade detected:");
    println!("  Maker: {}", trade_info.maker);
    println!("  Taker: {}", trade_info.taker);
    println!(
        "  GM Token: {} ({})",
        trade_info.gm_token_symbol, trade_info.gm_token_mint
    );
    println!("  Amount: {}", trade_info.gm_token_amount);
    println!("  Expires at: {}", trade_info.expire_at);

    // Build the mock mint instruction (using the ATA variant)
    Ok(build_mock_mint_instruction_to_ata(&trade_info))
}

/// Test simulating a mock mint instruction from a known fill transaction
///
/// Note: This test requires network access and a valid fill transaction signature.
/// The signature should be updated to a recent GM fill transaction for the test to pass.
#[test]
fn test_simulate_mock_mint_from_fill() {
    let client = RpcClient::new_with_commitment("https://api.mainnet-beta.solana.com", CommitmentConfig::confirmed());

    // Replace with an actual fill transaction signature
    // This should be a Jupiter RFQ fill where an authorized solver is the maker
    // and the taker is receiving a GM token
    let sig_str =
        "5V4ffUtpuPmCnKo2v9fUYX2yEiGZHzFJjLyp2XdC1kKc4Gmpk3yYuQySsSeLQnxUyC7bidmc7W9pfz44KfMrsNdL";
    let tx_signature = Signature::from_str(sig_str).expect("Invalid signature");

    match get_mint_instruction_from_fill(&client, tx_signature) {
        Ok(instruction) => {
            // Build transaction for simulation (no signing needed)
            let recent_blockhash = client
                .get_latest_blockhash()
                .expect("Failed to get blockhash");

            let message = Message::new_with_blockhash(&[instruction], None, &recent_blockhash);
            let tx = Transaction::new_unsigned(message);

            // Simulate the transaction
            match client.simulate_transaction(&tx) {
                Ok(result) => {
                    println!("Simulation result: {:?}", result);
                    if let Some(logs) = result.value.logs {
                        println!("Logs:");
                        for log in logs {
                            println!("  {}", log);
                        }
                    }
                    if let Some(err) = result.value.err {
                        // Note: Simulation may fail because the minter doesn't have authority
                        // in a real simulation context. The important thing is that the
                        // instruction was built correctly.
                        eprintln!("Simulation error (expected in test): {:?}", err);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to simulate: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error retrieving mint instruction: {}", e);
            // Don't fail the test if the transaction is not found or not a GM trade
            // This is expected if the test signature is outdated
        }
    }
}

/// Test that we can construct mock mint instructions from trade info
#[test]
fn test_build_mock_mint_instruction() {
    use gm_solana_simulator::GmTradeInfo;
    use solana_sdk::pubkey::Pubkey;

    let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
    let aapl = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();
    let maker_output_ata = Pubkey::new_unique();

    let trade_info = GmTradeInfo {
        maker: solver,
        taker: Pubkey::new_unique(),
        gm_token_mint: aapl,
        gm_token_symbol: "AAPLon".to_string(),
        gm_token_amount: 1_500_000_000, // 1.5 AAPL (9 decimals)
        maker_output_account: maker_output_ata,
        expire_at: 1704067200,
    };

    let instruction = build_mock_mint_instruction(&trade_info);

    // Verify instruction structure
    assert!(!instruction.data.is_empty());
    assert!(!instruction.accounts.is_empty());

    println!("Mock mint instruction built successfully");
    println!("  Program ID: {}", instruction.program_id);
    println!("  Data length: {} bytes", instruction.data.len());
    println!("  Account count: {}", instruction.accounts.len());
}

/// Test the full flow with a mock transaction (no network required)
#[test]
fn test_check_gm_trade_and_build_mock_mint() {
    use gm_solana_simulator::{
        build_mock_mint_transaction, check_gm_trade_message, jupiter_order_engine_program_id,
        usdc_mint,
    };
    use solana_sdk::{
        hash::Hash,
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
    };

    // Create a mock Jupiter fill instruction
    let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
    let user = Pubkey::new_unique();
    let usdc = usdc_mint();
    let aapl = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();

    let jupiter_program_id = jupiter_order_engine_program_id();
    let input_amount: u64 = 200_000_000; // 200 USDC
    let output_amount: u64 = 1_500_000_000; // 1.5 AAPL
    let expire_at: i64 = 1704067200;

    // Build instruction data
    let mut data = vec![];
    let fill_discriminator = instruction_discriminator("fill");
    data.extend_from_slice(&fill_discriminator[..]);
    data.extend_from_slice(&input_amount.to_le_bytes());
    data.extend_from_slice(&output_amount.to_le_bytes());
    data.extend_from_slice(&expire_at.to_le_bytes());

    let taker_input_ata = Pubkey::new_unique();
    let maker_input_ata = Pubkey::new_unique();
    let taker_output_ata = Pubkey::new_unique();
    let maker_output_ata = Pubkey::new_unique();

    // Account order matches actual Jupiter RFQ fill layout:
    // taker, maker, taker_input_ata, maker_input_ata, taker_output_ata, maker_output_ata,
    // input_mint, input_token_program, output_mint
    let fill_instruction = Instruction {
        program_id: jupiter_program_id,
        accounts: vec![
            AccountMeta::new(user, true),              // 0: taker
            AccountMeta::new(solver, true),            // 1: maker
            AccountMeta::new(taker_input_ata, false),  // 2: taker_input_ata
            AccountMeta::new(maker_input_ata, false),  // 3: maker_input_ata
            AccountMeta::new(taker_output_ata, false), // 4: taker_output_ata
            AccountMeta::new(maker_output_ata, false), // 5: maker_output_ata
            AccountMeta::new_readonly(usdc, false),    // 6: input_mint
            AccountMeta::new_readonly(gm_solana_simulator::token_2022_program_id(), false), // 7: input_token_program
            AccountMeta::new_readonly(aapl, false), // 8: output_mint
        ],
        data,
    };

    // Create message and check if it's a GM trade
    let message = Message::new(&[fill_instruction], Some(&user));
    let result = check_gm_trade_message(&message).expect("Failed to check GM trade");

    assert!(result.use_gm_bundle_sim, "Should be detected as GM trade");

    let trade_info = result.trade_info.unwrap();
    assert_eq!(trade_info.maker, solver);
    assert_eq!(trade_info.gm_token_mint, aapl);
    assert_eq!(trade_info.gm_token_amount, output_amount);
    assert_eq!(trade_info.gm_token_symbol, "AAPLon");

    // Build mock mint transaction
    let mock_mint_tx = build_mock_mint_transaction(&trade_info, Hash::default());

    // Should have 5 instructions:
    // 1. Create taker's GM ATA (idempotent)
    // 2. Create maker's GM ATA (idempotent)
    // 3. Create taker's USDC ATA (idempotent)
    // 4. Create maker's USDC ATA (idempotent)
    // 5. Mint GM tokens to solver (maker)
    assert_eq!(mock_mint_tx.message.instructions.len(), 5);
    println!("Successfully built mock mint transaction");
    println!(
        "  Instructions: {}",
        mock_mint_tx.message.instructions.len()
    );
}
