use std::str::FromStr;

pub use anchor_lang::prelude::*;
use anchor_lang::{solana_program::{self, instruction::Instruction, native_token::LAMPORTS_PER_SOL}, system_program::{transfer, Transfer}, InstructionData};
use anchor_spl::{metadata::{Metadata as MetadataProgram, MetadataAccount}, token::{Mint, TokenAccount}};

use clockwork_sdk::state::ThreadSettings;
use clockwork_sdk::state::Thread;
use mpl_token_metadata::accounts::{MasterEdition, Metadata};

use crate::{constants::{BLOCK_HASHES, PET_STATE_SEED, PLAYER_CLOCKWORK_FEE_IN_SOL, PLAYER_STATE_SEED, PROGRAM_STATE_SEED}, errors::PetaiErrorCode, PetState, PlayerState, ProgramState, ID};

pub fn update_player_pet(ctx: Context<UpdatePlayerPet>) -> Result<()> {
    require!(ctx.accounts.new_pet_nft_mint_ata.amount == 1, PetaiErrorCode::InvalidPetNftAta);

    ctx.accounts.player_state.current_pet = Some(ctx.accounts.pet_state.key());

    let new_target_ix = Instruction {
        program_id: ID,
        accounts: crate::accounts::UpdatePetStateCron {
            player_state: ctx.accounts.player_state.key(),
            pet_state: ctx.accounts.pet_state.key(),
            state: ctx.accounts.state.key(),
            thread: ctx.accounts.thread.key(),
            pet_nft_mint: ctx.accounts.new_pet_nft_mint.key(),
            metadata_program: ctx.accounts.metadata_program.key(),
            metadata_account: ctx.accounts.metadata_account.key(),
            master_edition: ctx.accounts.master_edition.key(),
            system_program: ctx.accounts.system_program.key(),
            recent_slothashes: Pubkey::from_str(BLOCK_HASHES).unwrap(),
            sysvar_instructions: solana_program::sysvar::instructions::id(),
        }
        .to_account_metas(Some(true)),
        data: crate::instruction::UpdatePetStateCron {
            player_id: ctx.accounts.initializer.key()
        }.data(),
    };

    clockwork_sdk::cpi::thread_update(
        CpiContext::new_with_signer(
            ctx.accounts.clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadUpdate {
                system_program: ctx.accounts.system_program.to_account_info(),
                thread: ctx.accounts.thread.to_account_info(),
                authority: ctx.accounts.state.to_account_info(),
            },
            &[&[PROGRAM_STATE_SEED.as_bytes(), &[ctx.accounts.state.bump]]],
        ),
        ThreadSettings {
            fee: None,
            instructions: Some(vec![new_target_ix.into()]),
            name: None,
            rate_limit: None,
            trigger: None,
        }
    )?;

    // Transfer SOL from payer to the thread.
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.initializer.to_account_info(),
                to: ctx.accounts.thread.to_account_info(),
            },
        ),
        (PLAYER_CLOCKWORK_FEE_IN_SOL * LAMPORTS_PER_SOL as f64) as u64
    )?;
    
    return Ok(());
}

#[derive(Accounts)]
pub struct UpdatePlayerPet<'info> {
    #[account(
        mut,
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    pub state: Account<'info, ProgramState>,
    #[account(
        mut,
        seeds=[PLAYER_STATE_SEED.as_bytes(), initializer.key.as_ref()],
        bump=player_state.bump,
    )]
    pub player_state: Account<'info, PlayerState>,
    #[account(
        mut,
        seeds=[PET_STATE_SEED.as_bytes(), new_pet_nft_mint.key().as_ref()],
        bump=pet_state.bump,
    )]
    pub pet_state: Account<'info, PetState>,
    pub new_pet_nft_mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = new_pet_nft_mint,
        associated_token::authority = initializer
    )]
    pub new_pet_nft_mint_ata: Account<'info, TokenAccount>,
    /// CHECK: manual verify address
    #[account(
        mut,
        address=Metadata::find_pda(&new_pet_nft_mint.key()).0
    )]
    pub metadata_account: Account<'info, MetadataAccount>,
    /// CHECK: verify address
    #[account(
        mut,
        address=MasterEdition::find_pda(&new_pet_nft_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,

    // Signer
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        mut,
        address = Thread::pubkey(state.key(), pet_state.thread_id.clone()),
        constraint = thread.id.eq(&pet_state.thread_id),
        constraint = thread.authority.eq(&state.key()),
    )]
    pub thread: Account<'info, Thread>,
    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    pub metadata_program: Program<'info, MetadataProgram>,
    pub system_program: Program<'info, System>,
}