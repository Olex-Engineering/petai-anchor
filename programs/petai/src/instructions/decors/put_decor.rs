use anchor_lang::prelude::*;

use crate::{constants::{DECOR_STATE_SEED, PROGRAM_STATE_SEED}, DecorArgs, DecorState, ProgramState};

pub fn put_decor(ctx: Context<PutDecor>, decor_args: DecorArgs) -> Result<()> {
    ctx.accounts.decor_state.set_inner(DecorState {
        key: decor_args.mint,
        global_type: decor_args.global_type,
        bump: ctx.bumps.decor_state
    });

    return Ok(());
}

#[derive(Accounts)]
#[instruction(asset_args: DecorArgs)]
pub struct PutDecor<'info> {
    // states (pda's)
    #[account(
        init_if_needed,
        seeds=[DECOR_STATE_SEED.as_bytes(), asset_args.mint.as_ref()],
        bump,
        payer = signer,
        space = DecorState::get_size()
    )]
    pub decor_state: Account<'info, DecorState>,

    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump
    )]
    pub state: Account<'info, ProgramState>,

    // Signer
    #[account(
        mut,
        address=state.authority.key()
    )]
    pub signer: Signer<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}
