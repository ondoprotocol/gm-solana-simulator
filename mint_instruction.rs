//! Build mock mint_gm / mint_portfolio instructions for bundle simulation.
//!
//! The `mint_gm` and `mint_portfolio` instructions are admin mints that don't
//! require attestations, making them suitable for simulation purposes.

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::constants::{admin_minter, ondo_gm_program_id, token_2022_program_id};

/// Anchor discriminator for "mint_gm" instruction
/// Verified from on-chain IDL at XzTT4XB8m7sLD2xi6snefSasaswsKCxx5Tifjondogm
const MINT_GM_DISCRIMINATOR: [u8; 8] = [117, 223, 58, 111, 44, 36, 16, 43];

/// Anchor discriminator for "mint_portfolio" instruction: sha256("global:mint_portfolio")[..8]
const MINT_PORTFOLIO_DISCRIMINATOR: [u8; 8] = [203, 66, 164, 209, 77, 35, 140, 206];

/// PDA seeds (verified from Ondo GM program source)
const MINT_AUTHORITY_SEED: &[u8] = b"mint_authority";
const MINTER_ROLE_GMTOKEN_SEED: &[u8] = b"MinterRoleGMToken";
const ORACLE_SANITY_CHECK_SEED: &[u8] = b"sanity_check";
const USDON_MANAGER_STATE_SEED: &[u8] = b"usdon_manager";
const TOKEN_LIMIT_ACCOUNT_SEED: &[u8] = b"token";
const GMTOKEN_MANAGER_STATE_SEED: &[u8] = b"gmtoken_manager";
const MINTER_ROLE_PORTFOLIO_TOKEN_SEED: &[u8] = b"MinterRolePortfolioToken";
const PORTFOLIO_MINT_AUTHORITY_SEED: &[u8] = b"portfolio_mint_authority";
const PORTFOLIO_TOKEN_MANAGER_STATE_SEED: &[u8] = b"portfolio_token_manager";

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
/// 5. oracle_sanity_check (PDA) - supplies the price for the notional cap
/// 6. token_limit (PDA) - per-mint pause flag
/// 7. mint_authority (PDA) - the mint authority PDA
/// 8. mint (writable) - the GM token mint
/// 9. destination (writable, init_if_needed ATA) - destination token account
/// 10. usdon_manager_state (PDA) - manager state for validation
/// 11. gmtoken_manager (PDA) - global GM minting pause flag
/// 12. token_program - Token-2022
/// 13. associated_token_program - ATA program
/// 14. system_program - System program
///
/// Until the program upgrade lands on mainnet the deployed program still
/// expects the pre-upgrade layout; see `build_legacy_mock_mint_gm_instruction`.
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
    let destination_ata = get_gm_token_ata(destination_owner, gm_token_mint);
    build_mock_mint_gm_instruction_with_ata(gm_token_mint, &destination_ata, destination_owner, amount)
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
    build_mock_mint_instruction_for(
        TokenKind::Gm,
        gm_token_mint,
        destination_ata,
        destination_owner,
        amount,
    )
}

/// Build a mock mint_portfolio instruction for simulation.
///
/// Portfolio tokens are minted by the GM program's `mint_portfolio` instruction,
/// which takes the same 14-account layout as `mint_gm` but checks
/// MINTER_ROLE_PORTFOLIO_TOKEN and uses the portfolio mint authority and
/// manager-state PDAs. The admin minter is assumed to hold that role.
///
/// # Arguments
///
/// * `portfolio_token_mint` - The portfolio token mint address
/// * `destination_owner` - The wallet that will own the minted tokens (the solver)
/// * `amount` - Amount of tokens to mint (in base units, 9 decimals)
pub fn build_mock_mint_portfolio_instruction(
    portfolio_token_mint: &Pubkey,
    destination_owner: &Pubkey,
    amount: u64,
) -> Instruction {
    let destination_ata = get_gm_token_ata(destination_owner, portfolio_token_mint);
    build_mock_mint_portfolio_instruction_with_ata(
        portfolio_token_mint,
        &destination_ata,
        destination_owner,
        amount,
    )
}

