use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub verifier_addr: Addr,
    pub relayer_handler_addr: Addr,
    pub account_handler_addr: Addr,
}

#[cw_serde]
pub enum QueryMsg {
    VerifierAddr {},
    RelayerHandlerAddr {},
    AccountHandlerAddr {},
}
