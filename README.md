# Ondo GM Simulator

A Rust library for handling Ondo Global Markets (GM) JIT trades via Jupiter RFQ in wallet transaction simulation.

## Problem

Ondo GM tokens are minted just-in-time (JIT) when a swap occurs. This means standard transaction simulation fails because the solver doesn't have the GM tokens until the mint instruction executes in the Jito bundle.

## Solution

This crate detects GM trades and generates mock mint transactions that can be simulated as a bundle, allowing the simulation to succeed and show accurate balance changes.

## Installation

```toml
[dependencies]
gm-solana-simulator = { git = "https://github.com/ondoprotocol/gm-solana-simulator" }
```

## Quick Start

```rust
use gm_solana_simulator::{
    check_gm_trade, build_mock_mint_transaction, simulate_as_bundle,
    GmSimulatorError, BundleSimulationResult
};
use solana_sdk::{hash::Hash, transaction::Transaction};

fn handle_simulation(tx: &Transaction, recent_blockhash: Hash, rpc_url: &str) {
    match check_gm_trade(tx) {
        Ok(result) if result.use_gm_bundle_sim => {
            // This is a GM trade - simulate as bundle
            let trade_info = result.trade_info.unwrap();
            let mock_mint_tx = build_mock_mint_transaction(&trade_info, recent_blockhash);

            // Simulate bundle and get taker balance changes
            match simulate_as_bundle(vec![mock_mint_tx, tx.clone()], &trade_info, rpc_url) {
                Ok(sim_result) => {
                    if sim_result.success {
                        println!("Simulation succeeded!");
                        for change in &sim_result.taker_balance_changes {
                            println!(
                                "{}: {} (pre: {}, post: {})",
                                change.symbol.as_deref().unwrap_or("?"),
                                change.change_display(),
                                change.pre_balance,
                                change.post_balance
                            );
                        }
                    } else {
                        println!("Simulation failed: {:?}", sim_result.error);
                    }
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
        Ok(_) => {
            // Not a GM trade - use normal simulation
            simulate_single(tx);
        }
        Err(GmSimulatorError::UnauthorizedMaker(maker)) => {
            // Jupiter RFQ with GM token but unauthorized maker
            warn!("Unauthorized GM maker: {}", maker);
        }
        Err(_) => {
            // Parse error - fall back to normal simulation
            simulate_single(tx);
        }
    }
}
```

## Detection Criteria

A transaction qualifies as a GM trade that requires bundle simulation if **ALL** of these are true:

| Criterion | Check |
|-----------|-------|
| Contains Jupiter RFQ fill | Transaction contains at least one Jupiter Order Engine fill instruction |
| Authorized maker | Maker is one of 3 authorized solvers |
| GM token output | Taker receives a GM token |

**Note:** GM transactions typically include additional instructions like `createAssociatedTokenAccountIdempotent` to ensure the taker's ATA exists. The detector searches for the Jupiter fill instruction among all instructions in the transaction.

## API Reference

### Main Functions

```rust
/// Check if a transaction is a GM trade
pub fn check_gm_trade(transaction: &Transaction) -> Result<GmCheckResult, GmSimulatorError>

/// Build mock mint transaction for bundle simulation
pub fn build_mock_mint_transaction(trade_info: &GmTradeInfo, recent_blockhash: Hash) -> Transaction

/// Convenience: check and build in one call
pub fn maybe_build_mock_mint(
    transaction: &Transaction,
    recent_blockhash: Hash,
) -> Result<Option<Transaction>, GmSimulatorError>

/// Simulate bundle via Jito and return taker balance changes
pub fn simulate_as_bundle(
    transactions: Vec<Transaction>,
    trade_info: &GmTradeInfo,
    rpc_url: &str,
) -> Result<BundleSimulationResult, GmSimulatorError>
```

### Types

