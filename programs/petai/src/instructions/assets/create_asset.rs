use anchor_lang::prelude::*;
use crate::{AssetState, ProgramState, constants::{PROGRAM_STATE_SEED, ASSET_STATE_SEED}};

pub fn create_asset(ctx: Context<CreateAsset>, asset_args: AssetArgs) -> Result<()> {
    ctx.accounts.asset_state.set_inner(AssetState {
        key: asset_args.asset_mint,
        increase_food: asset_args.increase_food,
        increase_loneliness: asset_args.increase_loneliness,
        increase_love: asset_args.increase_love,
        bump: ctx.bumps.asset_state
    });

    return Ok(());
}

#[derive(Accounts)]
#[instruction(asset_args: AssetArgs)]
pub struct CreateAsset<'info> {
    // states (pda's)
    #[account(
        init,
        seeds=[ASSET_STATE_SEED.as_bytes(), asset_args.asset_mint.as_ref()],
        bump,
        payer = signer,
        space = AssetState::get_size()
    )]
    pub asset_state: Account<'info, AssetState>,

    #[account(
        seeds=[PROGRAM_STATE_SEED.as_bytes()],
        bump=state.bump
    )]
    pub state: Account<'info, ProgramState>,

    // Signer
    #[account(
        mut,
        address=state.authority.key()
    )]
    pub signer: Signer<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AssetArgs {
    pub asset_mint: Pubkey,
    pub increase_food: u8,
    pub increase_loneliness: u8,
    pub increase_love: u8,
}