/// Build a mock mint_portfolio instruction using a specific destination ATA and owner.
///
/// See `build_mock_mint_gm_instruction_with_ata` for the argument semantics.
pub fn build_mock_mint_portfolio_instruction_with_ata(
    portfolio_token_mint: &Pubkey,
    destination_ata: &Pubkey,
    destination_owner: &Pubkey,
    amount: u64,
) -> Instruction {
    build_mock_mint_instruction_for(
        TokenKind::Portfolio,
        portfolio_token_mint,
        destination_ata,
        destination_owner,
        amount,
    )
}

// ==============================================================================
// Shared GM / Portfolio Mint Layout
// ==============================================================================

/// Mirrors the program's `TokenKind`. `GMTokenMinter` and `PortfolioTokenMinter`
/// have identical account layouts; only the discriminator and three PDA seeds differ.
#[derive(Clone, Copy)]
enum TokenKind {
    Gm,
    Portfolio,
}

impl TokenKind {
    fn discriminator(self) -> [u8; 8] {
        match self {
            TokenKind::Gm => MINT_GM_DISCRIMINATOR,
            TokenKind::Portfolio => MINT_PORTFOLIO_DISCRIMINATOR,
        }
    }

    fn minter_role_seed(self) -> &'static [u8] {
        match self {
            TokenKind::Gm => MINTER_ROLE_GMTOKEN_SEED,
            TokenKind::Portfolio => MINTER_ROLE_PORTFOLIO_TOKEN_SEED,
        }
    }

    fn mint_authority_seed(self) -> &'static [u8] {
        match self {
            TokenKind::Gm => MINT_AUTHORITY_SEED,
            TokenKind::Portfolio => PORTFOLIO_MINT_AUTHORITY_SEED,
        }
    }

    fn manager_state_seed(self) -> &'static [u8] {
        match self {
            TokenKind::Gm => GMTOKEN_MANAGER_STATE_SEED,
            TokenKind::Portfolio => PORTFOLIO_TOKEN_MANAGER_STATE_SEED,
        }
    }
}

fn build_mock_mint_instruction_for(
    kind: TokenKind,
    mint: &Pubkey,
    destination_ata: &Pubkey,
    destination_owner: &Pubkey,
    amount: u64,
) -> Instruction {
    let program_id = ondo_gm_program_id();
    let minter = admin_minter();
    let token_program = token_2022_program_id();

    // Derive PDAs with verified seeds
    let (authority_role_account, _) =
        Pubkey::find_program_address(&[kind.minter_role_seed(), minter.as_ref()], &program_id);

    let (oracle_sanity_check, _) =
        Pubkey::find_program_address(&[ORACLE_SANITY_CHECK_SEED, mint.as_ref()], &program_id);

    let (mint_authority, _) =
        Pubkey::find_program_address(&[kind.mint_authority_seed()], &program_id);

    let (usdon_manager_state, _) =
        Pubkey::find_program_address(&[USDON_MANAGER_STATE_SEED], &program_id);

    let (token_limit, _) =
        Pubkey::find_program_address(&[TOKEN_LIMIT_ACCOUNT_SEED, mint.as_ref()], &program_id);

    let (manager_state, _) =
        Pubkey::find_program_address(&[kind.manager_state_seed()], &program_id);

    // Build instruction data: discriminator + amount
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&kind.discriminator());
    data.extend_from_slice(&amount.to_le_bytes());

    // Build accounts list - using destination_ata directly with correct owner
    let accounts = vec![
        AccountMeta::new(minter, true),          // 0: payer (signer, writable)
        AccountMeta::new_readonly(minter, true), // 1: authority (signer)
        AccountMeta::new_readonly(*destination_owner, false), // 2: user (destination owner)
        AccountMeta::new_readonly(authority_role_account, false), // 3: authority_role_account PDA
        AccountMeta::new_readonly(oracle_sanity_check, false), // 4: oracle_sanity_check PDA
        AccountMeta::new_readonly(token_limit, false), // 5: token_limit PDA
        AccountMeta::new_readonly(mint_authority, false), // 6: mint_authority PDA
        AccountMeta::new(*mint, false),          // 7: mint (writable)
        AccountMeta::new(*destination_ata, false), // 8: destination ATA (writable)
        AccountMeta::new_readonly(usdon_manager_state, false), // 9: usdon_manager_state PDA
        AccountMeta::new_readonly(manager_state, false), // 10: gmtoken_manager / portfolio_token_manager PDA
        AccountMeta::new_readonly(token_program, false), // 11: token_program (Token-2022)
        AccountMeta::new_readonly(spl_associated_token_account::id(), false), // 12: ATA program
        AccountMeta::new_readonly(solana_system_interface::program::id(), false), // 13: system_program
    ];

    Instruction {
        program_id,
        accounts,
        data,
    }
}

