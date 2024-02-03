use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PlayerState {
    pub pet_states: Vec<Vec<String>>, // 4 + items
    pub current_effects: Vec<GameEffect>, // 4 + items
    pub decors: Vec<Decor>, // 4 + items
    pub updated_at: i64, // 8
    pub last_free_collection_mint: i64, // 8
    pub bump: u8, // 1
}

impl PlayerState {
    pub fn get_size(
        pet_states: Vec<Vec<String>>
    ) -> usize {
        let mut current_size = 8 + 4 + 4 + 4 + 8 + 8 + 1;

        for age in pet_states.iter() {
            for condition in age {
                current_size += 4 + condition.len();
            }
        }

        current_size += GameEffect::MAX_EFFECT_COUNT * GameEffect::get_size();
        current_size += Decor::MAX_DECOR_COUNT * Decor::get_size();

        return current_size;
    }
}

// TODO: add other effects
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GameEffect {
    effect_type: GameEffectType, // 1
    effect_power: u8, // 1
    end: i64 // 8
}

impl GameEffect {
    pub fn get_size() -> usize {
        return 1 + 1 + 8;
    }

    const MAX_EFFECT_COUNT: usize = 4;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GameEffectType {
    Love,
    Game,
    Walk,
    Food
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Decor {
    pub decor_type: DecorType,
    pub nft: Pubkey,
    pub bonus: u8,
}

impl Decor {
    pub fn get_size() -> usize {
        return 1 + 1 + 32;
    }

    const MAX_DECOR_COUNT: usize = 5;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum DecorType {
    SomeType
}
