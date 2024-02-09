pub use anchor_lang::prelude::*;
use crate::decor_state::DecorType;

#[derive(AnchorSerialize, AnchorDeserialize,)]
pub struct DecorArgs {
    pub mint: Pubkey,
    pub global_type: DecorType,
}