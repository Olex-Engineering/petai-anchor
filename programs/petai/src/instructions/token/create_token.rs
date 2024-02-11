use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, Token, TokenAccount, MintTo, mint_to}, metadata::{mpl_token_metadata::{accounts::{MasterEdition, Metadata}, instructions::{CreateV1Cpi, CreateV1CpiAccounts, CreateV1InstructionArgs, VerifyCollectionV1Cpi, VerifyCollectionV1CpiAccounts}}, Metadata as MetadataProgram}, associated_token::AssociatedToken};

use crate::{constants::{PROGRAM_STATE_SEED, SELLER_FEE}, states::program_state::ProgramState, models::MetatadataArgs};

/// Creates a new token.
///
/// # Arguments
///
/// * `ctx` - The context in which this function is called.
/// * `mint_seed` - Mint seed.
/// * `collection_seed` - An optional seed for the collection.
/// * `amount` - The amount of the token to mint.
/// * `metadata_args` - The metadata arguments for the token.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the token is successfully created, otherwise returns an error.
pub fn create_token(ctx: Context<CreateToken>, _mint_seed: String, amount: u64, metadata_args: MetatadataArgs) -> Result<()> {
    let metadata_program_info = &ctx.accounts.metadata_program.to_account_info();
    let metadata_info = &ctx.accounts.metadata_account.to_account_info();
    let mint_info = &ctx.accounts.mint.to_account_info();
    let state_info = &ctx.accounts.state.to_account_info();
    let signer_info = &ctx.accounts.initializer.to_account_info();
    let system_program_info = &ctx.accounts.system_program.to_account_info();
    let sysvar_info = &ctx.accounts.rent.to_account_info();
    let spl_token_program_info = &ctx.accounts.token_program.to_account_info();

    let master_edition_info: Option<AccountInfo<'_>> = ctx.accounts.master_edition.as_ref().and_then(|master_edition: &UncheckedAccount<'_>| Some(master_edition.to_account_info()));

    let _cpi_create = CreateV1Cpi::new(
        &metadata_program_info,
        CreateV1CpiAccounts {
            metadata: metadata_info,
            master_edition: master_edition_info.as_ref(),
            mint: (mint_info, false),
            authority: state_info,
            payer: signer_info,
            update_authority: (state_info, true),
            system_program: system_program_info,
            sysvar_instructions: sysvar_info,
            spl_token_program: spl_token_program_info,
        }, CreateV1InstructionArgs {
            name: metadata_args.name,
            symbol: metadata_args.symbol,
            uri: metadata_args.uri,
            seller_fee_basis_points: SELLER_FEE,
            creators: metadata_args.creators,
            primary_sale_happened: metadata_args.primary_sale_happened,
            is_mutable: true,
            token_standard: metadata_args.token_standart,
            collection: metadata_args.collection,
            uses: None,
            collection_details: metadata_args.collection_details,
            rule_set: None,
            decimals: metadata_args.decimals,
            print_supply: metadata_args.print_supply,
        }
    );
   

    msg!(amount.to_string().as_str());
    mint_to(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.ata_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        },
        &[&[
            PROGRAM_STATE_SEED.as_bytes(),
            &[ctx.accounts.state.bump]
        ]]
    ), amount)?;


    _cpi_create.invoke_signed(&[
        &[
            PROGRAM_STATE_SEED.as_bytes(),
            &[ctx.accounts.state.bump]
        ]
    ])?;

    msg!("Created");

    if ctx.accounts.collection_mint.is_some() {
        let collection_mint_info = ctx.accounts.collection_mint.as_ref().unwrap().to_account_info();
        let collection_metadata_info = ctx.accounts.collection_metadata.as_ref().unwrap().to_account_info();
        let collection_master_edition_info = ctx.accounts.collection_master_edition.as_ref().unwrap().to_account_info();

        // Verify pet nft collection
        let _cpi_verify = VerifyCollectionV1Cpi::new(
            &metadata_program_info,
            VerifyCollectionV1CpiAccounts {
                authority: &state_info,
                delegate_record: None,
                metadata: &metadata_info,
                collection_mint: &collection_mint_info,
                collection_metadata: Some(&collection_metadata_info),
                collection_master_edition: Some(&collection_master_edition_info),
                system_program: &system_program_info,
                sysvar_instructions: &sysvar_info,
            }
        );

        _cpi_verify.invoke_signed(&[&[
            PROGRAM_STATE_SEED.as_bytes(),
            &[ctx.accounts.state.bump]
        ]])?;

        msg!("verified");
    }

    return Ok(());
}

#[derive(Accounts)]
#[instruction(mint_seed: String)]
pub struct CreateToken<'info> {
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    state: Account<'info, ProgramState>,

    #[account(
        init,
        seeds=[mint_seed.as_bytes(), initializer.key.as_ref()],
        bump,
        payer=initializer,
        mint::decimals=0,
        mint::authority=state,
        mint::freeze_authority=state
    )]
    pub mint: Account<'info, Mint>,
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

    // Collection accounts
    #[account()]
    pub collection_mint: Option<Account<'info, Mint>>,
    /// CHECK: verify address
    #[account(
        mut,
        address=Metadata::find_pda(&collection_mint.as_ref().unwrap().key()).0
    )]
    pub collection_metadata: Option<UncheckedAccount<'info>>,
    /// CHECK: verify address
    #[account(
        mut,
        address=MasterEdition::find_pda(&collection_mint.as_ref().unwrap().key()).0
    )]
    pub collection_master_edition: Option<UncheckedAccount<'info>>,


    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    ata_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, MetadataProgram>,
    pub rent: Sysvar<'info, Rent>,
}