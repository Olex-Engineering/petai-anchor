use anchor_lang::prelude::*;

use crate::{EffectType, GameEffectInAction};

#[account]
pub struct PetState {
    pub pet_states: Vec<Vec<String>>, // 8 + items
    pub condition: PetCondition, // 1
    pub age: PetAge, // 1
    pub loneliness: u8, // 1
    pub food: u8, // 1
    pub love: u8, // 1
    pub updates_number: u32, // 4
    pub updated_at: i64, // 8
    pub is_died: bool, // 1
    pub thread_id: Vec<u8>, // 4
    pub bump: u8 // 1
}

impl Default for PetState {
    #[inline(never)]
    fn default() -> Self {
        return PetState {
            condition: PetCondition::Super,
            age: PetAge::Puppy,
            loneliness: 100,
            food: 100,
            love: 100,
            updates_number: 0,
            updated_at: Clock::get().unwrap().unix_timestamp,
            is_died: false,
            pet_states: vec![vec![]],
            thread_id: vec![],
            bump: 0,
        }
    }
}

impl PetState {
    #[inline(never)]
    pub fn get_size(pet_states: &Vec<Vec<String>>, thread_id: &Vec<u8>) -> usize {
        let mut size = 8 + 8 + 1 + 1 + 1 + 1 + 1 + 4 + 8 + 1 + 4 + 1;

        for age in pet_states.iter() {
            for condition in age {
                size += 4 + condition.len();
            }
        }

        size += thread_id.len();

        return size;
    }

    #[inline(never)]
    pub fn incease_pet_state_params(&mut self, updates: &PetStateUpdates) -> &mut Self {
        match self.loneliness.checked_add(updates.loneliness) {
            Some(result) => self.loneliness = result,
            None => self.loneliness = 100,
        }

        match self.loneliness.checked_add(updates.food) {
            Some(result) => self.food = result,
            None => self.food = 100,
        }

        match self.loneliness.checked_add(updates.love) {
            Some(result) => self.love = result,
            None => self.love = 100,
        }

        if self.loneliness > 100 {
            self.loneliness = 100;
        }

        if self.food > 100 {
            self.food = 100;
        }

        if self.love > 100 {
            self.love = 100;
        }

        return self;
    }

    
    #[inline(never)]
    pub fn decrease_pet_state_params(&mut self, updates: &PetStateUpdates) -> &mut Self {
        match self.loneliness.checked_sub(updates.loneliness) {
            Some(result) => self.loneliness = result,
            None => self.loneliness = 0,
        }
        
        match self.food.checked_sub(updates.food) {
            Some(result) => self.food = result,
            None => self.food = 0,
        }

        match self.love.checked_sub(updates.love) {
            Some(result) => self.love = result,
            None => self.love = 0
        }

        return self;
    }

    #[inline(never)]
    pub fn handle_effects(&mut self, effects: &Vec<GameEffectInAction>) -> &mut Self {
        effects.iter().for_each(|game_effect| {
            let updates = PetStateUpdates {
                food: game_effect.effect.food_impact,
                love: game_effect.effect.love_impact,
                loneliness: game_effect.effect.loneliness_impact
            };

            if game_effect.effect.effect_type == EffectType::Increase {
                self.incease_pet_state_params(&updates);
            }

            if game_effect.effect.effect_type == EffectType::Decrease {
                self.decrease_pet_state_params(&updates);
            }
        });

        return self;
    }

    #[inline(never)]
    pub fn increase_updates_count(&mut self) -> &mut Self {
        match self.updates_number.checked_add(1) {
            Some(result) => self.updates_number = result,
            None => msg!("Max updates count number"),
        }

        return self;
    }

    // TODO: implement
    #[inline(never)]
    pub fn update_age_if_needed(&mut self) -> bool {
        let age_to_set: PetAge;

        if self.updates_number > 180 { // > month
            age_to_set = PetAge::Young;
        } else if self.updates_number > 500 { // > 2 month and houlf
            age_to_set = PetAge::Adult;
        } else {
            age_to_set = PetAge::Puppy;
        }

        let is_age_changed = self.age != age_to_set;
        self.age = age_to_set;

        return is_age_changed;
    }

    // TODO: implement
    #[inline(never)]
    pub fn update_condition_if_needed(&mut self) -> bool {
        let condition_to_set: PetCondition;

        if self.is_bad_condition() {
            condition_to_set = PetCondition::Bad;
        } else if self.is_ok_condition() {
            condition_to_set = PetCondition::Ok;
        } else if self.is_good_condition() {
            condition_to_set = PetCondition::Good;
        } else {
            condition_to_set = PetCondition::Super;
        }

        let is_condition_changed = self.condition != condition_to_set;
        self.condition = condition_to_set;
        
        return is_condition_changed;
    }

    #[inline(never)]
    pub fn set_updated_at(&mut self) -> &mut Self {
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        return self;
    }

    #[inline(never)]
    pub fn get_pet_state_metadata_uri(&self) -> String {
        return self.pet_states[self.age.index()][self.condition.index()].clone();
    }

    #[inline(never)]
    fn is_bad_condition(&self) -> bool {
        return self.food < 10 && self.love < 20 && self.loneliness < 10;
    }

    #[inline(never)]
    fn is_ok_condition(&self) -> bool {
        return self.food < 30 && (self.love < 80 || self.loneliness < 80);
    }

    #[inline(never)]
    fn is_good_condition(&self) -> bool {
        return self.food < 70 || self.love < 90 || self.loneliness < 90;
    }
}

pub struct PetStateUpdates {
    pub loneliness: u8,
    pub food: u8,
    pub love: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Clone)]
pub enum PetAge {
    Puppy,
    Young,
    Adult
}

impl PetAge {
    fn index(&self) -> usize {
        match *self {
            PetAge::Puppy => 0,
            PetAge::Young => 1,
            PetAge::Adult => 2,
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum PetCondition {
    Super,
    Good,
    Ok,
    Bad
}

impl PetCondition {
    fn index(&self) -> usize {
        match *self {
            PetCondition::Super => 3,
            PetCondition::Good => 2,
            PetCondition::Ok => 1,
            PetCondition::Bad => 0
        }
    }
}