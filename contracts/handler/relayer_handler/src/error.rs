use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0} is empty")]
    EmptyData(&'static str),

    #[error("{0} is already registered")]
    AlreadyRegistered(&'static str),
}
