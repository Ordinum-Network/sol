use anchor_lang::prelude::*;

#[error_code]
pub enum OrdinumError {
    #[msg("sponsor not verified")]
    SponsorNotVerified,

    #[msg("end date is smaller than start date")]
    InvalidDate,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Trial Not Initialised")]
    InvalidTrial,
}