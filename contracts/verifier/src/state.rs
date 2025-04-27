use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint256;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct BaseProof {
    pub r: Uint256,
    pub q: Uint256,
    pub alphax: Uint256,
    pub alphay: Uint256,
    pub betax1: Uint256,
    pub betax2: Uint256,
    pub betay1: Uint256,
    pub betay2: Uint256,
    pub gammax1: Uint256,
    pub gammax2: Uint256,
    pub gammay1: Uint256,
    pub gammay2: Uint256,
}

#[cw_serde]
pub struct ExtraProof {
    pub deltax1: Uint256,
    pub deltax2: Uint256,
    pub deltay1: Uint256,
    pub deltay2: Uint256,
    pub ic_x: Vec<Uint256>,
    pub ic_y: Vec<Uint256>,
}

pub(crate) const BASE_PROOF: Item<BaseProof> = Item::new("BASE_PROOF");
pub(crate) const EXTRA_PROOF: Map<&'static str, ExtraProof> = Map::new("EXTRA_PROOF");
