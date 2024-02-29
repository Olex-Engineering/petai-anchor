pub use anchor_lang::prelude::*;
use anchor_spl::{metadata::{Metadata as MetadataProgram, MetadataAccount}, token::{Mint, TokenAccount}};

use clockwork_sdk::state::Thread;
use mpl_token_metadata::accounts::{MasterEdition, Metadata};

use crate::{constants::{PET_STATE_SEED, PLAYER_STATE_SEED, PROGRAM_STATE_SEED}, errors::PetaiErrorCode, start_pet_update_cron_tread, EffectState, PetState, PlayerState, ProgramState, StartPetUpdateCronThreadAccounts};

pub fn init_pet<'info>(
    ctx: Context<'_, '_, 'info, 'info, InitPet<'info>>,
    pet_states: Vec<Vec<String>>,
    thread_id: Vec<u8>,
) -> Result<()> {
    require!(
        ctx.accounts.metadata_account.collection.as_ref().is_some_and(|collection| collection.verified == true
            && collection.key.eq(&ctx.accounts.state.pet_collection)),
        PetaiErrorCode::InvalidDogNft
    );
    require!(ctx.accounts.pet_nft_mint_ata.amount == 1, PetaiErrorCode::InvalidPetNftAta);

    let mut pet_state: PetState = PetState::default();

    pet_state.pet_states = pet_states;
    pet_state.thread_id = thread_id.clone();
    pet_state.bump = ctx.bumps.pet_state;

    ctx.accounts.pet_state.set_inner(pet_state);
    ctx.accounts.player_state.current_pet = Some(ctx.accounts.pet_state.key());

    let effects_for_update_pet_cron: Vec<AccountMeta> = ctx.remaining_accounts.iter().filter_map(|x: & AccountInfo| {
        let effect = Account::<EffectState>::try_from(x);
        
        if effect.is_ok() && x.owner == &crate::ID {
            return Some(AccountMeta::new(effect.unwrap().key(), false));
        } else {
            return None;
        }
    }).collect();
    
    start_pet_update_cron_tread(
        &mut StartPetUpdateCronThreadAccounts {
            player_state: ctx.accounts.player_state.clone(),
            pet_state: ctx.accounts.pet_state.clone(),
            state: ctx.accounts.state.clone(),
            thread: ctx.accounts.thread.clone(),
            pet_nft_mint: ctx.accounts.pet_nft_mint.clone(),
            metadata_program: ctx.accounts.metadata_program.clone(),
            system_program: ctx.accounts.system_program.clone(),
            metadata_account: ctx.accounts.metadata_account.clone(),
            master_edition: ctx.accounts.master_edition.clone(),
            initializer: ctx.accounts.initializer.clone(),
            clockwork_program: ctx.accounts.clockwork_program.clone(),
            effects_metas: effects_for_update_pet_cron
        },
        &thread_id
    )?;

    return Ok(());
}

#[derive(Accounts)]
#[instruction(
    pet_states: Vec<Vec<String>>,
    thread_id: Vec <u8>
)]
pub struct InitPet<'info> {
    #[account(
        seeds=[PLAYER_STATE_SEED.as_bytes(), initializer.key.as_ref()],
        bump=player_state.bump,
    )]
    pub player_state: Account<'info, PlayerState>,
    #[account(
        init,
        seeds=[PET_STATE_SEED.as_bytes(), pet_nft_mint.key().as_ref()],
        bump,
        payer = initializer,
        space = PetState::get_size(&pet_states, &thread_id)
    )]
    pub pet_state: Account<'info, PetState>,
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    pub state: Account<'info, ProgramState>,

    // Pet nft accounts
    pub pet_nft_mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = pet_nft_mint,
        associated_token::authority = initializer
    )]
    pub pet_nft_mint_ata: Account<'info, TokenAccount>,
    /// CHECK: manual verify address
    #[account(
        mut,
        address=Metadata::find_pda(&pet_nft_mint.key()).0
    )]
    pub metadata_account: Account<'info, MetadataAccount>,
    /// CHECK: verify address
    #[account(
        mut,
        address=MasterEdition::find_pda(&pet_nft_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,

    // Signer
    #[account(mut)]
    pub initializer: Signer<'info>,

    // Programs
    /// Address to assign to the newly created thread.
    #[account(mut, address = Thread::pubkey(state.key(), thread_id))]
    pub thread: SystemAccount<'info>,
    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    pub metadata_program: Program<'info, MetadataProgram>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}