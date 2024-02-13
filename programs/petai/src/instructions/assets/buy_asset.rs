pub use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{burn, mint_to, transfer, Burn, Mint, MintTo, Token, TokenAccount, Transfer}};

use crate::{constants::{BURN_PERCENT, PLAYER_STATE_SEED, PROGRAM_COMISSION_PERCENT, PROGRAM_STATE_SEED, TRANSFER_TO_REAL_DOG_PERCENT}, AssetState, PlayerState, ProgramState};

#[inline(never)]
pub fn buy_asset(ctx: Context<BuyAsset>, amount: u32) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();

    let total_payement_amount = ctx.accounts.asset_state.price * u64::from(amount)
        .checked_mul(10u64.pow(ctx.accounts.token_mint.decimals as u32))
        .unwrap();

    let burn_amount = total_payement_amount * u64::from(BURN_PERCENT) / 100;
    let transfer_to_real_dog_treasury_amount = total_payement_amount * u64::from(TRANSFER_TO_REAL_DOG_PERCENT) / 100;
    let transfer_to_treasury_amount = total_payement_amount * u64::from(PROGRAM_COMISSION_PERCENT) / 100;


    transfer(
        CpiContext::new(
            cpi_program.clone(),
            Transfer {
                from: ctx.accounts.signer_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.initializer.to_account_info(),
            }
        ),
        transfer_to_treasury_amount
    )?;

    transfer(
        CpiContext::new(
            cpi_program.clone(),
            Transfer {
                from: ctx.accounts.signer_token_account.to_account_info(),
                to: ctx.accounts.real_dog_token_account.to_account_info(),
                authority: ctx.accounts.initializer.to_account_info(),
            }
        ),
        transfer_to_real_dog_treasury_amount
    )?;

    burn(
        CpiContext::new(cpi_program.clone(), Burn {
            mint: ctx.accounts.token_mint.to_account_info(),
            from: ctx.accounts.signer_token_account.to_account_info(),
            authority: ctx.accounts.initializer.to_account_info()
        }),
        burn_amount
    )?;

    mint_to(
        CpiContext::new_with_signer(
            cpi_program,
            MintTo {
                mint: ctx.accounts.asset_mint.to_account_info(),
                to: ctx.accounts.signer_asset_account.to_account_info(),
                authority: ctx.accounts.state.to_account_info(),
        },
    &[&[
            PROGRAM_STATE_SEED.as_bytes(),
            &[ctx.accounts.state.bump]
        ]]),
        amount.into()
    )?;

    return Ok(());
}

#[derive(Accounts)]
pub struct BuyAsset<'info> {
    // States
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump
    )]
    pub state: Box<Account<'info, ProgramState>>,
    #[account(
        seeds=[PLAYER_STATE_SEED.as_bytes(), initializer.key.as_ref()],
        bump,
    )]
    pub player_state: Box<Account<'info, PlayerState>>,
    pub asset_state: Box<Account<'info, AssetState>>,

    // Ata accounts
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = initializer
    )]
    pub signer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = state
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = real_dog_treasury
    )]
    pub real_dog_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = asset_mint,
        associated_token::authority = initializer
    )]
    pub signer_asset_account: Account<'info, TokenAccount>,

    // Mints
    #[account(mut)]
    pub asset_mint: Account<'info, Mint>,
    #[account(
        mut,
        address=state.token_mint
    )]
    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    /// CHECK: manual verify address
    #[account(
        address=player_state.real_dog_treasury
    )]
    pub real_dog_treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>
}