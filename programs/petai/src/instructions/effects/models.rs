
pub use anchor_lang::prelude::*;

use crate::{EffectAutoSetOptions, EffectType};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EffectArgs {
    pub name: String,
    pub effect_type: EffectType,
    pub loneliness_impact: u8,
    pub food_impact: u8,
    pub love_impact: u8,
    pub auto_set: Option<EffectAutoSetOptions>,
    pub duration: u32, // duration in seconds
}