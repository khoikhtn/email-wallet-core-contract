use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint256};

#[cw_serde]
pub struct InstantiateMsg {
    pub relayer_handler_addr: Addr,
    pub verifier_addr: Addr,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Option<Uint256>)]
    GetRandHash { relayer: Addr },
}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterRelayer {
        rand_hash: Uint256,
        email_addr: String,
        hostname: String,
    },
}
