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

    pub const MAX_EFFECT_COUNT: usize = 4;
}
