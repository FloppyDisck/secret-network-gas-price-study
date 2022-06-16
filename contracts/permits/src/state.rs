use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::convert::TryInto;
use cosmwasm_std::{Api, StdResult, Storage};
use sha2::{Sha256, Digest};
use shade_protocol::utils::storage::default::BucketStorage;
use crate::errors::{incorrect_password, password_incorrect_length, password_not_set, permit_key_banned, permit_rejected};
use crate::msgs::QueryPermit;

pub const VIEWING_KEY_SIZE: usize = 32;

///
/// Hashes the given string
///
pub fn hash_password(password: &str) -> StdResult<[u8; VIEWING_KEY_SIZE]> {
    let sha_result = Sha256::digest(password.as_bytes()).as_slice().try_into();
    if sha_result.is_err() {
        return Err(password_incorrect_length(password.len() as u16))
    }
    Ok(sha_result.unwrap())
}

///
/// Checks if password is correct
///
pub fn validate_password<S: Storage>(storage: &S, password: &str, account: String) -> StdResult<()> {
    let pwd_hash = hash_password(password)?;
    let saved_hash = ViewingKeys::may_load(storage, account.as_bytes())?;
    match saved_hash {
        None => {
            Err(password_not_set())
        }
        Some(hash) => {
            return if pwd_hash.eq(&hash.0) {
                Ok(())
            } else {
                Err(incorrect_password())
            }
        }
    }
}

///
/// Stores the hashed password
///
pub fn store_password<S: Storage>(storage: &mut S, password: &str, account: String) -> StdResult<()> {
    let hash = hash_password(password)?;
    ViewingKeys(hash).save(storage, account.as_bytes())
}

///
/// Validate permit signature and if the permit's key has been blocked
///
pub fn validate_permit<S: Storage, A: Api>(storage: &S, api: &A, permit: QueryPermit) -> StdResult<()> {
    let account = match permit.validate(api, None) {
        Ok(pubkey) => pubkey.as_humanaddr(None)?.to_string(),
        Err(_) => return Err(permit_rejected())
    };

    if BannedPermitKey::is_banned(storage, permit.params.key, account)? {
        return Err(permit_key_banned())
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ViewingKeys(pub [u8; VIEWING_KEY_SIZE]);

impl BucketStorage for ViewingKeys {
    const NAMESPACE: &'static [u8] = b"viewing_key";
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BannedPermitKey(pub bool);

impl BucketStorage for BannedPermitKey {
    const NAMESPACE: &'static [u8] = b"banned_permit_key";
}

impl BannedPermitKey {
    pub fn is_banned<S: Storage>(storage: & S, key: String, account: String) -> StdResult<bool> {
        Ok(BannedPermitKey::may_load(storage, (account + &key).as_bytes())?.is_some())
    }

    pub fn ban<S: Storage>(storage: &mut S, key: String, account: String) -> StdResult<()> {
        BannedPermitKey(true).save(storage, (account + &key).as_bytes())?;
        Ok(())
    }
}