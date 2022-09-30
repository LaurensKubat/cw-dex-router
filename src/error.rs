use cosmwasm_std::{OverflowError, StdError};
use cw_dex::CwDexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CwDexError(#[from] CwDexError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Incorrect amount of native token sent. You don't need to pass in offer_amount if using native tokens.")]
    IncorrectNativeAmountSent,

    #[error("Unsupported asset type. Only native and cw20 tokens are supported.")]
    UnsupportedAssetType,

    #[error("No swap operations provided")]
    MustProvideOperations,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid swap operations")]
    InvalidSwapOperations,

    #[error("Did not receive minimum amount")]
    FailedMinimumReceive,
}

impl From<ContractError> for StdError {
    fn from(x: ContractError) -> Self {
        Self::generic_err(x.to_string())
    }
}