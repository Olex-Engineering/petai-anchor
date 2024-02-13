use anchor_lang::prelude::*;

use crate::GameEffect;

#[account]
#[derive(Default)]
pub struct PlayerState {
    pub pet_states: Vec<Vec<String>>, // 4 + items
    pub current_effects: Vec<GameEffect>, // 4 + items
    pub real_dog_treasury: Pubkey, // 32
    pub decors: Vec<Pubkey>, // 4 + items
    pub updated_at: i64, // 8
    pub bump: u8, // 1
}

impl PlayerState {
    pub fn get_size(
        pet_states: Vec<Vec<String>>,
        decors: Option<Vec<Pubkey>>
    ) -> usize {
        let mut current_size = 8 + 4 + 4 + 32 + 4 + 8 + 1;

        for age in pet_states.iter() {
            for condition in age {
                current_size += 4 + condition.len();
            }
        }

        current_size += GameEffect::MAX_EFFECT_COUNT * GameEffect::get_size();

        if decors.is_some() {
            current_size += decors.unwrap().len() * 32;
        }

        return current_size;
    }
}

// TODO: add other effects
