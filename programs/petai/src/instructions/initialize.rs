use anchor_lang::prelude::*;
use crate::states::program_state::ProgramState;
use crate::constants::{PROGRAM_STATE_SEED, REAL_DOGS_STATE_SEED};
use crate::RealDogsConfigState;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    // TODO: create token 
    let mut program_state = ProgramState::default();
    program_state.authority = ctx.accounts.initializer.key();
    program_state.bump = ctx.bumps.state;

    ctx.accounts.state.set_inner(program_state);
    
    ctx.accounts.real_dogs_state.bump = ctx.bumps.real_dogs_state;
    return Ok(());
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump,
        payer=initializer,
        space=ProgramState::get_size()
    )]
    pub state: Account<'info, ProgramState>,
    #[account(
        init,
        seeds=[REAL_DOGS_STATE_SEED.as_bytes()],
        bump,
        space=RealDogsConfigState::get_size(None),
        payer=initializer

    )]
    pub real_dogs_state: Account<'info, RealDogsConfigState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}