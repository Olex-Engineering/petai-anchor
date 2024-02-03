use anchor_lang::prelude::*;

#[account]
pub struct AssetState {
    pub key: Pubkey, // 32
    pub increase_food: u8, // 1
    pub increase_loneliness: u8, // 1
    pub increase_love: u8, // 1
    pub bump: u8 // 1
}

impl AssetState {
    pub fn get_size() -> usize {
        return 8 + 32 + 1 + 1 + 1 + 1;
    }
}