```rust
pub struct GmCheckResult {
    pub use_gm_bundle_sim: bool,
    pub trade_info: Option<GmTradeInfo>,
}

pub struct GmTradeInfo {
    pub maker: Pubkey,           // Solver address
    pub taker: Pubkey,           // User address
    pub gm_token_mint: Pubkey,   // GM token being traded
    pub gm_token_symbol: String, // e.g., "AAPLon"
    pub gm_token_amount: u64,    // Amount (9 decimals)
    pub maker_output_account: Pubkey, // Solver's token account
    pub expire_at: i64,          // Quote expiration timestamp
}

pub struct BundleSimulationResult {
    pub success: bool,                           // Whether simulation succeeded
    pub error: Option<String>,                   // Error message if failed
    pub taker_balance_changes: Vec<BalanceChange>, // Balance changes for taker
    pub logs: Option<Vec<String>>,               // Simulation logs
}

pub struct BalanceChange {
    pub mint: Pubkey,           // Token mint address
    pub symbol: Option<String>, // Token symbol (e.g., "USDC", "AAPLon")
    pub owner: Pubkey,          // Account owner
    pub token_account: Pubkey,  // Token account address
    pub pre_balance: u64,       // Balance before transaction
    pub post_balance: u64,      // Balance after transaction
    pub change: i128,           // Change amount (positive = received)
    pub decimals: u8,           // Token decimals for display
}

pub enum GmSimulatorError {
    NotJupiterRfq,
    NotSingleInstruction,
    TakerNotReceivingGmToken,
    UnauthorizedMaker(Pubkey),
    InstructionParseError(String),
    InvalidAccountIndex,
    MissingAccount,
    EmptyTransaction,
}
```

## Integration Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                 Transaction Received for Simulation              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    check_gm_trade(&tx)                          │
└─────────────────────────────────────────────────────────────────┘
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
    Ok(use_gm_bundle_sim=true)  Ok(use_gm_bundle_sim=false)  Err(UnauthorizedMaker)
          │                   │                   │
          ▼                   ▼                   ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ Build mock mint │  │ Normal single   │  │ Reject/Warn     │
│ Simulate bundle │  │ TX simulation   │  │                 │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

## Bundle Simulation Order

When simulating as a bundle, the order is critical:

```
Bundle = [
    Transaction 1: Mock mint_gm (mints GM tokens to solver)
    Transaction 2: Original Jupiter fill (swaps tokens with user)
]
```

The mock mint provides the GM tokens that the fill instruction needs.

## Constants

### Authorized Solvers
```
DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds
2Cq2RNFFxxPXL7teNQAji1beA2vFbBDYW5BGPBFvoN9m
9BB7Tt5uE5VdRsxA5XRqrjwNaq8XtgAUQW8czA6ymUPG
```

### Admin Minter (Real On-Chain Authority)
```
4pfyfezvwjBrsHtJpXPPKsqH9cphwSDDb7s63KzkVEqF
```
This is the actual admin minter with MINTER_ROLE_GMTOKEN permissions on mainnet.

### Ondo GM Program
```
XzTT4XB8m7sLD2xi6snefSasaswsKCxx5Tifjondogm
```

### Jupiter Order Engine
```
61DFfeTKM7trxYcPQCM78bJ794ddZprZpAwAnLiwTpYH
```

## Token List

The crate includes 201 GM tokens representing tokenized equities:

| Symbol | Mint Address |
|--------|--------------|
| AAPLon | 123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo |
| TSLAon | KeGv7bsfR4MheC1CkmnAVceoApjrkvBhHYjWb67ondo |
| NVDAon | gEGtLTPNQ7jcg25zTetkbmF7teoDLcrfTnQfmn2ondo |
| AMZNon | 14Tqdo8V1FhzKsE3W2pFsZCzYPQxxupXRcqw9jv6ondo |
| GOOGLon | bbahNA5vT9WJeYft8tALrH1LXWffjwqVoUbqYa1ondo |
| METAon | fDxs5y12E7x7jBwCKBXGqt71uJmCWsAQ3Srkte6ondo |
| MSFTon | FRmH6iRkMr33DLG6zVLR7EM4LojBFAuq6NtFzG6ondo |
| ... | (see constants.rs for full list) |

## Important Notes

### IDL Verification

The `mint_gm` instruction has been verified against the actual on-chain program at `XzTT4XB8m7sLD2xi6snefSasaswsKCxx5Tifjondogm`:

**Verified values:**
- Instruction discriminator: `[117, 223, 58, 111, 44, 36, 16, 43]`
- Account structure: 12 accounts (see [mint_instruction.rs](mint_instruction.rs) for details)
- PDA seeds:
  - `authority_role_account`: `[b"MinterRoleGMToken", minter]`
  - `oracle_sanity_check`: `[b"sanity_check", mint]`
  - `mint_authority`: `[b"mint_authority"]`
  - `usdon_manager_state`: `[b"usdon_manager"]`

