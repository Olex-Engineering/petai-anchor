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

    pub fn create_token(ctx: Context<CreateToken>, mint_seed: String, collection_seed: Option<String>, amount: u64, metadata_args: MetatadataArgs) -> Result<()> {
        return instructions::create_token(ctx, mint_seed, collection_seed, amount, metadata_args);
    }

    pub fn update_token(ctx: Context<UpdateTokenMetadata>, mint_seed: String, metadata_args: MetatadataArgs) -> Result<()> {
        return instructions::update_token(ctx, mint_seed, metadata_args);
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
    pub fn create_asset(ctx: Context<CreateAsset>, asset_args: AssetArgs) -> Result<()> {
        return instructions::create_asset(ctx, asset_args);
    }

    pub fn use_asset(ctx: Context<UseAsset>, mint_seed: String, amount: u8) -> Result<()> {
        return instructions::use_asset(ctx, mint_seed, amount);
    }
}


