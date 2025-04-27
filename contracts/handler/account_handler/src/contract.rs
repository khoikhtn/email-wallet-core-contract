use crate::error::ContractError;
use crate::error::ContractError::{IsExists, NotRegistered};
use crate::msg::ExecuteMsg;
use crate::msg::ExecuteMsg::CreateAccount;
use crate::state::{
    AccountKeyInfo, ACCOUNT_KEY_COMMIT_OF_POINTER, INFO_OF_ACCOUNT_KEY_COMMIT,
    POINTER_OF_PSI_POINT, RELAYER_HANDLER, VERIFIER,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult, Uint256,
};
use relayer_handler::msg::InstantiateMsg;

// instantiate the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    VERIFIER.save(deps.storage, &msg.verifier_addr)?;
    RELAYER_HANDLER.save(deps.storage, &msg.relayer_handler_addr)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        CreateAccount {
            email_addr_pointer,
            account_key_commit,
            wallet_salt,
            psi_point,
            proof,
        } => {
            let relayer_rand_hash: Option<Uint256> = deps.querier.query_wasm_smart(
                RELAYER_HANDLER.load(deps.storage)?,
                &relayer_handler::msg::QueryMsg::GetRandHash {
                    relayer: info.sender.clone(),
                },
            )?;
            ensure!(relayer_rand_hash.is_some(), NotRegistered("relayer"));
            ensure!(
                !ACCOUNT_KEY_COMMIT_OF_POINTER.has(deps.storage, email_addr_pointer.to_string()),
                IsExists("pointer")
            );
            ensure!(
                !POINTER_OF_PSI_POINT.has(deps.storage, format!("{:?}", psi_point)),
                IsExists("PSI point")
            );
            ensure!(
                !INFO_OF_ACCOUNT_KEY_COMMIT.has(deps.storage, account_key_commit.to_string()),
                IsExists("wallet salt")
            );

            // deps.querier.query_wasm_smart::<()>(
            //     VERIFIER.load(deps.storage)?,
            //     &verifier::msg::QueryMsg::AccountCreationProof {
            //         relayer_hash: relayer_rand_hash.unwrap(),
            //         email_addr_pointer,
            //         account_key_commit,
            //         wallet_salt,
            //         psi_point,
            //         proof,
            //     },
            // )?;

            ACCOUNT_KEY_COMMIT_OF_POINTER.save(
                deps.storage,
                email_addr_pointer.to_string(),
                &account_key_commit,
            )?;
            INFO_OF_ACCOUNT_KEY_COMMIT.save(
                deps.storage,
                account_key_commit.to_string(),
                &AccountKeyInfo {
                    relayer: info.sender,
                    initialized: false,
                    wallet_salt,
                },
            )?;

            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::ExecuteMsg;
    use cosmwasm_std::{Addr, Empty, Uint256};
    use cw_multi_test::{App, ContractWrapper, Executor};
    use relayer_handler::msg::InstantiateMsg;
    use std::str::FromStr;
    use verifier::types::Groth16Proof;

    #[test]
    fn test_create_account() {
        let mut app = App::default();
        let sender = Addr::unchecked("xion1y9qlye0laqgfyjhsj8ldy354qcew3qu5q6d7m8");
        let verifier = deploy_verifier(&mut app, sender.clone());
        let relayer_handler = deploy_relayer_handler(&mut app, sender.clone());
        let account_handler = deploy_account_handler(
            &mut app,
            sender.clone(),
            &InstantiateMsg {
                relayer_handler_addr: relayer_handler.clone(),
                verifier_addr: verifier,
            },
        );

        app.execute_contract(
            sender.clone(),
            relayer_handler.clone(),
            &relayer_handler::msg::ExecuteMsg::RegisterRelayer {
                rand_hash: Uint256::from_str(
                    "11178924929211687158520013739298184339065861535351299825423363178288336162040",
                )
                .unwrap(),
                email_addr: "hduoc2003@gmail.com".to_string(),
                hostname: "mail.google.com".to_string(),
            },
            &[],
        )
        .unwrap();

        app.execute_contract(sender, account_handler, &ExecuteMsg::CreateAccount {
            email_addr_pointer: Uint256::from_str("16758458715147206766079366791832805159132081689916047111685962786065206865082").unwrap(),
            account_key_commit: Uint256::from_str("21358398833223768133805053213095102992867253664433975338853258016417424358279").unwrap(),
            wallet_salt: Uint256::from_str("16508469561050526225511527870209946093433618218663537432955398936429622658009").unwrap(),
            psi_point: [
                Uint256::from_str("3184649499805007138992381079366129329246398522610842004765495454419110487368").unwrap(),
                Uint256::from_str("19216343773775236196829009925907810475209443811108956591285097563906515509918").unwrap(),
            ],
            proof: Groth16Proof {
                pi_a: [
                    Uint256::from_str("18980354038611159583603829068438479209879835904444229764909853571028484752387").unwrap(),
                    Uint256::from_str("15134739764834943818652132530513553752654809256283071775502211421223978479166").unwrap(),
                ],
                pi_b: [[
                    Uint256::from_str("14720297409456662672211431842900665913622792038510075631351929778817177141017").unwrap(),
                    Uint256::from_str("5561654135414916708995413252501189144917289209519271840091715158759480610710").unwrap(),
                ], [
                    Uint256::from_str("21790568378115664822362576705093841138493863143904363327649186389465808020596").unwrap(),
                    Uint256::from_str("1164558372460354134231913263483748869592011710060278027531344627721108583791").unwrap(),
                ]],
                pi_c: [
                    Uint256::from_str("19909690285969584061778416482688787435441321429425628776705217812897768375483").unwrap(),
                    Uint256::from_str("15574526073423763165033690134317815624636912357678839590963060626713338606273").unwrap(),
                ],
            },
        }, &[]).unwrap();
    }

    pub fn deploy_account_handler(app: &mut App, sender: Addr, msg: &InstantiateMsg) -> Addr {
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        app.instantiate_contract(code_id, sender, msg, &[], "account handler", None)
            .unwrap()
    }

    pub fn deploy_relayer_handler(app: &mut App, sender: Addr) -> Addr {
        let code = ContractWrapper::new(
            relayer_handler::contract::execute,
            relayer_handler::contract::instantiate,
            relayer_handler::contract::query,
        );

        let code_id = app.store_code(Box::new(code));
        app.instantiate_contract(code_id, sender, &Empty {}, &[], "relayer handler", None)
            .unwrap()
    }

    pub fn deploy_verifier(app: &mut App, sender: Addr) -> Addr {
        let code = ContractWrapper::new(
            verifier::contract::execute,
            verifier::contract::instantiate,
            verifier::contract::query,
        );
        let code_id = app.store_code(Box::new(code));

        app.instantiate_contract(code_id, sender, &Empty {}, &[], "verifier", None)
            .unwrap()
    }
}
