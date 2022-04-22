use cosmwasm_std::{StdError, StdResult, Storage};
use secret_storage_plus::Item;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::msgs::SimpleConfig;

const SIMPLE_CONFIG: Item<SimpleConfig> = Item::new("item_simple_config");

pub trait ItemStorage<'a>: Serialize + DeserializeOwned {
    const ITEM: Item<'a, Self>;

    fn load<S: Storage>(storage: &S) -> StdResult<Self> {
        Self::ITEM.load(storage)
    }

    fn may_load<S: Storage>(storage: &S) -> StdResult<Option<Self>> {
        Self::ITEM.may_load(storage)
    }

    fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()> {
        Self::ITEM.save(storage, self)
    }

    fn update<A, E, S: Storage>(&self, storage: &mut S, action: A) -> Result<T, E>
    where
        A: FnOnce(T) -> Result<T, E>,
        E: From<StdError>,
    {
        Self::ITEM.update(storage, action)
    }
}