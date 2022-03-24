use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    // Add any other custom errors you like here.
    #[error("InvalidNumTokens {max}, min: 1")]
    InvalidNumTokens { max: u32, min: u32 },

    #[error("Instantiate cw721 error")]
    InstantiateCW721Error {},

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Invalid token id")]
    InvalidTokenId {},

    #[error("Token id: {token_id} already sold")]
    TokenIdAlreadySold { token_id: u32 },

    #[error("Sold out")]
    SoldOut {},

    #[error("Max minting limit per transaction exceeded")]
    MaxPerTxLimitExceeded {},
}
