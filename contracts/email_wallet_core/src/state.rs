use cosmwasm_std::Addr;
use cw_storage_plus::Item;

/// ZK proof verifier
pub(crate) const VERIFIER: Item<Addr> = Item::new("verifier");

/// Relayer handler - Methods to create and update relayer config
pub(crate) const RELAYER_HANDLER: Item<Addr> = Item::new("relayer handler");

/// Account handler - Methods to create, initialize, transport user account and settings
pub(crate) const ACCOUNT_HANDLER: Item<Addr> = Item::new("account handler");


