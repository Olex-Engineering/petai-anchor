pub use anchor_lang::prelude::*;

use crate::{constants::PLAYER_STATE_SEED, PlayerState, UpdatePlayerArgs};

pub fn update_player(ctx: Context<UpdatePlayer>, update_args: UpdatePlayerArgs) -> Result<()> {
    ctx.accounts.player_state.decors = update_args.decors;

    return Ok(());
}

#[derive(Accounts)]
#[instruction(update_args: UpdatePlayerArgs)]
pub struct UpdatePlayer<'info> {
    #[account(
        mut,
        seeds=[PLAYER_STATE_SEED.as_bytes(), initializer.key.as_ref()],
        bump=player_state.bump,
        realloc=PlayerState::get_size(Some(update_args.decors.clone())),
        realloc::payer=initializer,
        realloc::zero=true,
    )]
    pub player_state: Account<'info, PlayerState>,

    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}