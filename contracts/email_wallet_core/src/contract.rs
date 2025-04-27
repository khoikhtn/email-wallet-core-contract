use crate::msg::{InstantiateMsg, QueryMsg};
use crate::state::{ACCOUNT_HANDLER, RELAYER_HANDLER, VERIFIER};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    VERIFIER.save(deps.storage, &msg.verifier_addr)?;
    RELAYER_HANDLER.save(deps.storage, &msg.relayer_handler_addr)?;
    ACCOUNT_HANDLER.save(deps.storage, &msg.account_handler_addr)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    to_json_binary(&match msg {
        QueryMsg::VerifierAddr {} => VERIFIER.load(deps.storage)?,
        QueryMsg::RelayerHandlerAddr {} => RELAYER_HANDLER.load(deps.storage)?,
        QueryMsg::AccountHandlerAddr {} => ACCOUNT_HANDLER.load(deps.storage)?
    })
}
