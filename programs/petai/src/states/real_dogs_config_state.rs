use anchor_lang::prelude::*;

use crate::RealDogConfig;

#[account]
#[derive(Default)]
pub struct RealDogsConfigState {
    pub configs: Option<Vec<RealDogConfig>>,  // Need for validation
    pub bump: u8 // 1
}

impl RealDogsConfigState {
    pub fn get_size(real_dogs_configs: Option<&Vec<RealDogConfig>>) -> usize {
        let mut size = 8 + 4 + 1 + 1;

        if real_dogs_configs.is_some() {
            size += RealDogConfig::get_size() * real_dogs_configs.unwrap().len();
        }

        return size;
    }
}