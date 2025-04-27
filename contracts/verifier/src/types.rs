use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint256;
use pairing_ce::bn256::{Fq, Fr};

pub type BaseField = Fq;
pub type ScalarField = Fr;

#[cw_serde]
pub struct Groth16Proof {
    pub pi_a: [Uint256; 2],
    pub pi_b: [[Uint256; 2]; 2],
    pub pi_c: [Uint256; 2],
}
