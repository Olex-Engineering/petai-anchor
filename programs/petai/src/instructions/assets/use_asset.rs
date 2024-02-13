use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};
use mpl_token_metadata::accounts::Metadata;

use crate::{constants::{ASSET_STATE_SEED, PET_STATE_SEED, PLAYER_STATE_SEED, PROGRAM_STATE_SEED}, AssetState, PetState, PetStateUpdates, PlayerState, ProgramState};

pub fn use_asset(ctx: Context<UseAsset>, _: String, amount: u8) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let burn_cpi_accounts = Burn {
        mint: ctx.accounts.asset_mint.to_account_info(),
        from: ctx.accounts.ata_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info()
    };

    burn(
        CpiContext::new(cpi_program, burn_cpi_accounts),
        amount.into()
    )?;

    let asset = &ctx.accounts.asset_state;
    let pet = &mut ctx.accounts.pet_state;

    pet.incease_pet_state_params(PetStateUpdates {
        food: asset.increase_food * amount,
        loneliness: asset.increase_loneliness * amount,
        love: asset.increase_love * amount
    });

    pet.update_condition_if_needed();
    pet.update_age_if_needed();

    return Ok(());
}

#[derive(Accounts)]
#[instruction(mint_seed: String)]
pub struct UseAsset<'info> {
    // states (pda's)
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        mut,
        seeds=[PLAYER_STATE_SEED.as_bytes(), signer.key.as_ref()],
        bump=player_state.bump
    )]
    pub player_state: Account<'info, PlayerState>,

    #[account(
        mut,
        seeds=[PET_STATE_SEED.as_bytes(), player_state.key().as_ref()],
        bump=pet_state.bump
    )]
    pub pet_state: Account<'info, PetState>,

    #[account(
        seeds=[ASSET_STATE_SEED.as_bytes(), asset_mint.key().as_ref()],
        bump=asset_state.bump
    )]
    pub asset_state: Account<'info, AssetState>,

    // Pet nft accounts
    #[account(
        mut,
        seeds=[mint_seed.as_bytes(), state.authority.as_ref()],
        bump,
    )]
    pub asset_mint: Account<'info, Mint>,

    /// CHECK: manual verify address
    #[account(
        mut,
        address=Metadata::find_pda(&asset_mint.key()).0
    )]
    pub asset_metadata_account: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = asset_mint,
        associated_token::authority = signer
    )]
    pub ata_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    // Programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
     /// CHECK: validated in address
     #[account(address = solana_program::sysvar::instructions::id())]
     pub sysvar_instructions: UncheckedAccount<'info>,
}