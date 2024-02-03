// SEEDS
pub const PROGRAM_STATE_SEED: &str = "state";
pub const TOKEN_MINT_SEED: &str = "token-mint";
pub const PET_NFT_MINT_SEED: &str = "pet-nft-mint";
pub const PLAYER_STATE_SEED: &str = "player-state";
pub const PET_STATE_SEED: &str = "pet-state";
pub const ASSET_STATE_SEED: &str = "asset-state";

// CLOCKWORK
// Every second in testing mode
#[cfg(feature= "testing")]
pub const PLAYER_STATE_CRON_SHEDULER: &str = "* * * ? * *";
// Every 4 hour in prod mode
#[cfg(not(feature= "testing"))]
pub const PLAYER_STATE_CRON_SHEDULER: &str = "0 0 0/4 1/1 * ? *";

pub const PLAYER_CLOCKWORK_FEE_IN_SOL: f64 = 0.005856; // For two years

// FEE's
pub const SELLER_FEE: u16 = 500;