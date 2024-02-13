use anchor_lang::prelude::*;
use anchor_spl::metadata::Metadata as MetadataProgram;

use crate::{constants::{PET_STATE_SEED, PLAYER_STATE_SEED, PROGRAM_STATE_SEED}, states::{player_state::PlayerState, program_state::ProgramState}, PetState, PetStateUpdates};

use clockwork_sdk::state::Thread;

pub fn update_player_state_cron(ctx: Context<UpdatePetStateCron>, _: Pubkey) -> Result<()> {
    let pet = &mut ctx.accounts.pet_state;

    msg!("update transaction");

    pet.decrease_pet_state_params(PetStateUpdates {
        loneliness: 1,
        love: 1,
        food: 10
    });

    pet.increase_updates_count();
    pet.update_condition_if_needed();
    pet.update_age_if_needed();

    ctx.accounts.pet_state.updated_at = Clock::get().unwrap().unix_timestamp;

    return Ok(());
}

#[derive(Accounts)]
#[instruction(player_id: Pubkey)]
pub struct UpdatePetStateCron<'info> {
    // states (pda's)
    #[account(
        mut,
        seeds=[PLAYER_STATE_SEED.as_bytes(), player_id.as_ref()],
        bump=player_state.bump,
    )]
    pub player_state: Account<'info, PlayerState>,
    #[account(
        mut,
        seeds=[PET_STATE_SEED.as_bytes(), player_state.key().as_ref()],
        bump=pet_state.bump
    )]
    pub pet_state: Account<'info, PetState>,
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    pub state: Account<'info, ProgramState>,

    /// Verify that only this thread can execute this intruction
    #[account(signer, constraint = thread.authority.eq(&state.key()))]
    pub thread: Account<'info, Thread>,

    // Programs
    pub metadata_program: Program<'info, MetadataProgram>,
    pub system_program: Program<'info, System>,
}