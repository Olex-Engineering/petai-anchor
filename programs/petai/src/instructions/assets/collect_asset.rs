pub use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::{constants::{ASSET_STATE_SEED, FREE_ASSETS_STATE_SEED, PROGRAM_STATE_SEED}, errors::PetaiErrorCode, AssetState, CollectableAssetState, ProgramState};

pub fn collect_asset(ctx: Context<CollectFreeAssets>) -> Result<()> {
    require!(ctx.accounts.asset_state.is_can_be_collected, PetaiErrorCode::AssetIsNotCollectable);

    let last_time_collected = ctx.accounts.free_asset_state.last_time_collected;
    let current_time = Clock::get().unwrap().unix_timestamp;
    let collectable_time_diff = ctx.accounts.asset_state.collectable_time_diff.unwrap();

    require!(current_time - last_time_collected >= collectable_time_diff, PetaiErrorCode::AssetIsAlreadyCollected);

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.asset_mint.to_account_info(),
                to: ctx.accounts.ata_account.to_account_info(),
                authority: ctx.accounts.state.to_account_info(),
            },
            &[&[
                PROGRAM_STATE_SEED.as_bytes(),
                &[ctx.accounts.state.bump],
            ]]
        ),
        1,
    )?;

    ctx.accounts.free_asset_state.last_time_collected = current_time;

    return Ok(());
}

#[derive(Accounts)]
pub struct CollectFreeAssets<'info> {
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    pub state: Account<'info, ProgramState>,
    #[account(
        seeds=[ASSET_STATE_SEED.as_bytes(), asset_mint.key().as_ref()],
        bump=asset_state.bump,
    )]
    pub asset_state: Account<'info, AssetState>,
    // TODO: add independent instruction for init
    #[account(
        init_if_needed,
        seeds=[FREE_ASSETS_STATE_SEED.as_bytes(), asset_mint.key().as_ref(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = CollectableAssetState::get_size()
    )]
    pub free_asset_state: Account<'info, CollectableAssetState>,
    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint = asset_mint,
        associated_token::authority = signer,
    )]
    pub ata_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub asset_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
}