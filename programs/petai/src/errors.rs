use anchor_lang::prelude::*;

#[error_code]
pub enum PetaiErrorCode {
    #[msg("Invalid pet states provided")]
    PetStatesArgsError,
    #[msg("Invalid real dog config provided")]
    RealDogValidationError,
    #[msg("Invalid Dog NFT (invalid collection or collection is not verified)")]
    InvalidDogNft
}
