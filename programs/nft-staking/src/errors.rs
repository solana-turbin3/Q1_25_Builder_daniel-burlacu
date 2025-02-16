use anchor_lang::prelude::*;

#[error_code]
pub enum StakeErrors {
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Collection not verified")]
    CollectionNotVerified,
}

#[error_code]
pub enum UnstakeErrors {
    #[msg("Not enough days staked")]
    NotEnoughDaysStaked,
}