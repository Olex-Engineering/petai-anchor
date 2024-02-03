pub use anchor_lang::prelude::*;
use anchor_spl::metadata::mpl_token_metadata::types::{Collection, CollectionDetails, Creator, PrintSupply, TokenStandard};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MetatadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub token_standart: TokenStandard,
    pub primary_sale_happened: bool,
    pub collection: Option<Collection>,
    pub collection_details: Option<CollectionDetails>,
    pub decimals: Option<u8>,
    pub print_supply: Option<PrintSupply>,
    pub creators: Option<Vec<Creator>>
}