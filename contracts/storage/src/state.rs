use cosmwasm_std::{StdError, StdResult, Storage};
use secret_storage_plus::Item;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait ItemStorage: Serialize + DeserializeOwned {
    const ITEM: Item<'static, Self>;

    fn load<S: Storage>(storage: &S) -> StdResult<Self> {
        Self::ITEM.load(storage)
    }

    fn may_load<S: Storage>(storage: &S) -> StdResult<Option<Self>> {
        Self::ITEM.may_load(storage)
    }

    fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()> {
        Self::ITEM.save(storage, self)
    }

    fn update<A, E, S: Storage>(&self, storage: &mut S, action: A) -> Result<Self, E>
    where
        A: FnOnce(Self) -> Result<Self, E>,
        E: From<StdError>,
    {
        Self::ITEM.update(storage, action)
    }
}

