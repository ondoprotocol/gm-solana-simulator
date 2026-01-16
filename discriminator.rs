//! Anchor instruction discriminator utilities.
//!
//! Provides both static (compile-time) and dynamic (runtime) discriminator calculation.

use sha2::{Digest, Sha256};

/// Compute Anchor instruction discriminator: sha256("global:<name>")[0..8]
///
/// **Note:** This is provided for reference/documentation. The actual discriminators
/// used in this library are pre-computed constants verified against on-chain programs.
///
/// Anchor's actual discriminator calculation may differ from this implementation
/// due to version-specific hash function usage. Always verify discriminators against
/// the on-chain IDL.
///
/// # Arguments
///
/// * `name` - The instruction name (e.g., "mint_gm", "fill")
///
/// # Returns
///
/// The first 8 bytes of sha256("global:<name>")
///
/// # Example
///
/// ```
/// use ondo_gm_simulator::instruction_discriminator;
///
/// // This is the theoretical calculation, but actual discriminators
/// // should be verified against on-chain IDL
/// let _disc = instruction_discriminator("mint_gm");
/// ```
pub fn instruction_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());
    let hash_result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash_result[..8]);
    discriminator
}
