//! Parser for Jupiter Order Engine fill instructions.
//!
//! The Jupiter Order Engine program uses a specific instruction layout for RFQ fills.
//! This module parses those instructions to extract the relevant trade details.

use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};

use crate::{
    constants::{get_gm_token_symbol, is_authorized_solver, is_gm_token},
    instruction_discriminator,
    types::{GmSimulatorError, GmTradeInfo},
};

/// Jupiter Order Engine "fill" instruction discriminator
/// This is the first 8 bytes of the instruction data for a fill
/// Verified from Jupiter Order Engine on-chain program
///
/// Account indices in the Jupiter Order Engine fill instruction
/// Based on actual on-chain transaction analysis (verified from mainnet)
///
/// Layout: taker, maker, taker_input_ata, maker_input_ata, taker_output_ata, maker_output_ata,
///         input_mint, input_token_program, output_mint, output_token_program, system_program
mod account_indices {
    pub const TAKER: usize = 0; // Signer, user
    pub const MAKER: usize = 1; // Signer, market maker (solver)
    #[allow(dead_code)]
    pub const TAKER_INPUT_ATA: usize = 2; // Taker's input token account
    #[allow(dead_code)]
    pub const MAKER_INPUT_ATA: usize = 3; // Maker's input token account
    #[allow(dead_code)]
    pub const TAKER_OUTPUT_ATA: usize = 4; // Taker's output token account (receives GM tokens)
    pub const MAKER_OUTPUT_ATA: usize = 5; // Maker's output token account (receives USDC)
    #[allow(dead_code)]
    pub const INPUT_MINT: usize = 6; // Input token mint
    #[allow(dead_code)]
    pub const INPUT_TOKEN_PROGRAM: usize = 7; // Input token program
    pub const OUTPUT_MINT: usize = 8; // Output token mint (GM token)
}

/// Check if an instruction is a Jupiter Order Engine fill
pub fn is_jupiter_fill_instruction(
    instruction: &CompiledInstruction,
    program_id: &Pubkey,
    account_keys: &[Pubkey],
) -> bool {
    // Check program ID matches Jupiter Order Engine
    let ix_program_id = account_keys
        .get(instruction.program_id_index as usize)
        .cloned();

    if ix_program_id != Some(*program_id) {
        return false;
    }

    // Check discriminator
    if instruction.data.len() < 8 {
        return false;
    }

    let ix_discriminator = instruction_discriminator("fill");

    ix_discriminator == instruction.data[..8]
}

/// Parse a Jupiter Order Engine fill instruction and extract GM trade info
///
/// Returns Ok(Some(GmTradeInfo)) if this is a valid GM trade
/// Returns Ok(None) if this is a Jupiter fill but not a GM trade
/// Returns Err if parsing fails
pub fn parse_fill_for_gm_trade(
    instruction: &CompiledInstruction,
    account_keys: &[Pubkey],
) -> Result<Option<GmTradeInfo>, GmSimulatorError> {
    // Validate instruction data length
    // Discriminator (8) + input_amount (8) + output_amount (8) + expire_at (8) = 32 minimum
    if instruction.data.len() < 32 {
        return Err(GmSimulatorError::InstructionParseError(format!(
            "Instruction data too short: expected at least 32 bytes, got {}",
            instruction.data.len()
        )));
    }

    // Helper to get account pubkey from instruction accounts
    let get_account = |idx: usize| -> Result<Pubkey, GmSimulatorError> {
        let account_idx = instruction
            .accounts
            .get(idx)
            .ok_or(GmSimulatorError::InvalidAccountIndex)?;
        account_keys
            .get(*account_idx as usize)
            .cloned()
            .ok_or(GmSimulatorError::MissingAccount)
    };

    // Extract accounts
    let maker = get_account(account_indices::MAKER)?;
    let taker = get_account(account_indices::TAKER)?;
    let maker_output_account = get_account(account_indices::MAKER_OUTPUT_ATA)?;
    let output_mint = get_account(account_indices::OUTPUT_MINT)?;

    // Check 1: Is maker an authorized solver?
    if !is_authorized_solver(&maker) {
        return Err(GmSimulatorError::UnauthorizedMaker(maker));
    }

    // Check 2: Is output_mint (what taker receives) a GM token?
    if !is_gm_token(&output_mint) {
        return Ok(None); // Valid Jupiter fill, but not a GM trade
    }

    // Parse fill instruction arguments
    // Data layout: discriminator (8) + input_amount (8) + output_amount (8) + expire_at (8)
    let output_amount = u64::from_le_bytes(instruction.data[16..24].try_into().map_err(|_| {
        GmSimulatorError::InstructionParseError("Invalid output amount".to_string())
    })?);

    let expire_at = i64::from_le_bytes(instruction.data[24..32].try_into().map_err(|_| {
        GmSimulatorError::InstructionParseError("Invalid expire_at timestamp".to_string())
    })?);

    // Get GM token symbol
    let gm_token_symbol = get_gm_token_symbol(&output_mint)
        .unwrap_or("GM")
        .to_string();

    Ok(Some(GmTradeInfo {
        maker,
        taker,
        gm_token_mint: output_mint,
        gm_token_symbol,
        gm_token_amount: output_amount,
        maker_output_account,
        expire_at,
    }))
}
