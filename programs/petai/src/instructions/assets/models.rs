use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AssetArgs {
    pub asset_mint: Pubkey,
    pub increase_food: u8,
    pub increase_loneliness: u8,
    pub increase_love: u8,
    pub price: u64,
    pub is_can_be_collected: bool,
    pub collectable_time_diff: Option<i64>
}