use anchor_lang::prelude::*;
use anchor_spl::{metadata::Metadata as MetadataProgram, token::Mint};

use crate::{constants::{PET_STATE_SEED, PLAYER_STATE_SEED, PROGRAM_STATE_SEED}, states::{player_state::PlayerState, program_state::ProgramState}, PetState, PetStateUpdates};

use clockwork_sdk::state::Thread;

pub fn update_player_state_cron(ctx: Context<UpdatePetStateCron>, _: Pubkey) -> Result<()> {
    let pet = &mut ctx.accounts.pet_state;

    // Decrease pet state parameters
    pet.decrease_pet_state_params(PetStateUpdates {
        loneliness: 1,
        love: 1,
        food: 10,
    });

    // Increase updates count
    pet.increase_updates_count();

    // Update condition and age if needed
    pet.update_condition_if_needed();
    pet.update_age_if_needed();

    // Update updated_at parameter with current Unix timestamp
    ctx.accounts.pet_state.updated_at = Clock::get().unwrap().unix_timestamp;

    Ok(())
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
        seeds=[PET_STATE_SEED.as_bytes(), pet_nft_mint.key().as_ref()],
        bump=pet_state.bump
    )]
    pub pet_state: Account<'info, PetState>,
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    pub state: Account<'info, ProgramState>,

    pub pet_nft_mint: Account<'info, Mint>,


    /// Verify that only this thread can execute this intruction
    #[account(signer, constraint = thread.authority.eq(&state.key()))]
    pub thread: Account<'info, Thread>,

    // Programs
    pub metadata_program: Program<'info, MetadataProgram>,
    pub system_program: Program<'info, System>,
}