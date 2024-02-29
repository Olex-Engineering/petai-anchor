use anchor_lang::prelude::*;

#[account]
pub struct AssetState {
    pub key: Pubkey, // 32
    pub increase_food: u8, // 1
    pub increase_loneliness: u8, // 1
    pub increase_love: u8, // 1
    pub price: u64, // 8
    pub remove_effect: Option<Pubkey>, // 33
    pub add_effect: Option<Pubkey>, // 33
    pub is_can_be_collected: bool, // 1
    pub collectable_time_diff: Option<i64>, // 9
    pub bump: u8 // 1
}

impl AssetState {
    pub fn get_size() -> usize {
        return 8 + 32 + 1 + 1 + 1 + 8 + 1 + 33 + 33 + 9 + 1;
    }
}