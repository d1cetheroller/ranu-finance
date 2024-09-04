use anchor_lang::prelude::*;

#[account]
pub struct RanuConfig {
    pub fee: u64,
    pub authority: Pubkey,
}

impl RanuConfig {
    pub const SEED: &'static str = "RanuConfig";

    // Discriminator (8) + u64 (8)
    pub const ACCOUNT_SIZE: usize = 8 + 8 + 32;

    pub fn new(fee: u64, authority: Pubkey) -> Self {
        Self { fee, authority }
    }
}
