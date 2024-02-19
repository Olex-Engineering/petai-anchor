pub use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdatePlayerArgs {
    pub decors: Vec<Pubkey>, // 4 + items
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdatePetNftArgs {
    pub thread_id: Vec<u8>,
}