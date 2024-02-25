pub use anchor_lang::prelude::*;

#[account()]
pub struct EffectState {
    pub name: String, // 4 + 32
    pub effect_type: EffectType, // 1
    pub loneliness_impact: u8, // 1
    pub food_impact: u8, // 1
    pub love_impact: u8, // 1
    pub chance_of_auto_set_on_bad_state: Option<u8>, // 1
    pub duration_in_hours: u32, // 4
    pub bump: u8, // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Clone)]
pub enum EffectType {
    Increase,
    Decrease
}

impl EffectState {
    pub const SIZE: usize = 8 + 4 + 32 + 1 + 1 + 1 + 1 + 4 + 1;
}