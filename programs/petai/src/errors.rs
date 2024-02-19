use anchor_lang::prelude::*;

#[error_code]
pub enum PetaiErrorCode {
    #[msg("Invalid pet states provided")]
    PetStatesArgsError,
    #[msg("Invalid real dog config provided")]
    RealDogValidationError,
    #[msg("Invalid Dog NFT (invalid collection or collection is not verified)")]
    InvalidDogNft,
    #[msg("Invalid Pet NFT (signer doesn't have such NFT on ata account)")]
    InvalidPetNftAta,
    #[msg("Provided asset is not collectable")]
    AssetIsNotCollectable,
    #[msg("Provided asset is already collected, please wait for the next collectable time")]
    AssetIsAlreadyCollected,
}
