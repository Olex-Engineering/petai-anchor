use anchor_lang::{prelude::*, solana_program::{entrypoint::ProgramResult, instruction::Instruction, native_token::LAMPORTS_PER_SOL}, InstructionData};
use anchor_spl::{token::Mint, metadata::{Metadata as MetadataProgram, MetadataAccount}};
use mpl_token_metadata::accounts::Metadata;


use crate::{constants::{PET_NFT_MINT_SEED, PET_STATE_SEED, PLAYER_CLOCKWORK_FEE_IN_SOL, PLAYER_STATE_CRON_SHEDULER, PLAYER_STATE_SEED, PROGRAM_STATE_SEED, REAL_DOGS_STATE_SEED}, errors::PetaiErrorCode, states::{player_state::PlayerState, program_state::ProgramState}, PetState, RealDogsWalletState, ID};

use clockwork_sdk::state::Thread;

pub fn init_player(
    ctx: Context<InitPlayerState>,
    pet_states: Vec<Vec<String>>,
    thread_id: Vec<u8>,
    real_dog_wallet: Pubkey,
) -> Result<()> {
    msg!("Start init player");
    let is_real_dog_valid = ctx.accounts.real_dogs_config_state.wallets.iter()
        .find(|valid_wallet| Pubkey::eq(&valid_wallet, &real_dog_wallet));

    require!(
        ctx.accounts.metadata_account.collection.as_ref().is_some_and(|collection| collection.verified == true
            && collection.key.eq(&ctx.accounts.state.pet_collection)),
        PetaiErrorCode::InvalidDogNft
    );

    require!(is_real_dog_valid.is_some(), PetaiErrorCode::RealDogValidationError);

    // Set player and pet state
    let mut player_state: PlayerState = PlayerState::default();
    let mut pet_state: PetState = PetState::default();

    pet_state.current_pet_nft = ctx.accounts.pet_nft_mint.key();
    pet_state.bump = ctx.bumps.pet_state;

    player_state.real_dog_treasury = real_dog_wallet;
    player_state.pet_states = pet_states;
    player_state.bump = ctx.bumps.player_state;
    
    ctx.accounts.player_state.set_inner(player_state);
    ctx.accounts.pet_state.set_inner(pet_state);

    start_cron_tread(&ctx, &thread_id)?;

    return Ok(());
}

#[inline(never)]
fn start_cron_tread(ctx: &Context<InitPlayerState>, thread_id: &Vec<u8>) -> ProgramResult {
    let target_ix = Instruction {
        program_id: ID,
        accounts: crate::accounts::UpdatePetStateCron {
            player_state: ctx.accounts.player_state.key(),
            pet_state: ctx.accounts.pet_state.key(),
            state: ctx.accounts.state.key(),
            thread: ctx.accounts.thread.key(),
            metadata_program: ctx.accounts.metadata_program.key(),
            system_program: ctx.accounts.system_program.key(),
        }
        .to_account_metas(Some(true)),
        data: crate::instruction::UpdatePetStateCron {
            player_id: ctx.accounts.signer.key()
        }.data(),
    };

    let trigger = clockwork_sdk::state::Trigger::Cron {
        schedule: PLAYER_STATE_CRON_SHEDULER.into(),
        skippable: true,
    };

    clockwork_sdk::cpi::thread_create(
        CpiContext::new_with_signer(
            ctx.accounts.clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadCreate {
                payer: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                thread: ctx.accounts.thread.to_account_info(),
                authority: ctx.accounts.state.to_account_info(),
            },
            &[&[PROGRAM_STATE_SEED.as_bytes(), &[ctx.accounts.state.bump]]],
        ),
        (PLAYER_CLOCKWORK_FEE_IN_SOL * LAMPORTS_PER_SOL as f64) as u64,       // user clockwork fee
        thread_id.clone(),              // id
        vec![target_ix.into()], // instructions
        trigger,                // trigger
    )?;

    return Ok(());
}

#[derive(Accounts)]
#[instruction(
    pet_states: Vec<Vec<String>>,
    thread_id: Vec <u8>
)]
pub struct InitPlayerState<'info> {
    // states (pda's)
    #[account(
        init,
        seeds=[PLAYER_STATE_SEED.as_bytes(), signer.key.as_ref()],
        bump,
        payer = signer,
        space = PlayerState::get_size(pet_states, None)
    )]
    pub player_state: Account<'info, PlayerState>,
    #[account(
        init,
        seeds=[PET_STATE_SEED.as_bytes(), player_state.key().as_ref()],
        bump,
        payer = signer,
        space = PetState::get_size()
    )]
    pub pet_state: Account<'info, PetState>,
    #[account(
        mut,
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        seeds=[REAL_DOGS_STATE_SEED.as_bytes()],
        bump=real_dogs_config_state.bump

    )]
    pub real_dogs_config_state: Account<'info, RealDogsWalletState>,

    // Pet nft accounts
    #[account(
        seeds=[PET_NFT_MINT_SEED.as_bytes(), signer.key.as_ref()],
        bump,
    )]
    pub pet_nft_mint: Account<'info, Mint>,
    /// CHECK: manual verify address
    #[account(
        mut,
        address=Metadata::find_pda(&pet_nft_mint.key()).0
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    // Signer
    #[account(mut)]
    pub signer: Signer<'info>,

    // Programs
    /// Address to assign to the newly created thread.
    #[account(mut, address = Thread::pubkey(state.key(), thread_id))]
    pub thread: SystemAccount<'info>,
    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    pub metadata_program: Program<'info, MetadataProgram>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}