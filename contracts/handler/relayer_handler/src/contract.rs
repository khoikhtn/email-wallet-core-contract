use crate::contract::execute::register_relayer;
use crate::contract::query::get_rand_hash;
use crate::error::ContractError;
use crate::msg::QueryMsg::GetRandHash;
use crate::msg::{ExecuteMsg, QueryMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterRelayer {
            rand_hash,
            email_addr,
            hostname,
        } => register_relayer(deps, info, rand_hash, email_addr, hostname),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        GetRandHash { relayer } => to_json_binary(&get_rand_hash(&deps, &relayer)),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::default())
}

pub mod execute {
    use crate::error::ContractError;
    use crate::error::ContractError::{AlreadyRegistered, EmptyData};
    use crate::state::{
        RelayerConfig, RELAYER_CONFIG, RELAYER_OF_EMAIL_ADDR, RELAYER_OF_RAND_HASH,
    };
    use cosmwasm_std::{ensure, DepsMut, MessageInfo, Response, Uint256};

    pub fn register_relayer(
        deps: DepsMut,
        info: MessageInfo,
        rand_hash: Uint256,
        email_addr: String,
        hostname: String,
    ) -> Result<Response, ContractError> {
        let sender = info.sender;
        let rand_hash_str = rand_hash.to_string();
        ensure!(rand_hash != Uint256::zero(), EmptyData("rand_hash"));
        ensure!(!email_addr.is_empty(), EmptyData("email_addr"));
        ensure!(!hostname.is_empty(), EmptyData("hostname"));
        ensure!(
            !RELAYER_CONFIG.has(deps.storage, sender.clone()),
            AlreadyRegistered("relayer")
        );
        ensure!(
            !RELAYER_OF_RAND_HASH.has(deps.storage, rand_hash_str.as_str()),
            AlreadyRegistered("rand_hash")
        );
        ensure!(
            !RELAYER_OF_EMAIL_ADDR.has(deps.storage, email_addr.as_str()),
            AlreadyRegistered("email_addr")
        );

        RELAYER_OF_RAND_HASH.save(deps.storage, rand_hash_str.as_str(), &sender)?;
        RELAYER_OF_EMAIL_ADDR.save(deps.storage, email_addr.as_str(), &sender)?;
        RELAYER_CONFIG.save(
            deps.storage,
            sender.clone(),
            &RelayerConfig {
                rand_hash,
                email_addr,
                hostname,
            },
        )?;
        Ok(Response::default())
    }
}

pub mod query {
    use crate::state::RELAYER_CONFIG;
    use cosmwasm_std::{Addr, Deps, Uint256};

    pub fn get_rand_hash(deps: &Deps, relayer: &Addr) -> Option<Uint256> {
        match RELAYER_CONFIG.load(deps.storage, relayer.clone()) {
            Ok(data) => Some(data.rand_hash),
            Err(_) => None,
        }
    }
}
