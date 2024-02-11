pub use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GameEffect {
    pub effect_type: GameEffectType, // 1
    pub effect_power: u8, // 1
    pub end: i64 // 8
}

impl GameEffect {
    pub fn get_size() -> usize {
        return 1 + 1 + 8;
    }

    pub const MAX_EFFECT_COUNT: usize = 4;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GameEffectType {
    Game,
    Walk,
    Food
}