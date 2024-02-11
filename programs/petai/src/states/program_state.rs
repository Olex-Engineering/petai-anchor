use anchor_lang::prelude::*;

use crate::RealDogConfig;

#[account]
#[derive(Default)]
pub struct ProgramState {
    pub bump: u8, // 1
    pub authority: Pubkey, // 32 Need for validation
    pub pet_collection: Pubkey, // 32 Need for validation
    pub asset_collection: Pubkey, // 32 Need for validation
    pub decor_collection: Pubkey, // 32 Need for validation
    pub token_mint: Pubkey, // 32 Need for validation
    pub real_dogs_configs: Option<Vec<RealDogConfig>>  // Need for validation
}

impl ProgramState {
    pub fn get_size(real_dogs_configs: Option<&Vec<RealDogConfig>>) -> usize {
        let mut size = 8 + 1 + 32 + 32 + 32 + 32 + 32 + 4;

        if real_dogs_configs.is_some() {
            size += 1;
            
            size += RealDogConfig::get_size() * real_dogs_configs.unwrap().len();
        }

        return size;
    }
}