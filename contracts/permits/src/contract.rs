use cosmwasm_std::{Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage, to_binary};
use secret_toolkit::utils::{pad_handle_result, pad_query_result};
use crate::handle::{try_block_permit_key, try_set_viewing_key, try_use_permit, try_use_viewing_key};
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
            HandleMsg::SetViewingKey { key, .. } => try_set_viewing_key(deps, env, key),
            HandleMsg::UseViewingKey { key, .. } => try_use_viewing_key(deps, env, key),
            HandleMsg::BlockPermitKey { key, .. } => try_block_permit_key(deps, env, key),
            HandleMsg::UsePermit { permit, .. } => try_use_permit(deps, env, permit),
        },
        RESPONSE_BLOCK_SIZE
    )
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    pad_query_result(
        match msg {
            QueryMsg::ViewingKey { address, key } => to_binary(&query::viewing_key(deps, address, key)?),
            QueryMsg::Permit { permit } => to_binary(&query::permit(deps, permit)?),
        },
        RESPONSE_BLOCK_SIZE
    )
}