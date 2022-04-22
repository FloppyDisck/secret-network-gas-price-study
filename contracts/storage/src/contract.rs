use cosmwasm_std::{Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage, to_binary};
use secret_toolkit::utils::{pad_handle_result, pad_query_result};
use crate::handle::{try_item_read, try_item_write, try_singleton_read, try_singleton_write};
use crate::msgs::{HandleMsg, InitMsg, QueryMsg};
use crate::query;

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
            HandleMsg::SingletonRead { data } => try_singleton_read(deps, env, data),
            HandleMsg::ItemWrite { data } => try_item_write(deps, env, data),
            HandleMsg::ItemRead { data } => try_item_read(deps, env, data),
        },
        RESPONSE_BLOCK_SIZE
    )
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    Ok(Binary::default())
}