// ==============================================================================
// Pre-upgrade Layout (remove once the GM program upgrade is live on mainnet)
// ==============================================================================

/// Build a mock mint_gm instruction using the pre-upgrade account layout.
///
/// The upgraded program inserts `token_limit` and `gmtoken_manager` into the
/// middle of the account list, and Anchor resolves accounts by position, so no
/// single layout is accepted by both program versions. `simulate_as_bundle`
/// falls back to this layout when the current one is rejected, which keeps
/// simulations working on either side of the upgrade.
///
/// This is a verbatim copy of the layout the simulator shipped before the
/// upgrade, including `oracle_sanity_check` being writable (the old program
/// declares it `mut`, so a read-only meta fails `ConstraintMut`).
pub(crate) fn build_legacy_mock_mint_gm_instruction(
    gm_token_mint: &Pubkey,
    destination_owner: &Pubkey,
    amount: u64,
) -> Instruction {
    let program_id = ondo_gm_program_id();
    let minter = admin_minter();
    let token_program = token_2022_program_id();

    let (authority_role_account, _) =
        Pubkey::find_program_address(&[MINTER_ROLE_GMTOKEN_SEED, minter.as_ref()], &program_id);

    let (oracle_sanity_check, _) = Pubkey::find_program_address(
        &[ORACLE_SANITY_CHECK_SEED, gm_token_mint.as_ref()],
        &program_id,
    );

    let (mint_authority, _) = Pubkey::find_program_address(&[MINT_AUTHORITY_SEED], &program_id);

    let (usdon_manager_state, _) =
        Pubkey::find_program_address(&[USDON_MANAGER_STATE_SEED], &program_id);

    let destination_ata = get_gm_token_ata(destination_owner, gm_token_mint);

    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&MINT_GM_DISCRIMINATOR);
    data.extend_from_slice(&amount.to_le_bytes());

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
        AccountMeta::new_readonly(solana_system_interface::program::id(), false), // 11: system_program
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

    /// Pins the upgraded `GMTokenMinter` account order, which Anchor resolves positionally.
    #[test]
    fn test_mint_gm_account_layout() {
        let gm_token = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let program_id = ondo_gm_program_id();
        let pda = |seeds: &[&[u8]]| Pubkey::find_program_address(seeds, &program_id).0;

        let ix = build_mock_mint_gm_instruction(&gm_token, &solver, 1);

        let expected = [
            (admin_minter(), true, true),
            (admin_minter(), true, false),
            (solver, false, false),
            (pda(&[MINTER_ROLE_GMTOKEN_SEED, admin_minter().as_ref()]), false, false),
            (pda(&[ORACLE_SANITY_CHECK_SEED, gm_token.as_ref()]), false, false),
            (pda(&[TOKEN_LIMIT_ACCOUNT_SEED, gm_token.as_ref()]), false, false),
            (pda(&[MINT_AUTHORITY_SEED]), false, false),
            (gm_token, false, true),
            (get_gm_token_ata(&solver, &gm_token), false, true),
            (pda(&[USDON_MANAGER_STATE_SEED]), false, false),
            (pda(&[GMTOKEN_MANAGER_STATE_SEED]), false, false),
            (token_2022_program_id(), false, false),
            (spl_associated_token_account::id(), false, false),
            (solana_system_interface::program::id(), false, false),
        ];
        let actual: Vec<_> = ix
            .accounts
            .iter()
            .map(|a| (a.pubkey, a.is_signer, a.is_writable))
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(&ix.data[..8], &MINT_GM_DISCRIMINATOR);
        assert_eq!(&ix.data[8..], &1u64.to_le_bytes());

        // Deriving the ATA ourselves must give the same instruction
        let with_ata = build_mock_mint_gm_instruction_with_ata(
            &gm_token,
            &get_gm_token_ata(&solver, &gm_token),
            &solver,
            1,
        );
        assert_eq!(with_ata, ix);
    }

    /// Pins `PortfolioTokenMinter`: same positions as `GMTokenMinter`, with the
    /// portfolio role, mint authority and manager-state PDAs.
    #[test]
    fn test_mint_portfolio_account_layout() {
        let brain = Pubkey::from_str("7dX7VSxrCBxR76jzymFmV6qXA6qzV4ZW1NSUDquondo").unwrap();
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        let program_id = ondo_gm_program_id();
        let pda = |seeds: &[&[u8]]| Pubkey::find_program_address(seeds, &program_id).0;

        let ix = build_mock_mint_portfolio_instruction(&brain, &solver, 1);

        let expected = [
            (admin_minter(), true, true),
            (admin_minter(), true, false),
            (solver, false, false),
            (pda(&[b"MinterRolePortfolioToken", admin_minter().as_ref()]), false, false),
            (pda(&[b"sanity_check", brain.as_ref()]), false, false),
            (pda(&[b"token", brain.as_ref()]), false, false),
            (pda(&[b"portfolio_mint_authority"]), false, false),
            (brain, false, true),
            (get_gm_token_ata(&solver, &brain), false, true),
            (pda(&[b"usdon_manager"]), false, false),
            (pda(&[b"portfolio_token_manager"]), false, false),
            (token_2022_program_id(), false, false),
            (spl_associated_token_account::id(), false, false),
            (solana_system_interface::program::id(), false, false),
        ];
        let actual: Vec<_> = ix
            .accounts
            .iter()
            .map(|a| (a.pubkey, a.is_signer, a.is_writable))
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(&ix.data[8..], &1u64.to_le_bytes());

        let with_ata = build_mock_mint_portfolio_instruction_with_ata(
            &brain,
            &get_gm_token_ata(&solver, &brain),
            &solver,
            1,
        );
        assert_eq!(with_ata, ix);
    }

    /// Anchor discriminators are sha256("global:<name>")[..8]; this reproduces the
    /// on-chain-verified `mint_gm` value, so it also pins `mint_portfolio`.
    #[test]
    fn test_mint_discriminators() {
        use crate::instruction_discriminator;
        assert_eq!(MINT_GM_DISCRIMINATOR, instruction_discriminator("mint_gm"));
        assert_eq!(MINT_PORTFOLIO_DISCRIMINATOR, instruction_discriminator("mint_portfolio"));
    }

    /// The legacy layout is the upgraded one without `token_limit` (5) and
    /// `gmtoken_manager` (10), and with `oracle_sanity_check` writable.
    #[test]
    fn test_legacy_mint_gm_account_layout() {
        let gm_token = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();

        let legacy = build_legacy_mock_mint_gm_instruction(&gm_token, &solver, 1);

        let mut expected = build_mock_mint_gm_instruction(&gm_token, &solver, 1);
        expected.accounts.remove(10);
        expected.accounts.remove(5);
        expected.accounts[4].is_writable = true;

        assert_eq!(legacy, expected);
        assert_eq!(legacy.accounts.len(), 12);
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
