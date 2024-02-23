use anchor_lang::prelude::*;

use crate::GameEffect;

#[account]
#[derive(Default)]
pub struct PlayerState {
    // current pet state
    pub current_pet: Option<Pubkey>, // 32
    pub current_effects: Vec<GameEffect>, // 4 + items
    pub real_dog_treasury: Pubkey, // 32
    pub decors: Vec<Pubkey>, // 4 + items
    pub bump: u8, // 1
}

impl PlayerState {
    pub fn get_size(
        decors: Option<Vec<Pubkey>>
    ) -> usize {
        let mut size = 8 + 1 + 32 + 4 + 32 + 4 + 1;

        size += GameEffect::MAX_EFFECT_COUNT * GameEffect::get_size();

        if decors.is_some() {
            size += decors.unwrap().len() * 32;
        }

        return size;
    }
}

// TODO: add other effects
