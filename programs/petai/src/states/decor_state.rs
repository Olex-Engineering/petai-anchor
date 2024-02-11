use anchor_lang::prelude::*;

use crate::decors::DecorType;

#[account]
pub struct DecorState {
    pub key: Pubkey, // 32
    pub global_type: DecorType, // 1
    pub bump: u8 // 1
}

impl DecorState {
    pub fn get_size() -> usize {
        return 8 + 32 + 1 + 1;
    }
}

