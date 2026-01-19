//! Build mock mint_gm instructions for bundle simulation.
//!
//! The `mint_gm` instruction is an admin mint that doesn't require attestations,
//! making it suitable for simulation purposes.

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::constants::{admin_minter, ondo_gm_program_id, token_2022_program_id};

/// Anchor discriminator for "mint_gm" instruction
/// Verified from on-chain IDL at XzTT4XB8m7sLD2xi6snefSasaswsKCxx5Tifjondogm
const MINT_GM_DISCRIMINATOR: [u8; 8] = [117, 223, 58, 111, 44, 36, 16, 43];

/// PDA seeds (verified from Ondo GM program source)
const MINT_AUTHORITY_SEED: &[u8] = b"mint_authority";
const MINTER_ROLE_GMTOKEN_SEED: &[u8] = b"MinterRoleGMToken";
const ORACLE_SANITY_CHECK_SEED: &[u8] = b"sanity_check";
const USDON_MANAGER_STATE_SEED: &[u8] = b"usdon_manager";

/// Build a mock mint_gm instruction for simulation.
///
/// This creates an admin mint instruction that will mint GM tokens to the
/// specified destination account. Uses the real admin minter account which
/// has MINTER_ROLE_GMTOKEN and the necessary on-chain permissions.
///
/// # Account Structure (verified from on-chain program)
///
/// The mint_gm instruction requires these accounts in order:
/// 1. payer (signer, writable) - pays for destination ATA if needed
/// 2. authority (signer) - the minter with MINTER_ROLE_GMTOKEN
/// 3. user (unchecked) - the recipient wallet owner
/// 4. authority_role_account (PDA) - verifies MINTER_ROLE_GMTOKEN
/// 5. oracle_sanity_check (writable, PDA) - validates oracle price updates
/// 6. mint_authority (PDA) - the mint authority PDA
/// 7. mint (writable) - the GM token mint
/// 8. destination (writable, init_if_needed ATA) - destination token account
/// 9. usdon_manager_state (PDA) - manager state for validation
/// 10. token_program - Token-2022
/// 11. associated_token_program - ATA program
/// 12. system_program - System program
///
/// # Arguments
///
/// * `gm_token_mint` - The GM token mint address
/// * `destination_owner` - The wallet that will own the minted tokens (the solver)
/// * `amount` - Amount of tokens to mint (in base units, 9 decimals)
///
/// # Returns
///
/// An unsigned `Instruction` that can be used for bundle simulation
pub fn build_mock_mint_gm_instruction(
    gm_token_mint: &Pubkey,
    destination_owner: &Pubkey,
    amount: u64,
) -> Instruction {
    let program_id = ondo_gm_program_id();
    let minter = admin_minter();
    let token_program = token_2022_program_id();

    // Derive PDAs with verified seeds
    let (authority_role_account, _) =
        Pubkey::find_program_address(&[MINTER_ROLE_GMTOKEN_SEED, minter.as_ref()], &program_id);

    let (oracle_sanity_check, _) = Pubkey::find_program_address(
        &[ORACLE_SANITY_CHECK_SEED, gm_token_mint.as_ref()],
        &program_id,
    );

    let (mint_authority, _) = Pubkey::find_program_address(&[MINT_AUTHORITY_SEED], &program_id);

    let (usdon_manager_state, _) =
        Pubkey::find_program_address(&[USDON_MANAGER_STATE_SEED], &program_id);

    // Get the destination ATA (Token-2022)
    let destination_ata = get_associated_token_address_with_program_id(
        destination_owner,
        gm_token_mint,
        &token_program,
    );

    // Build instruction data: discriminator + amount
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&MINT_GM_DISCRIMINATOR);
    data.extend_from_slice(&amount.to_le_bytes());

    // Build accounts list in the exact order from the on-chain IDL
    let accounts = vec![
        AccountMeta::new(minter, true),          // 0: payer (signer, writable)
        AccountMeta::new_readonly(minter, true), // 1: authority (signer)
        AccountMeta::new_readonly(*destination_owner, false), // 2: user (recipient)
        AccountMeta::new_readonly(authority_role_account, false), // 3: authority_role_account PDA
        AccountMeta::new(oracle_sanity_check, false), // 4: oracle_sanity_check PDA (writable)
        AccountMeta::new_readonly(mint_authority, false), // 5: mint_authority PDA
        AccountMeta::new(*gm_token_mint, false), // 6: mint (writable)
        AccountMeta::new(destination_ata, false), // 7: destination ATA (writable)
        AccountMeta::new_readonly(usdon_manager_state, false), // 8: usdon_manager_state PDA
        AccountMeta::new_readonly(token_program, false), // 9: token_program (Token-2022)
        AccountMeta::new_readonly(spl_associated_token_account::id(), false), // 10: ATA program
        AccountMeta::new_readonly(system_program::id(), false), // 11: system_program
    ];

    Instruction {
        program_id,
        accounts,
        data,
    }
}

