use cosmwasm_std::StdError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shade_protocol::impl_into_u8;
use shade_protocol::utils::errors::{build_string, CodeType, DetailedError};
use crate::state::VIEWING_KEY_SIZE;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug, JsonSchema)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    IncorrectPasswordLength,
    IncorrectPassword,
    PasswordNotSet,
    PermitRejected,
    PermitKeyBanned,
}

impl_into_u8!(Error);

impl CodeType for Error {
    fn to_verbose(&self, context: &Vec<&str>) -> String {
        match self {
            Error::IncorrectPasswordLength => build_string(
                "Password length must be {}, got {}", context),
            Error::IncorrectPassword => build_string("Password is incorrect", context),
            Error::PasswordNotSet => build_string("Password has not been set", context),
            Error::PermitRejected => build_string("Permit signature was not valid", context),
            Error::PermitKeyBanned => build_string("Permit key is banned", context)
        }
    }
}

const TARGET: &str = "permits";

pub fn password_incorrect_length(password_len: u16) -> StdError {
    DetailedError::from_code(
        TARGET,
        Error::IncorrectPasswordLength,
        vec![&password_len.to_string(), &VIEWING_KEY_SIZE.to_string()])
        .to_error()
}

pub fn incorrect_password() -> StdError {
    DetailedError::from_code(
        TARGET,
        Error::IncorrectPassword,
        vec![])
        .to_error()
}

pub fn password_not_set() -> StdError {
    DetailedError::from_code(
        TARGET,
        Error::PasswordNotSet,
        vec![])
        .to_error()
}

pub fn permit_rejected() -> StdError {
    DetailedError::from_code(
        TARGET,
        Error::PermitRejected,
        vec![])
        .to_error()
}

pub fn permit_key_banned() -> StdError {
    DetailedError::from_code(
        TARGET,
        Error::PermitKeyBanned,
        vec![])
        .to_error()
}