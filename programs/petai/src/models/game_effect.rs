pub use anchor_lang::prelude::*;

use crate::{EffectArgs, EffectState};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GameEffectInAction {
    pub effect: EffectArgs,
    pub end: i64 // 8
}

impl GameEffectInAction {
    pub fn get_size() -> usize {
        return 8 + 8 + EffectState::SIZE;
    }

    pub fn from_effect(effect: EffectState) -> GameEffectInAction {
        let end = Clock::get().unwrap().unix_timestamp + effect.duration as i64;

        return GameEffectInAction {
            effect: EffectArgs {
                name: effect.name,
                effect_type: effect.effect_type,
                duration: effect.duration,
                loneliness_impact: effect.loneliness_impact,
                food_impact: effect.food_impact,
                love_impact: effect.love_impact,
                auto_set: effect.auto_set
            },
            end: end
        }
    }

    pub const MAX_EFFECT_COUNT: usize = 4;
}
