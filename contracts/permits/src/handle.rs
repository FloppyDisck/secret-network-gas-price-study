use cosmwasm_std::{Api, Env, Extern, HandleResponse, Querier, StdResult, Storage, to_binary};
use shade_protocol::utils::generic_response::ResponseStatus;
use crate::msgs::{HandleAnswer, QueryPermit};
use crate::state::{BannedPermitKey, store_password, validate_permit, validate_password};

pub fn try_set_viewing_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    key: String,
) -> StdResult<HandleResponse> {

    store_password(&mut deps.storage, &key, env.message.sender.to_string())?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::SetViewingKey {
            status: ResponseStatus::Success,
        })?),
    })
}

pub fn try_use_viewing_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    key: String,
) -> StdResult<HandleResponse> {

    validate_password(&deps.storage, &key, env.message.sender.to_string())?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::UseViewingKey {
            status: ResponseStatus::Success,
        })?),
    })
}

pub fn try_block_permit_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    key: String,
) -> StdResult<HandleResponse> {

    BannedPermitKey::ban(&mut deps.storage, key, env.message.sender.to_string())?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::BlockPermitKey {
            status: ResponseStatus::Success,
        })?),
    })
}

pub fn try_use_permit<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    permit: QueryPermit
) -> StdResult<HandleResponse> {

    validate_permit(&deps.storage, &deps.api, permit)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::UsePermit {
            status: ResponseStatus::Success,
        })?),
    })
}