To re-verify in the future:
```bash
anchor idl fetch XzTT4XB8m7sLD2xi6snefSasaswsKCxx5Tifjondogm --provider.cluster mainnet
```

### Token Standard

GM tokens use **Token-2022** (not SPL Token). The crate handles this automatically when deriving ATAs.

### Decimal Places

All GM tokens have **9 decimal places**. Amounts in `GmTradeInfo.gm_token_amount` are in base units (1 token = 1,000,000,000 base units).

## Helper Functions

```rust
// Check if an address is an authorized solver
is_authorized_solver(&pubkey) -> bool

// Check if an address is a GM token mint
is_gm_token(&pubkey) -> bool

// Get symbol for a GM token
get_gm_token_symbol(&pubkey) -> Option<&str>

// Get GM token ATA (Token-2022)
get_gm_token_ata(&owner, &mint) -> Pubkey
```

## Example: Full Integration

```rust
use ondo_gm_simulator::{
    check_gm_trade, build_mock_mint_transaction, 
    GmSimulatorError, is_gm_token
};
use solana_sdk::{hash::Hash, transaction::Transaction};

pub struct SimulationResult {
    pub success: bool,
    pub error: Option<String>,
    pub balance_changes: Vec<BalanceChange>,
}

pub fn simulate_with_gm_support(
    tx: &Transaction,
    recent_blockhash: Hash,
) -> SimulationResult {
    // First, check if this is a GM trade
    match check_gm_trade(tx) {
        Ok(result) if result.use_gm_bundle_sim => {
            let info = result.trade_info.unwrap();
            log::info!(
                "GM trade detected: {} {} tokens to {}",
                info.gm_token_amount,
                info.gm_token_symbol,
                info.taker
            );
            
            // Build mock mint and simulate as bundle
            let mock_mint = build_mock_mint_transaction(&info, recent_blockhash);
            
            // Your bundle simulation logic here
            simulate_bundle(&[mock_mint, tx.clone()])
        }
        
        Ok(_) => {
            // Not a GM trade, normal simulation
            simulate_single(tx)
        }
        
        Err(GmSimulatorError::UnauthorizedMaker(maker)) => {
            // This is suspicious - GM token with unauthorized maker
            SimulationResult {
                success: false,
                error: Some(format!(
                    "Transaction involves GM tokens but maker {} is not authorized",
                    maker
                )),
                balance_changes: vec![],
            }
        }
        
        Err(e) => {
            // Other parsing errors - fall back to normal
            log::debug!("GM check failed: {}, using normal simulation", e);
            simulate_single(tx)
        }
    }
}
```

## Testing

### Unit Tests

Run all unit tests:
```bash
cargo test
```

### Mainnet Integration Test

The crate includes a mainnet integration test that fetches a real GM trade transaction, validates detection, and tests simulation. This test requires network access and is marked as `#[ignore]` to prevent running in regular CI.

#### Basic Usage

Note that your RPC must support Jito `simulateBundle`.

```bash
RPC_URL=https://your-rpc-endpoint.com TX_HASH=<tx_hash> cargo test test_mainnet_transaction -- --ignored --nocapture
```

#### Example

```bash
RPC_URL=https://mainnet.helius-rpc.com/?api-key=xxxxx \
TX_HASH=3HHNNCR2q4VtLjRjmMZEJqRBph1Ve6nnc6HigBWkCUijX5V7zxmVtjCFCDE8JHaZAXEq5rQEN3g7jZrpiHiZRZxk \
cargo test test_mainnet_transaction -- --ignored --nocapture
```

#### What the Test Does

1. **Fetches transaction** from Solana mainnet via RPC
2. **Strips signatures** to create an unsigned transaction
3. **Validates `check_gm_trade()`** correctly identifies the GM trade
4. **Extracts trade info** - displays maker, taker, token details, amounts
5. **Builds mock mint transaction** with proper instruction structure
6. **Attempts simulation** against mainnet (blockhash errors are expected for old transactions)

#### Expected Output

