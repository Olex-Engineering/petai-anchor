use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct RealDogsWalletState {
    pub wallets: Vec<Pubkey>,  // 4
    pub bump: u8 // 1
}

impl RealDogsWalletState {
    pub fn get_size(real_dogs_configs: Option<&Vec<Pubkey>>) -> usize {
        let mut size = 8 + 4 + 1;

        if real_dogs_configs.is_some() {
            size += 32 * real_dogs_configs.unwrap().len();
        }

        return size;
    }
}