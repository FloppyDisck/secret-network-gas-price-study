use cosmwasm_std::{Api, Extern, Querier, StdResult, Storage};
use shade_protocol::utils::generic_response::ResponseStatus;
use crate::msgs::{QueryAnswer, QueryPermit};
use crate::state::{validate_permit, validate_password};
