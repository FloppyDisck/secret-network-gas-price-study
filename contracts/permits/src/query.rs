use cosmwasm_std::{Api, Extern, Querier, StdResult, Storage};
use shade_protocol::utils::generic_response::ResponseStatus;
use crate::msgs::{QueryAnswer, QueryPermit};
use crate::state::{validate_permit, validate_password};

pub fn viewing_key<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: String,
    key: String
) -> StdResult<QueryAnswer> {

    let status: ResponseStatus;

    if validate_password(&deps.storage, &key, address).is_err() {
        status = ResponseStatus::Failure
    }
    else {
        status = ResponseStatus::Success
    }

    Ok(QueryAnswer::ViewingKey {
        status
    })
}

pub fn permit<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    permit: QueryPermit
) -> StdResult<QueryAnswer> {

    let status: ResponseStatus;

    if validate_permit(&deps.storage, &deps.api, permit).is_err() {
        status = ResponseStatus::Failure
    }
    else {
        status = ResponseStatus::Success
    }

    Ok(QueryAnswer::Permit {
        status
    })
}