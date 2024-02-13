use anchor_lang::prelude::*;

#[account()]
#[derive(Default)]
pub struct ProgramState {
    pub bump: u8, // 1
    pub authority: Pubkey, // 32 Need for validation
    pub pet_collection: Pubkey, // 32 Need for validation
    pub asset_collection: Pubkey, // 32 Need for validation
    pub decor_collection: Pubkey, // 32 Need for validation
    pub token_mint: Pubkey, // 32 Need for validation
}

impl ProgramState {
    pub fn get_size() -> usize {
        return 8 + 1 + 32 + 32 + 32 + 32 + 32;
    }
}