use anchor_lang::prelude::*;
use crate::states::program_state::ProgramState;
use crate::constants::PROGRAM_STATE_SEED;

pub fn update_program_state(
    ctx: Context<UpdateProgramState>,
    data: ProgramState,
) -> Result<()> {
    ctx.accounts.state.set_inner(data);

    return Ok(());
}


#[derive(Accounts)]
#[instruction(data: ProgramState)]
pub struct UpdateProgramState<'info> {
    #[account(
        mut,
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump,
        realloc=ProgramState::get_size(data.real_dogs_configs.as_ref()),
        realloc::payer=signer,
        realloc::zero=true,
    )]
    state: Account<'info, ProgramState>,
    #[account(
        mut,
        address=state.authority
    )]
    signer: Signer<'info>,
    system_program: Program<'info, System>
}