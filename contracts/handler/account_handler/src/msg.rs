use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint256;
use verifier::types::Groth16Proof;

#[cw_serde]
pub enum ExecuteMsg {
    CreateAccount {
        email_addr_pointer: Uint256,
        account_key_commit: Uint256,
        wallet_salt: Uint256,
        psi_point: [Uint256; 2],
        proof: Groth16Proof,
    },
}
