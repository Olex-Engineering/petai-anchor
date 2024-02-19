use anchor_lang::prelude::*;

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
    fn default() -> Self {
        return PetState {
            condition: PetCondition::Super,
            age: PetAge::Kid,
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

    pub fn incease_pet_state_params(&mut self, updates: PetStateUpdates) {
        self.loneliness += updates.loneliness;
        self.food += updates.food;
        self.love += updates.love;

        if self.loneliness > 100 {
            self.loneliness = 100;
        }

        if self.food > 100 {
            self.food = 100;
        }

        if self.love > 100 {
            self.love = 100;
        }
    }

    
    pub fn decrease_pet_state_params(&mut self, updates: PetStateUpdates) {
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
    }

    pub fn increase_updates_count(&mut self) {
        match self.updates_number.checked_add(1) {
            Some(result) => self.updates_number = result,
            None => msg!("Max updates count number"),
        }
    }

    // TODO: implement
    pub fn update_age_if_needed(&mut self) -> PetAge {
        self.age = PetAge::Kid;

        return PetAge::Kid;
    }

    // TODO: implement
    pub fn update_condition_if_needed(&mut self) -> PetCondition {
        self.condition = PetCondition::Super;
        // TODO: add pet condition change logic
        return PetCondition::Super;
    }
}

pub struct PetStateUpdates {
    pub loneliness: u8,
    pub food: u8,
    pub love: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum PetAge {
    Kid,
    Young,
    Adult,
    Old
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum PetCondition {
    Super,
    Good,
    Middle,
    Bad,
    Dead
}