```
================================================================================
COMPREHENSIVE MAINNET TRANSACTION TEST
================================================================================
Testing with transaction: 3kuMi91iAVbp2QPfhykUD5T7KmWA6UGFHRrUiZruB57dzMmodwm8BMTy1aEnQCXhDoDFsVpkBa8L8r8gcdYpiRsR
Using RPC: https://your-rpc.com

Fetching transaction from mainnet...
✓ Fetched transaction successfully
  Instructions: 4
  Signatures: 2

Transaction Analysis:
  Programs:
    Instruction 0: ComputeBudget111111111111111111111111111111
    Instruction 1: ComputeBudget111111111111111111111111111111
    Instruction 2: ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
    Instruction 3: 61DFfeTKM7trxYcPQCM78bJ794ddZprZpAwAnLiwTpYH
      ✓ Jupiter Order Engine fill instruction found!
      ...

      Trade Analysis:
        Taker (user): 5Qaeh6cMTBbzmGUdexmGx8KKTvAgMtsTsBTATM293Q63
        Maker (solver): 2Cq2RNFFxxPXL7teNQAji1beA2vFbBDYW5BGPBFvoN9m ✓ AUTHORIZED
        Input mint: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v (USDC)
        Output mint: 123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo (AAPLon)

      Detection Criteria:
        ✓ Is GM trade (GM token involved)
        ✓ Maker is authorized
        ✓ Taker receives GM token (output)

      Trade Type: BUY (USDC → GM)
      Bundle Simulation: REQUIRED
      Reason: Solver needs GM tokens minted JIT

Stripping signatures...
✓ Signatures stripped

Checking GM trade detection...

  Result: GM trade detected, bundle simulation REQUIRED
         (BUY transactions need minting - solver needs GM tokens)

✓ GM BUY Trade Confirmed:
  Maker (solver): 2Cq2RNFFxxPXL7teNQAji1beA2vFbBDYW5BGPBFvoN9m
  Taker (user): 5Qaeh6cMTBbzmGUdexmGx8KKTvAgMtsTsBTATM293Q63
  GM Token: AAPLon (123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo)
  Amount: 39704332
  Expire At: 1768882681

Building mock mint transaction...
✓ Mock mint transaction built
  Instructions: 5

Simulating bundle on mainnet...
  Simulating: [mock_mint_tx, original_fill_tx]

  Fetching fresh blockhash for simulation...
  ✓ Got fresh blockhash: 2yw7YyXm9RDTGvF3nFcbYx939bE1bskRzuxU2FS3ryut
  Updated expire_at to: 1768888695

  Using Jito bundle simulation via simulate_as_bundle...
  ✓ Bundle simulation succeeded

  Taker Balance Changes:
    ... USDC: -X.XXXXXX
    ... AAPLon: +0.039704

================================================================================
✓ MAINNET BUY TRANSACTION TEST COMPLETED
================================================================================
```

#### Finding GM Trade Transactions

To find GM trade transactions to test with:

