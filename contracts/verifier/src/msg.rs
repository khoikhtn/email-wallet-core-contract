use crate::types::Groth16Proof;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint256;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(())]
    AccountCreationProof {
        relayer_hash: Uint256,
        email_addr_pointer: Uint256,
        account_key_commit: Uint256,
        wallet_salt: Uint256,
        psi_point: [Uint256; 2],
        proof: Groth16Proof,
    },
}
