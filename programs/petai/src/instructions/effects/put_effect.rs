pub use anchor_lang::prelude::*;

use crate::{constants::PROGRAM_STATE_SEED, EffectArgs, EffectState, ProgramState};

pub fn put_effect(ctx: Context<PutEffect>, effect: EffectArgs) -> Result<()> {
    ctx.accounts.effect_state.set_inner(EffectState {
        name: effect.name,
        effect_type: effect.effect_type,
        loneliness_impact: effect.loneliness_impact,
        food_impact: effect.food_impact,
        love_impact: effect.love_impact,
        chance_of_auto_set_on_bad_state: effect.chance_of_auto_set_on_bad_state,
        duration_in_hours: effect.duration_in_hours,
        bump: ctx.bumps.effect_state
    });

    return Ok(());
}

#[derive(Accounts)]
#[instruction(effect: EffectArgs)]
pub struct PutEffect<'info> {
    // states (pda's)
    #[account(
        init_if_needed,
        seeds=[effect.name.as_bytes()],
        bump,
        payer = initializer,
        space = EffectState::SIZE
    )]
    pub effect_state: Account<'info, EffectState>,

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
    pub initializer: Signer<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}
