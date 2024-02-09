pub use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self};
use anchor_spl::{metadata::{mpl_token_metadata::{accounts::{MasterEdition, Metadata}, instructions::UpdateV1CpiBuilder, types::Data}, Metadata as MetadataProgram}, token::{Mint, TokenAccount, Token}, associated_token::AssociatedToken};

use crate::{models::MetatadataArgs, constants::{PROGRAM_STATE_SEED, SELLER_FEE}, states::program_state::ProgramState};

pub fn update_token(ctx: Context<UpdateTokenMetadata>, metadata_args: MetatadataArgs) -> Result<()> {
    let metadata_program_info = ctx.accounts.metadata_program.to_account_info();
    let authority = ctx.accounts.state.to_account_info();
    let token = ctx.accounts.ata_account.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let metadata = ctx.accounts.metadata_account.to_account_info();
    let payer = ctx.accounts.initializer.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();
    let sysvar_instructions = ctx.accounts.sysvar_instructions.to_account_info();
    let master_edition = ctx.accounts.master_edition.as_ref().and_then(|master_edition: &UncheckedAccount<'_>| Some(master_edition.to_account_info()));

    UpdateV1CpiBuilder::new(&metadata_program_info)
        .edition(master_edition.as_ref())
        .metadata(&metadata)
        .authority(&authority)
        .token(Some(&token))
        .mint(&mint)
        .payer(&payer)
        .sysvar_instructions(&sysvar_instructions)
        .system_program(&system_program)
        .data(Data {
            name: metadata_args.name,
            symbol: metadata_args.symbol,
            uri: metadata_args.uri,
            seller_fee_basis_points: SELLER_FEE,
            creators: metadata_args.creators,
        })
        .is_mutable(true)
        .invoke_signed(&[&[
            PROGRAM_STATE_SEED.as_bytes(),
            &[ctx.accounts.state.bump]
        ],])?;


    return Ok(());
}


#[derive(Accounts)]
pub struct UpdateTokenMetadata<'info> {
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    state: Account<'info, ProgramState>,
    
    #[account(
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    ata_account: Account<'info, TokenAccount>,

    #[account()]
    mint: Account<'info, Mint>,
    /// CHECK: verify address
    #[account(
        mut,
        address=Metadata::find_pda(&mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,
    /// CHECK: verify address
    #[account(
        mut,
        address=MasterEdition::find_pda(&mint.key()).0
    )]
    pub master_edition: Option<UncheckedAccount<'info>>,

    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, MetadataProgram>,
    /// CHECK: validated in address
    #[account(address = solana_program::sysvar::instructions::id())]
    pub sysvar_instructions: UncheckedAccount<'info>,
}