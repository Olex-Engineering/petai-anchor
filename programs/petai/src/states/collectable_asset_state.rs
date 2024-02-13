pub use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct CollectableAssetState {
    pub last_time_collected: i64, // 8
    pub bump: u8, //1
}

impl CollectableAssetState {
    pub fn get_size() -> usize {
        return 8 + 8 + 1;
    }
}