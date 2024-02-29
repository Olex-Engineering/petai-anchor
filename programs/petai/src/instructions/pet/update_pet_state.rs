use std::str::FromStr;

use anchor_lang::{prelude::*, solana_program};
use anchor_spl::{metadata::{mpl_token_metadata::{instructions::UpdateV1CpiBuilder, types::Data}, Metadata as MetadataProgram, MetadataAccount}, token::Mint};
use arrayref::array_ref;
use mpl_token_metadata::accounts::{MasterEdition, Metadata};

use crate::{constants::{BLOCK_HASHES, PET_STATE_SEED, PLAYER_STATE_SEED, PROGRAM_STATE_SEED, SELLER_FEE}, states::{player_state::PlayerState, program_state::ProgramState}, EffectState, PetCondition, PetState, PetStateUpdates};

use clockwork_sdk::state::Thread;

pub fn update_pet_state_cron<'info>(
    ctx: Context<'_, '_, 'info, 'info, UpdatePetStateCron<'info>>,
    _: Pubkey
) -> Result<()> {
    let pet = &mut ctx.accounts.pet_state;

    ctx.accounts.player_state.remove_outdated_game_effected();

    // Decrease pet state parameters
    pet
        .decrease_pet_state_params(&PetStateUpdates {
            loneliness: 1,
            love: 1,
            food: 10,
        })
        .handle_effects(&ctx.accounts.player_state.current_effects)
        .increase_updates_count()
        .set_updated_at();

    let is_condition_updated = pet.update_condition_if_needed();
    let is_age_updated = pet.update_age_if_needed();

    // Update METADATA Uri depends on condition and age
    if is_condition_updated || is_age_updated {
        UpdateV1CpiBuilder::new(&ctx.accounts.metadata_program.to_account_info())
            .edition(Some(ctx.accounts.master_edition.to_account_info()).as_ref())
            .metadata(&ctx.accounts.metadata_account.to_account_info())
            .authority(&ctx.accounts.state.to_account_info())
            .mint(&ctx.accounts.pet_nft_mint.to_account_info())
            .payer(&ctx.accounts.thread.to_account_info())
            .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .data(Data {
                name: ctx.accounts.metadata_account.name.clone(),
                symbol: ctx.accounts.metadata_account.symbol.clone(),
                uri: ctx.accounts.pet_state.get_pet_state_metadata_uri(),
                seller_fee_basis_points: SELLER_FEE,
                creators: ctx.accounts.metadata_account.creators.clone(),
            })
            .is_mutable(true)
            .invoke_signed(&[&[
                PROGRAM_STATE_SEED.as_bytes(),
                &[ctx.accounts.state.bump]
            ],])?;
    }

    UpdatePetStateCron::handle_effects_in_remaining_accounts(
        ctx.remaining_accounts,
        &ctx.accounts.recent_slothashes,
        &mut ctx.accounts.player_state,
        &ctx.accounts.pet_state.condition
    )?;

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
    /// CHECK: verify address
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


    /// Verify that only this thread can execute this intruction
    #[account(mut, signer, constraint = thread.authority.eq(&state.key()))]
    pub thread: Account<'info, Thread>,

    // Programs
    pub metadata_program: Program<'info, MetadataProgram>,

    /// CHECK: Checked in address
    #[account(
        address=Pubkey::from_str(BLOCK_HASHES).unwrap()
    )]
    pub recent_slothashes: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: validated in address
    #[account(address = solana_program::sysvar::instructions::id())]
    pub sysvar_instructions: UncheckedAccount<'info>,
}

impl<'info> UpdatePetStateCron<'info> {
    #[inline(never)]
    pub fn handle_effects_in_remaining_accounts(
        remaining_accounts: &'info [AccountInfo<'info>],
        recent_slothashes: &UncheckedAccount<'info>,
        player_state: &mut Account<'info, PlayerState>,
        pet_condition: &PetCondition
    ) -> Result<()> {
        let data = recent_slothashes.data.borrow();
        let most_recent = array_ref!(data, 12, 8);

        let clock = Clock::get()?;

        // seed for the random number is a combination of the slot_hash - timestamp
        let seed = u64::from_le_bytes(*most_recent).saturating_sub(clock.unix_timestamp as u64);

        remaining_accounts.iter().for_each(|account| {
            let account = Account::<EffectState>::try_from(account);

            if account.is_err() {
                return;
            }

            let effect = account.unwrap().into_inner();

            if effect.is_need_to_auto_set(seed, pet_condition) {
                player_state.add_effect(effect);
            }
        });

        return Ok(());
    }
}