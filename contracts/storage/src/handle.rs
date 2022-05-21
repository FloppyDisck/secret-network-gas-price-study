use crate::msgs::{HandleAnswer, Config, UpdateConfig};
use crate::state::{ItemStorage, MapStorage};
use cosmwasm_std::{to_binary, Api, Env, Extern, HandleResponse, Querier, StdResult, Storage};
use shade_protocol::utils::generic_response::ResponseStatus;
use shade_protocol::utils::storage::{BucketStorage, SingletonStorage};
use secret_storage_plus::{Bound};

pub fn try_singleton_write<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    data: Config,
) -> StdResult<HandleResponse> {
    <Config as SingletonStorage>::save(&data, &mut deps.storage)?;

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
    <Config as SingletonStorage>::load(&deps.storage)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_item_write<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    data: Config,
) -> StdResult<HandleResponse> {
    <Config as ItemStorage>::save(&data, &mut deps.storage)?;

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
    <Config as ItemStorage>::load(&deps.storage)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_item_fractioned_write<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    data: UpdateConfig,
) -> StdResult<HandleResponse> {
    Config::save_some(data, &mut deps.storage)?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_item_fractioned_read<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    Config::load_fractioned(&deps.storage)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

fn make_key(id: (u64, u64)) -> Vec<u8> {
    let key = id.0.to_string() + "-" + &id.1.to_string();
    key.as_bytes().to_vec()
}

pub fn try_bucket_write<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: (u64, u64),
    data: Config
) -> StdResult<HandleResponse> {
    <Config as BucketStorage>::save(&data, &mut deps.storage, &make_key(id))?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_bucket_write_range<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: (u64, u64),
    data: Vec<Config>
) -> StdResult<HandleResponse> {
    let mut key = id;
    for data in data.iter() {
        <Config as BucketStorage>::save(&data, &mut deps.storage, &make_key(key))?;
        key.1 += 1;
    }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_bucket_read<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: (u64, u64),
) -> StdResult<HandleResponse> {
    <Config as BucketStorage>::load(&deps.storage, &make_key(id))?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_bucket_range<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: u64,
    min: u64,
    max: u64
) -> StdResult<HandleResponse> {
    for i in min..=max {
        <Config as BucketStorage>::load(&deps.storage, &make_key((id, i)))?;
    }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_map_write<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: (u64, u64),
    data: Config
) -> StdResult<HandleResponse> {
    <Config as MapStorage<(u64, u64)>>::save(&data, &mut deps.storage, id)?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_map_write_range<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: (u64, u64),
    data: Vec<Config>
) -> StdResult<HandleResponse> {
    let mut key = id;
    for data in data.iter() {
        <Config as MapStorage<(u64, u64)>>::save(&data, &mut deps.storage, key)?;
        key.1 += 1;
    }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_map_read<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: (u64, u64),
) -> StdResult<HandleResponse> {
    <Config as MapStorage<(u64, u64)>>::load(&deps.storage, id)?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}

pub fn try_map_range<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: u64,
    min: u64,
    max: u64
) -> StdResult<HandleResponse> {
    // let all: StdResult<Vec<_>> = <Config as MapStorage<(u64, u64)>>::prefix(id).range(
    //     &deps.storage,
    //     Some(Bound::inclusive(min)),
    //     Some(Bound::inclusive(max)),
    //     Order::Ascending
    // ).collect();
    for i in min..=max {
        <Config as MapStorage<(u64, u64)>>::load(&deps.storage, (id, i))?;
    }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Answer {})?),
    })
}