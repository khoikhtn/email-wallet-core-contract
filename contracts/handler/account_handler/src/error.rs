use cosmwasm_std::StdError;
use thiserror::Error;
use verifier::error::ContractError as VerifierError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0} is not registered")]
    NotRegistered(&'static str),

    #[error("{0} is already existed")]
    IsExists(&'static str),

    #[error("{0}")]
    VerifierError(#[from] VerifierError),
}

// impl From<VerifierError> for ContractError {
//     fn from(err: VerifierError) -> Self {
//         match err {
//             VerifierError::Std(err) => ContractError::Std(err),
//             VerifierError::NotInBaseField(_) => {}
//             VerifierError::NotInScalarField(_) => {}
//             VerifierError::MalformedProof => {}
//             VerifierError::PointNotOnCurve => {}
//             VerifierError::PairingFailed => {}
//         }
//     }
//
// }
