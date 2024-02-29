use anchor_lang::prelude::*;

use crate::{EffectState, GameEffectInAction};

#[account]
#[derive(Default)]
pub struct PlayerState {
    // current pet state
    pub current_pet: Option<Pubkey>, // 32
    pub current_effects: Vec<GameEffectInAction>, // 4 + items
    pub real_dog_treasury: Pubkey, // 32
    pub decors: Vec<Pubkey>, // 4 + items
    pub bump: u8, // 1
}

impl PlayerState {
    #[inline(never)]
    pub fn get_size(
        decors: Option<Vec<Pubkey>>
    ) -> usize {
        let mut size = 8 + 1 + 32 + 4 + 32 + 4 + 1;

        size += GameEffectInAction::MAX_EFFECT_COUNT * GameEffectInAction::get_size();

        if decors.is_some() {
            size += decors.unwrap().len() * 32;
        }

        return size;
    }

    #[inline(never)]
    pub fn add_effect(&mut self, effect: EffectState) -> &mut Self {
        let game_effect = GameEffectInAction::from_effect(effect);
        self.current_effects.push(game_effect);

        return self;
    }

    #[inline(never)]
    pub fn remove_effect(&mut self, effect: EffectState) -> &mut Self {
        let filtered_effects: Vec<GameEffectInAction> = self.current_effects.clone().into_iter()
            .filter(|game_effect| return game_effect.effect.name != effect.name)
            .collect();

        self.current_effects = filtered_effects;

        return self;
    }

    #[inline(never)]
    pub fn remove_outdated_game_effected(&mut self) -> &mut Self {
        let filtered_effects: Vec<GameEffectInAction> = self.current_effects.clone().into_iter()
            .filter(|game_effect| return game_effect.end <= Clock::get().unwrap().unix_timestamp)
            .collect();

        self.current_effects = filtered_effects;

        return self;
    }
}

// TODO: add other effects
