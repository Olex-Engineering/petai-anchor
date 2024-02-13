use anchor_lang::prelude::*;
use crate::states::program_state::ProgramState;
use crate::constants::{PROGRAM_STATE_SEED, REAL_DOGS_STATE_SEED};
use crate::RealDogsConfigState;

pub fn update_program_state(
    ctx: Context<UpdateProgramState>,
    data: ProgramState,
    real_dogs: RealDogsConfigState
) -> Result<()> {
    ctx.accounts.state.set_inner(data);

    ctx.accounts.real_dogs_state.configs = real_dogs.configs;

    return Ok(());
}


#[derive(Accounts)]
#[instruction(_data: ProgramState, real_dogs: RealDogsConfigState)]
pub struct UpdateProgramState<'info> {
    #[account(
        mut,
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump,
        
    )]
    state: Account<'info, ProgramState>,
    #[account(
        mut,
        seeds=[REAL_DOGS_STATE_SEED.as_bytes()],
        bump=real_dogs_state.bump,
        realloc=RealDogsConfigState::get_size(real_dogs.configs.as_ref()),
        realloc::payer=signer,
        realloc::zero=true,
    )]
    pub real_dogs_state: Account<'info, RealDogsConfigState>,

    #[account(
        mut,
        address=state.authority
    )]
    signer: Signer<'info>,
    system_program: Program<'info, System>
}