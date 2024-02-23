use anchor_lang::prelude::*;


use crate::{constants::{PLAYER_STATE_SEED, REAL_DOGS_STATE_SEED}, errors::PetaiErrorCode, states::player_state::PlayerState, RealDogsWalletState};


pub fn init_player(
    ctx: Context<InitPlayerState>,
    real_dog_wallet: Pubkey,
) -> Result<()> {
    let is_real_dog_valid = ctx.accounts.real_dogs_config_state.wallets.iter()
        .find(|valid_wallet| Pubkey::eq(&valid_wallet, &real_dog_wallet));

    require!(is_real_dog_valid.is_some(), PetaiErrorCode::RealDogValidationError);

    // Set player and pet state
    let mut player_state: PlayerState = PlayerState::default();

    player_state.current_pet = None;
    player_state.real_dog_treasury = real_dog_wallet;
    player_state.bump = ctx.bumps.player_state;
    
    ctx.accounts.player_state.set_inner(player_state);

    return Ok(());
}

#[derive(Accounts)]

pub struct InitPlayerState<'info> {
    // states (pda's)
    #[account(
        init,
        seeds=[PLAYER_STATE_SEED.as_bytes(), initializer.key.as_ref()],
        bump,
        payer = initializer,
        space = PlayerState::get_size(None)
    )]
    pub player_state: Account<'info, PlayerState>,


    #[account(
        seeds=[REAL_DOGS_STATE_SEED.as_bytes()],
        bump=real_dogs_config_state.bump

    )]
    pub real_dogs_config_state: Account<'info, RealDogsWalletState>,

    // Signer
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}