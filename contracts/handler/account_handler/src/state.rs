use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint256};
use cw_storage_plus::{Item, Map};

pub type U256Str = String;
pub type VecU256Str = String;

#[cw_serde]
pub struct AccountKeyInfo {
    pub relayer: Addr,
    pub initialized: bool,
    pub wallet_salt: Uint256,
}

pub(crate) const RELAYER_HANDLER: Item<Addr> = Item::new("relayer handler");
pub(crate) const VERIFIER: Item<Addr> = Item::new("verifier");

pub(crate) const ACCOUNT_KEY_COMMIT_OF_POINTER: Map<U256Str, Uint256> =
    Map::new("ACCOUNT_KEY_COMMIT_OF_POINTER");

pub(crate) const POINTER_OF_PSI_POINT: Map<VecU256Str, Uint256> = Map::new("POINTER_OF_PSI_POINT");
pub(crate) const INFO_OF_ACCOUNT_KEY_COMMIT: Map<U256Str, AccountKeyInfo> =
    Map::new("INFO_OF_ACCOUNT_KEY_COMMIT");
