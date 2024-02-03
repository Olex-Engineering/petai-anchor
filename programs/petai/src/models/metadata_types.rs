use anchor_lang::prelude::{*};

#[derive(AnchorDeserialize, AnchorSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
    ProgrammableNonFungible,
    ProgrammableNonFungibleEdition,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum CollectionDetails {
    V1 { size: u64 },
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum PrintSupply {
    Zero,
    Limited(u64),
    Unlimited,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}