/// Build a mock mint_gm instruction using a specific destination ATA and owner.
///
/// This is useful when you already have the destination ATA computed and want
/// to avoid deriving it again.
///
/// # Arguments
///
/// * `gm_token_mint` - The GM token mint address
/// * `destination_ata` - The pre-computed destination token account (ATA)
/// * `destination_owner` - The owner of the destination ATA (must match for constraint)
/// * `amount` - Amount of tokens to mint (in base units, 9 decimals)
pub fn build_mock_mint_gm_instruction_with_ata(
    gm_token_mint: &Pubkey,
    destination_ata: &Pubkey,
    destination_owner: &Pubkey,
    amount: u64,
) -> Instruction {
    let program_id = ondo_gm_program_id();
    let minter = admin_minter();
    let token_program = token_2022_program_id();

    // Derive PDAs with verified seeds
    let (authority_role_account, _) =
        Pubkey::find_program_address(&[MINTER_ROLE_GMTOKEN_SEED, minter.as_ref()], &program_id);

    let (oracle_sanity_check, _) = Pubkey::find_program_address(
        &[ORACLE_SANITY_CHECK_SEED, gm_token_mint.as_ref()],
        &program_id,
    );

    let (mint_authority, _) = Pubkey::find_program_address(&[MINT_AUTHORITY_SEED], &program_id);

    let (usdon_manager_state, _) =
        Pubkey::find_program_address(&[USDON_MANAGER_STATE_SEED], &program_id);

    // Build instruction data: discriminator + amount
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&MINT_GM_DISCRIMINATOR);
    data.extend_from_slice(&amount.to_le_bytes());

    // Build accounts list - using destination_ata directly with correct owner
    let accounts = vec![
        AccountMeta::new(minter, true),          // 0: payer (signer, writable)
        AccountMeta::new_readonly(minter, true), // 1: authority (signer)
        AccountMeta::new_readonly(*destination_owner, false), // 2: user (destination owner)
        AccountMeta::new_readonly(authority_role_account, false), // 3: authority_role_account PDA
        AccountMeta::new(oracle_sanity_check, false), // 4: oracle_sanity_check PDA (writable)
        AccountMeta::new_readonly(mint_authority, false), // 5: mint_authority PDA
        AccountMeta::new(*gm_token_mint, false), // 6: mint (writable)
        AccountMeta::new(*destination_ata, false), // 7: destination ATA (writable)
        AccountMeta::new_readonly(usdon_manager_state, false), // 8: usdon_manager_state PDA
        AccountMeta::new_readonly(token_program, false), // 9: token_program (Token-2022)
        AccountMeta::new_readonly(spl_associated_token_account::id(), false), // 10: ATA program
        AccountMeta::new_readonly(system_program::id(), false), // 11: system_program
    ];

    Instruction {
        program_id,
        accounts,
        data,
    }
}

/// Get the expected destination ATA for a GM token mint.
///
/// GM tokens use Token-2022, so this derives the ATA using the Token-2022 program.
pub fn get_gm_token_ata(owner: &Pubkey, gm_token_mint: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(owner, gm_token_mint, &token_2022_program_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_build_mock_mint_instruction() {
        let gm_token = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let amount = 1_500_000_000u64; // 1.5 tokens

        let ix = build_mock_mint_gm_instruction(&gm_token, &solver, amount);

        assert_eq!(ix.program_id, ondo_gm_program_id());
        assert!(!ix.accounts.is_empty());
        assert!(ix.data.len() >= 16); // discriminator + amount
    }

    #[test]
    fn test_get_gm_token_ata() {
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let ata = get_gm_token_ata(&owner, &mint);

        // Verify it's a valid derived address
        assert_ne!(ata, owner);
        assert_ne!(ata, mint);
    }
}
