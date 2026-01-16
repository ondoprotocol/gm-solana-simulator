// Quick test to check what discriminator [a8, 60, b7, a3, 5c, 0a, 28, a0] might be
use sha2::{Digest, Sha256};

fn compute_discriminator(name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);
    discriminator
}

fn main() {
    let actual = [0xa8, 0x60, 0xb7, 0xa3, 0x5c, 0x0a, 0x28, 0xa0];

    let candidates = [
        "global:fill",
        "global:fill_v2",
        "global:fillOrder",
        "global:fill_order",
        "fill",
        "fillOrder",
        "fill_order",
        "execute_fill",
        "executeFill",
    ];

    println!("Looking for discriminator: {:02x?}", actual);
    println!();

    for candidate in candidates {
        let disc = compute_discriminator(candidate);
        println!("{:30} => {:02x?}", candidate, disc);
        if disc == actual {
            println!("  ✓ MATCH!");
        }
    }
}
