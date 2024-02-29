use anchor_lang::{prelude::*, solana_program};
use anchor_spl::{metadata::{mpl_token_metadata::{instructions::UpdateV1CpiBuilder, types::Data}, MasterEditionAccount, Metadata as MetadataProgram, MetadataAccount}, token::{burn, Burn, Mint, Token, TokenAccount}};
use mpl_token_metadata::accounts::{MasterEdition, Metadata};

use crate::{constants::{ASSET_STATE_SEED, PET_STATE_SEED, PLAYER_STATE_SEED, PROGRAM_STATE_SEED, SELLER_FEE}, AssetState, EffectState, PetState, PetStateUpdates, PlayerState, ProgramState};

pub fn use_asset(ctx: Context<UseAsset>, _: String, amount: u8) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let burn_cpi_accounts = Burn {
        mint: ctx.accounts.asset_mint.to_account_info(),
        from: ctx.accounts.ata_account.to_account_info(),
        authority: ctx.accounts.initializer.to_account_info()
    };

    burn(
        CpiContext::new(cpi_program, burn_cpi_accounts),
        amount.into()
    )?;

    let asset = &ctx.accounts.asset_state;
    let pet = &mut ctx.accounts.pet_state;

    pet
        .incease_pet_state_params(&PetStateUpdates {
            food: asset.increase_food * amount,
            loneliness: asset.increase_loneliness * amount,
            love: asset.increase_love * amount
        })
        .update_age_if_needed();

    let is_condition_updated = pet.update_condition_if_needed();
    let is_age_updated = pet.update_age_if_needed();

    // Update METADATA Uri depends on condition and age
    if is_condition_updated || is_age_updated {
        UpdateV1CpiBuilder::new(&ctx.accounts.metadata_program.to_account_info())
            .edition(Some(ctx.accounts.master_edition.to_account_info()).as_ref())
            .metadata(&ctx.accounts.metadata_account.to_account_info())
            .authority(&ctx.accounts.state.to_account_info())
            .mint(&ctx.accounts.pet_nft_mint.to_account_info())
            .payer(&ctx.accounts.initializer.to_account_info())
            .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .data(Data {
                name: ctx.accounts.metadata_account.name.clone(),
                symbol: ctx.accounts.metadata_account.symbol.clone(),
                uri: ctx.accounts.metadata_account.uri.clone(),
                seller_fee_basis_points: SELLER_FEE,
                creators: ctx.accounts.metadata_account.creators.clone(),
            })
            .is_mutable(true)
            .invoke_signed(&[&[
                PROGRAM_STATE_SEED.as_bytes(),
                &[ctx.accounts.state.bump]
            ],])?;
    }


    if asset.add_effect.is_some() && ctx.accounts.add_effect.is_some() {
        ctx.accounts.player_state.add_effect(ctx.accounts.add_effect.as_ref().unwrap().clone().into_inner());
    }

    if asset.remove_effect.is_some() && ctx.accounts.remove_effect.is_some() {
        ctx.accounts.player_state.remove_effect(ctx.accounts.remove_effect.as_ref().unwrap().clone().into_inner());
    }

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
        seeds=[PET_STATE_SEED.as_bytes(), pet_nft_mint.key().as_ref()],
        bump=pet_state.bump
    )]
    pub pet_state: Account<'info, PetState>,

    #[account(
        mut,
        seeds=[PLAYER_STATE_SEED.as_bytes(), initializer.key.as_ref()],
        bump=player_state.bump,
    )]
    pub player_state: Account<'info, PlayerState>,

    #[account(
        seeds=[ASSET_STATE_SEED.as_bytes(), asset_mint.key().as_ref()],
        bump=asset_state.bump
    )]
    pub asset_state: Account<'info, AssetState>,

    #[account(
        address=asset_state.remove_effect.unwrap().key()
    )]
    pub remove_effect: Option<Account<'info, EffectState>>,
    #[account(
        address=asset_state.add_effect.unwrap().key()
    )]
    pub add_effect: Option<Account<'info, EffectState>>,

    // Pet nft accounts
    #[account(
        mut,
        seeds=[mint_seed.as_bytes(), state.authority.as_ref()],
        bump,
    )]
    pub asset_mint: Account<'info, Mint>,

    pub pet_nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = asset_mint,
        associated_token::authority = initializer
    )]
    pub ata_account: Account<'info, TokenAccount>,

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
    pub master_edition: Account<'info, MasterEditionAccount>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    // Programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    // Programs
    pub metadata_program: Program<'info, MetadataProgram>,
     /// CHECK: validated in address
     #[account(address = solana_program::sysvar::instructions::id())]
     pub sysvar_instructions: UncheckedAccount<'info>,
}