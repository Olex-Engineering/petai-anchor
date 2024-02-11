pub use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct RealDogConfig {
    pub wallet: Pubkey,
}

impl RealDogConfig {
    pub fn get_size() -> usize {
        return 32 + 4;
    }
}