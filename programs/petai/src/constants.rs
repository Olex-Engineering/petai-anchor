// SEEDS
pub const PROGRAM_STATE_SEED: &str = "state";
pub const PET_NFT_MINT_SEED: &str = "pet-nft-mint";
pub const PLAYER_STATE_SEED: &str = "player-state";
pub const REAL_DOGS_STATE_SEED: &str = "real-dogs-state";
pub const PET_STATE_SEED: &str = "pet-state";
pub const ASSET_STATE_SEED: &str = "asset-state";
pub const DECOR_STATE_SEED: &str = "decor-state";

// PERCENTAGES (must be 100 in sum)
pub const TRANSFER_TO_REAL_DOG_PERCENT: u8 = 30;
pub const BURN_PERCENT: u8 = 65;
pub const PROGRAM_COMISSION_PERCENT: u8 = 5;

// CLOCKWORK
// Every second in testing mode
#[cfg(feature= "testing")]
pub const PLAYER_STATE_CRON_SHEDULER: &str = "* * * ? * *";
// Every 1 hour in prod mode
#[cfg(feature= "dev")]
pub const PLAYER_STATE_CRON_SHEDULER: &str = "0 0 0/1 1/1 * ? *";
// Every 4 hour in prod mode
#[cfg(not(any(feature= "testing", feature= "dev")))]
pub const PLAYER_STATE_CRON_SHEDULER: &str = "0 0 0/4 1/1 * ? *";

pub const PLAYER_CLOCKWORK_FEE_IN_SOL: f64 = 0.005856; // For two years

// FEE's
pub const SELLER_FEE: u16 = 500;