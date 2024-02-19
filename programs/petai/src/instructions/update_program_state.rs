use anchor_lang::prelude::*;
use crate::states::program_state::ProgramState;
use crate::constants::{PROGRAM_STATE_SEED, REAL_DOGS_STATE_SEED};
use crate::RealDogsWalletState;

pub fn update_program_state(
    ctx: Context<UpdateProgramState>,
    data: ProgramState,
    real_dogs: Option<Vec<Pubkey>>
) -> Result<()> {
    ctx.accounts.state.set_inner(data);

    if real_dogs.is_some() {
        ctx.accounts.real_dogs_state.wallets = real_dogs.unwrap();
    }

    return Ok(());
}

#[derive(Accounts)]
#[instruction(_data: ProgramState, real_dogs: Option<Vec<Pubkey>>)]
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
        realloc=RealDogsWalletState::get_size(real_dogs.as_ref()),
        realloc::payer=initializer,
        realloc::zero=true,
    )]
    pub real_dogs_state: Account<'info, RealDogsWalletState>,
    #[account(
        mut,
        address=state.authority
    )]
    initializer: Signer<'info>,
    system_program: Program<'info, System>
}