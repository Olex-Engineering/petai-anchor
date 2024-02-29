pub use anchor_lang::prelude::*;

use crate::PetCondition;

#[account]
pub struct EffectState {
    pub name: String, // 4 + 32
    pub effect_type: EffectType, // 1
    pub loneliness_impact: u8, // 1
    pub food_impact: u8, // 1
    pub love_impact: u8, // 1
    pub auto_set: Option<EffectAutoSetOptions>,
    pub duration: u32, // 4 duration in seconds
    pub bump: u8, // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Clone)]
pub struct EffectAutoSetOptions {
    pub chance_of_auto_set: u8, // 1
    pub target_pet_conditions: Vec<PetCondition>, // 4 + 5
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Clone)]
pub enum EffectType {
    Increase,
    Decrease
}

impl EffectState {
    pub const SIZE: usize = 8 + 4 + 32 + 1 + 1 + 4 + 1 + 1 + EffectAutoSetOptions::SIZE;

    #[inline(never)]
    pub fn is_need_to_auto_set(&self, seed: u64, pet_condition: &PetCondition) -> bool {
        if self.auto_set.is_none() {
            return false;
        }

        let auto_set = self.clone().auto_set.unwrap();
        let random = seed.checked_rem(auto_set.chance_of_auto_set as u64);

        return random.unwrap() == 0 && auto_set.target_pet_conditions.iter().any(|condition| condition.eq(pet_condition));
    }
}

impl EffectAutoSetOptions {
    pub const SIZE: usize = 8 + 1 + 4 + 5;
}