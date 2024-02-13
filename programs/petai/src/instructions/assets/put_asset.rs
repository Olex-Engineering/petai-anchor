use anchor_lang::prelude::*;
use crate::{constants::{ASSET_STATE_SEED, PROGRAM_STATE_SEED}, AssetArgs, AssetState, ProgramState};

pub fn put_asset(ctx: Context<PutAsset>, asset_args: AssetArgs) -> Result<()> {
    ctx.accounts.asset_state.set_inner(AssetState {
        key: asset_args.asset_mint,
        increase_food: asset_args.increase_food,
        increase_loneliness: asset_args.increase_loneliness,
        increase_love: asset_args.increase_love,
        price: asset_args.price,
        bump: ctx.bumps.asset_state,
        is_can_be_collected: asset_args.is_can_be_collected,
        collectable_time_diff: asset_args.collectable_time_diff
    });

    return Ok(());
}

#[derive(Accounts)]
#[instruction(asset_args: AssetArgs)]
pub struct PutAsset<'info> {
    // states (pda's)
    #[account(
        init_if_needed,
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
