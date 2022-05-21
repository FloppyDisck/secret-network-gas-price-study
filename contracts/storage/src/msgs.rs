use crate::state::{ItemStorage, MapStorage};
use cosmwasm_std::{HumanAddr, StdResult, Storage, Uint128};
use schemars::JsonSchema;
use secret_storage_plus::{Item, Map};
use secret_toolkit::utils::{HandleCallback, InitCallback, Query};
use serde::{Deserialize, Serialize};
use shade_protocol::utils::storage::{BucketStorage, SingletonStorage};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

impl InitCallback for InitMsg {
    const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub address: HumanAddr,
    pub number: Uint128,
    pub other_data: String,
    pub array: Vec<u64>
}

const ADDR: Item<HumanAddr> = Item::new("human_addr");
const NUMBER: Item<Uint128> = Item::new("number");
const STRING: Item<String> = Item::new("string");
const ARR: Item<Vec<u64>> = Item::new("array");

impl Config {
    pub fn save_fractioned<S: Storage>(&self, storage: &mut S) -> StdResult<()> {
        ADDR.save(storage, &self.address)?;
        NUMBER.save(storage, &self.number)?;
        STRING.save(storage, &self.other_data)?;
        ARR.save(storage, &self.array)?;
        Ok(())
    }

    pub fn save_some<S: Storage>(config: UpdateConfig, storage: &mut S) -> StdResult<()> {
        if let Some(address) = config.address {
            ADDR.save(storage, &address)?;
        }
        if let Some(number) = config.number {
            NUMBER.save(storage, &number)?;
        }

        if let Some(other_data) = config.other_data.clone() {
            STRING.save(storage, &other_data)?;
        }

        if let Some(array) = config.array {
            ARR.save(storage, &array)?;
        }
        Ok(())
    }

    pub fn load_fractioned<S: Storage>(storage: & S) -> StdResult<Self> {
        Ok(Self {
            address: Self::address(storage)?,
            number: Self::number(storage)?,
            other_data: Self::other_data(storage)?,
            array: Self::array(storage)?
        })
    }

    pub fn address<S: Storage>(storage: & S) -> StdResult<HumanAddr> {
        ADDR.load(storage)
    }
    pub fn number<S: Storage>(storage: & S) -> StdResult<Uint128> {
        NUMBER.load(storage)
    }
    pub fn other_data<S: Storage>(storage: & S) -> StdResult<String> {
        STRING.load(storage)
    }
    pub fn array<S: Storage>(storage: & S) -> StdResult<Vec<u64>> {
        ARR.load(storage)
    }
}

impl SingletonStorage for Config {
    const NAMESPACE: &'static [u8] = b"singleton_config";
}

impl ItemStorage for Config {
    const ITEM: Item<'static, Config> = Item::new("item_config");
}

impl BucketStorage for Config {
    const NAMESPACE: &'static [u8] = b"bucket_config";
}

impl MapStorage<'_, (u64, u64)> for Config {
    const MAP: Map<'static, (u64, u64), Self> = Map::new("map_config");
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UpdateConfig {
    pub address: Option<HumanAddr>,
    pub number: Option<Uint128>,
    pub other_data: Option<String>,
    pub array: Option<Vec<u64>>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    // Simple config
    SingletonWrite { data: Config },
    SingletonRead {},
    ItemWrite { data: Config },
    ItemRead {},

    FractionedItemWrite { data: UpdateConfig },
    FractionedItemRead {},

    // Bucket iteration comparation
    BucketWrite { id: (u64, u64), data: Config },
    BucketWriteRange { id: (u64, u64), data: Vec<Config> },
    BucketRead { id: (u64, u64) },
    BucketRange { id: u64, min: u64, max: u64 },
    MapWrite { id: (u64, u64), data: Config },
    MapWriteRange { id: (u64, u64), data: Vec<Config> },
    MapRead { id: (u64, u64) },
    MapRange { id: u64, min: u64, max: u64 },
}

impl HandleCallback for HandleMsg {
    const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    Answer {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

impl Query for QueryMsg {
    const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {}

