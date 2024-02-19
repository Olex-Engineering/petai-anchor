pub use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_spl::{metadata::MetadataAccount, token::{Mint, TokenAccount}};
use clockwork_sdk::state::ThreadSettings;
use mpl_token_metadata::accounts::Metadata;

use clockwork_sdk::state::Thread;

use crate::{constants::{PET_STATE_SEED, PLAYER_CLOCKWORK_FEE_IN_SOL, PLAYER_STATE_SEED, PROGRAM_STATE_SEED}, errors::PetaiErrorCode, PetState, PlayerState, ProgramState, UpdatePetNftArgs};

pub fn update_pet_nft(ctx: Context<UpdatePetNft>, update_args: UpdatePetNftArgs) -> Result<()> {
    require!(ctx.accounts.new_pet_nft_mint_ata.amount == 1, PetaiErrorCode::InvalidPetNftAta);

    ctx.accounts.player_state.current_pet_nft = ctx.accounts.new_pet_nft_mint.key();
    ctx.accounts.pet_state.thread_id = update_args.thread_id;

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
            fee: Some((PLAYER_CLOCKWORK_FEE_IN_SOL * LAMPORTS_PER_SOL as f64) as u64),
            instructions: None,
            name: None,
            rate_limit: None,
            trigger: None,
        }
    )?;
    
    return Ok(());
}

#[derive(Accounts)]
#[instruction(update_args: UpdatePetNftArgs)]
pub struct UpdatePetNft<'info> {
    #[account(
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
    #[account(mut, address = Thread::pubkey(state.key(), update_args.thread_id))]
    pub thread: SystemAccount<'info>,
    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}