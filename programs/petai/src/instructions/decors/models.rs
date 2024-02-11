pub use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DecorArgs {
    pub mint: Pubkey,
    pub global_type: DecorType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum DecorType {
    Background,
    OnTheWall,
    Table,
    Floor,
    Bowl
}