1. Look for transactions on [Solana Explorer](https://explorer.solana.com)
2. Filter by program: `61DFfeTKM7trxYcPQCM78bJ794ddZprZpAwAnLiwTpYH` (Jupiter Order Engine)
3. Find transactions involving authorized solvers and GM token mints
4. Use the transaction signature as `TX_HASH`

### From Scratch Integration Tests

The crate includes hardcoded integration tests that don't require a transaction hash. These tests only need an RPC_URL and validate core detection and construction logic.

#### From Scratch Test

Tests BUY transactions (USDC → GM) using embedded transaction data:

```bash
RPC_URL=https://mainnet.helius-rpc.com/?api-key=xxxxx cargo test test_from_scratch -- --ignored --nocapture
```

**What it validates:**
- ✓ GM BUY trade correctly detected
- ✓ Mock mint transaction correctly built
- ✓ Detection identifies bundle simulation as required
- ✓ Mock mint and Jupiter RFQ Fill can be successfully simulated through Jito bundle

- ✓ GM SELL trade correctly detected
- ✓ Detection identifies bundle simulation as NOT required
- ✓ Proper differentiation between BUY and SELL flows

#### Expected Output

```
================================================================================
COMPREHENSIVE FROM-SCRATCH TEST (BUY AND SELL)
================================================================================
Using RPC: https://your-rpc.com

Fetching fresh blockhash...
✓ Got fresh blockhash: ...

================================================================================
TEST 1: BUY TRANSACTION (USDC → GM) - REQUIRES BUNDLE SIMULATION
================================================================================

Accounts (BUY):
  Taker:           7z86y3WYofAiuxhQvYV2U6ZQMQ7jLxncuyV9U7D8PwYV
  Maker:           DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds
  ...

Building BUY transaction:
  Input: 1 USDC
  Output: 3.880411 AAPLon
  Expire at: ... (future)

✓ BUY trade correctly identified as requiring bundle simulation

Building mock mint transaction...
✓ Mock mint built with 5 instructions

Simulating bundle...
  ✓ Bundle simulation succeeded

  Taker Balance Changes:
    ... USDC: -1.000000
    ... AAPLon: +0.003880

================================================================================
TEST 2: SELL TRANSACTION (GM → USDC) - NO BUNDLE SIMULATION REQUIRED
================================================================================

Building SELL transaction:
  Input: 0.007 AAPLon
  Output: 0.001801 USDC

✓ SELL trade correctly identified as NOT requiring bundle simulation
  (Solver already has USDC - no minting needed)

================================================================================
✓ FROM-SCRATCH TEST COMPLETED
================================================================================
```

### Payload File Test

Test a base64-encoded transaction payload directly from a file. This is useful for testing transactions captured from a browser wallet before signing.

#### Basic Usage

1. Save the base64-encoded transaction to a file (e.g., `payload`)
2. Run the test:

```bash
RPC_URL=https://your-jito-rpc.com cargo test test_payload_file -- --ignored --nocapture
```

By default, the test reads from `./payload`. To use a different file:

```bash
RPC_URL=https://your-jito-rpc.com PAYLOAD_FILE=./my_tx.txt cargo test test_payload_file -- --ignored --nocapture
```

#### What it does

1. Reads base64-encoded transaction from file
2. Deserializes as `VersionedTransaction` (supports both legacy and V0)
3. Checks if it's a GM trade using `check_gm_trade_versioned`
4. If it's a GM BUY trade, builds mock mint and simulates the bundle via Jito
5. Reports balance changes and simulation results

**Important:** The test uses the **exact** transaction payload with no modifications (blockhash, expiration, etc. are preserved). This validates that the transaction will simulate correctly as-is.

#### Expected Output

```
================================================================================
PAYLOAD FILE TEST
================================================================================
Reading payload from: payload
Payload length: 928 bytes (base64)
Decoded transaction: 695 bytes
Transaction type: V0 (versioned)

Checking GM trade detection...
✓ GM BUY trade detected - bundle simulation REQUIRED

✓ GM BUY Trade Details:
  Maker (solver): 2Cq2RNFFxxPXL7teNQAji1beA2vFbBDYW5BGPBFvoN9m
  Taker (user): FGb3upaQC5sRuTnEHRMbpmkkUGiqat9wPVDVnKvSeZvb
  GM Token: TSLAon (KeGv7bsfR4MheC1CkmnAVceoApjrkvBhHYjWb67ondo)
  Amount: 43983669 (0.043984 TSLAon)
  Expire At: 1768884710

Using RPC: https://your-rpc.com

Using EXACT transaction payload (no modifications):
  Blockhash from payload: 9bQhxgjDXVhDeAxrZRYkFJrJ8v3KCLxit3TkS2yiRRNs
  Expire At from payload: 1768884710

Building mock mint transaction...
✓ Mock mint transaction built (5 instructions)

Simulating bundle via Jito...
  Bundle: [mock_mint_tx, original_fill_tx (unchanged)]

✓ Bundle simulation SUCCEEDED!

Taker Balance Changes:
  FawntTfvEkiqQH4EfdjZqwgbegCBwPcWFF2b8unt3W8f USDC: -18.957855
    Pre:  37.915711 USDC (raw: 37915711)
    Post: 18.957856 USDC (raw: 18957856)
  6MXs25DaDt3hs1YKJVATo6Uot5mYeNybmSfmhrrWydQH TSLAon: +0.043984
    Pre:  0.000000 TSLAon (raw: 0)
    Post: 0.043984 TSLAon (raw: 43983669)

================================================================================
✓ PAYLOAD FILE TEST COMPLETED
================================================================================
```

## Maintenance

This crate is maintained by Ondo Finance. Contact: engineering@ondo.finance

Updates will be provided for:
- New GM token additions
- Solver address changes
- IDL updates

## License

MIT
