
pub use anchor_lang::prelude::*;

use crate::EffectType;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EffectArgs {
    pub name: String,
    pub effect_type: EffectType,
    pub loneliness_impact: u8,
    pub food_impact: u8,
    pub love_impact: u8,
    pub chance_of_auto_set_on_bad_state: Option<u8>, 
    pub duration_in_hours: u32,
}