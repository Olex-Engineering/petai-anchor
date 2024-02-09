use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AssetArgs {
    pub asset_mint: Pubkey,
    pub increase_food: u8,
    pub increase_loneliness: u8,
    pub increase_love: u8,
}