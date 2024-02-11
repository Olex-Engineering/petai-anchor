pub use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::{constants::PROGRAM_STATE_SEED, ProgramState};

pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
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
    
    return Ok(());
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump,
    )]
    pub state: Account<'info, ProgramState>,
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    ata_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(mut)]
    pub initializer: Signer<'info>,
}