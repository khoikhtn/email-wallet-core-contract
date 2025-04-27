use crate::error::ContractError;
use crate::state::{BASE_PROOF, EXTRA_PROOF};
use crate::tasks::Tasks;
use crate::types::Groth16Proof;
use crate::utils::verify_proof;
use cosmwasm_std::{Deps, Uint256};

pub fn verify_account_creation_proof(
    deps: &Deps,
    relayer_hash: &Uint256,
    email_addr_pointer: &Uint256,
    account_key_commit: &Uint256,
    wallet_salt: &Uint256,
    psi_point: [&Uint256; 2],
    proof: &Groth16Proof,
) -> Result<(), ContractError> {
    let pub_signals = vec![
        relayer_hash,
        email_addr_pointer,
        account_key_commit,
        wallet_salt,
        psi_point[0],
        psi_point[1],
    ];
    verify_proof(
        &BASE_PROOF.load(deps.storage)?,
        &EXTRA_PROOF.load(deps.storage, Tasks::CREATE_ACCOUNT)?,
        proof,
        &pub_signals,
    )
}
