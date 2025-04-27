use cosmwasm_std::{StdError, Uint256};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0} is not in base field")]
    NotInBaseField(Uint256),

    #[error("{0} is not in scalar field")]
    NotInScalarField(Uint256),

    #[error("Proof is malformed")]
    MalformedProof,

    #[error("Point is not on curve")]
    PointNotOnCurve,

    #[error("Pairing failed")]
    PairingFailed,
}
