use crate::handle::{try_bucket_range, try_bucket_read, try_bucket_write, try_bucket_write_range, try_item_fractioned_read, try_item_fractioned_write, try_item_read, try_item_write, try_map_range, try_map_read, try_map_write, try_map_write_range, try_singleton_read, try_singleton_write};
use crate::msgs::{HandleMsg, InitMsg, QueryMsg};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage,
};
use secret_toolkit::utils::{pad_handle_result, pad_query_result};

// Used to pad up responses for better privacy.
pub const RESPONSE_BLOCK_SIZE: usize = 256;

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse {
        messages: vec![],
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    pad_handle_result(
        match msg {
            HandleMsg::SingletonWrite { data } => try_singleton_write(deps, env, data),
            HandleMsg::SingletonRead {} => try_singleton_read(deps, env),
            HandleMsg::ItemWrite { data } => try_item_write(deps, env, data),
            HandleMsg::ItemRead {} => try_item_read(deps, env),
            HandleMsg::FractionedItemWrite { data } => try_item_fractioned_write(deps, env, data),
            HandleMsg::FractionedItemRead {} => try_item_fractioned_read(deps, env),

            HandleMsg::BucketWrite { id, data} => try_bucket_write(deps, env, id, data),
            HandleMsg::BucketWriteRange { id, data} => try_bucket_write_range(deps, env, id, data),
            HandleMsg::BucketRead { id } => try_bucket_read(deps, env, id),
            HandleMsg::BucketRange { id, min, max } => try_bucket_range(deps, env, id, min, max),
            HandleMsg::MapWrite { id, data} => try_map_write(deps, env, id, data),
            HandleMsg::MapWriteRange { id, data} => try_map_write_range(deps, env, id, data),
            HandleMsg::MapRead { id } => try_map_read(deps, env, id),
            HandleMsg::MapRange { id, min, max } => try_map_range(deps, env, id, min, max),
        },
        RESPONSE_BLOCK_SIZE,
    )
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    Ok(Binary::default())
}

