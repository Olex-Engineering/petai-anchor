mod utils;
mod states;
mod instructions;
mod errors;
mod constants;
mod models;

use instructions::*;
use states::*;
use models::*;
declare_id!("47WMeQhwKsEr9RgTxiTupUJKtohL9Rd1A5GyemBqsbKH");

#[program]
pub mod petai {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        return instructions::initialize(ctx);
    }

    pub fn update_program_state(ctx: Context<UpdateProgramState>, data: ProgramState) -> Result<()> {
        return instructions::update_program_state(ctx, data);
    }

    pub fn create_token(ctx: Context<CreateToken>, mint_seed: String, amount: u64, metadata_args: MetatadataArgs) -> Result<()> {
        return instructions::create_token(ctx, mint_seed, amount, metadata_args);
    }

    pub fn update_token(ctx: Context<UpdateTokenMetadata>, metadata_args: MetatadataArgs) -> Result<()> {
        return instructions::update_token(ctx, metadata_args);
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        return instructions::mint_token(ctx, amount);
    }

    pub fn init_player_state(
        ctx: Context<InitPlayerState>,
        pet_states: Vec<Vec<String>>,
        real_dog_config: RealDogConfig,
        thread_id: Vec<u8>,
    ) -> Result<()> {
        return instructions::init_player(
            ctx,
            pet_states,
            real_dog_config,
            thread_id,
        );
    }

    pub fn update_pet_state_cron(ctx: Context<UpdatePetStateCron>, player_id: Pubkey) -> Result<()> {
        return instructions::update_player_state_cron(ctx, player_id);
    }

    // Assets
    pub fn put_asset(ctx: Context<PutAsset>, asset_args: AssetArgs) -> Result<()> {
        return instructions::put_asset(ctx, asset_args);
    }

    pub fn use_asset(ctx: Context<UseAsset>, mint_seed: String, amount: u8) -> Result<()> {
        return instructions::use_asset(ctx, mint_seed, amount);
    }

    // Decors
    pub fn put_decor(ctx: Context<PutDecor>, decor_args: DecorArgs) -> Result<()> {
        return instructions::put_decor(ctx, decor_args);
    }
}


