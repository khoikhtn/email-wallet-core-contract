use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint256};
use cw_storage_plus::Map;

#[cw_serde]
pub struct RelayerConfig {
    pub rand_hash: Uint256,
    pub email_addr: String,
    pub hostname: String,
}

pub(crate) const RELAYER_CONFIG: Map<Addr, RelayerConfig> = Map::new("RELAYER_CONFIG");
pub(crate) const RELAYER_OF_RAND_HASH: Map<&str, Addr> = Map::new("RELAYER_OF_RAND_HASH");
pub(crate) const RELAYER_OF_EMAIL_ADDR: Map<&str, Addr> = Map::new("RELAYER_OF_EMAIL_ADDR");
