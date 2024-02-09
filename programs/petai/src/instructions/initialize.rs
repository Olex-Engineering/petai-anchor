use anchor_lang::prelude::*;
use crate::states::program_state::ProgramState;
use crate::constants::PROGRAM_STATE_SEED;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    // TODO: create token 
    let mut program_state = ProgramState::default();
    program_state.authority = ctx.accounts.initializer.key();
    program_state.bump = ctx.bumps.state;

    ctx.accounts.state.set_inner(program_state);
    return Ok(());
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump,
        payer=initializer,
        space=ProgramState::get_size(None)
    )]
    pub state: Account<'info, ProgramState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}