use crate::msgs::{HandleAnswer, SimpleConfig};
use crate::state::ItemStorage;
use cosmwasm_std::{to_binary, Api, Env, Extern, HandleResponse, Querier, StdResult, Storage};
use shade_protocol::utils::generic_response::ResponseStatus;
use shade_protocol::utils::storage::SingletonStorage;

pub fn try_singleton_write<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    data: SimpleConfig,
) -> StdResult<HandleResponse> {
    <SimpleConfig as SingletonStorage>::save(&data, &mut deps.storage)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_singleton_read<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    <SimpleConfig as SingletonStorage>::load(&deps.storage)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_item_write<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    data: SimpleConfig,
) -> StdResult<HandleResponse> {
    <SimpleConfig as ItemStorage>::save(&data, &mut deps.storage)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_item_read<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    <SimpleConfig as ItemStorage>::load(&deps.storage)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

