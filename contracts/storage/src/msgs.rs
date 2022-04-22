use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use secret_storage_plus::Item;
use secret_toolkit::utils::{HandleCallback, InitCallback, Query};
use serde::{Deserialize, Serialize};
use shade_protocol::utils::generic_response::ResponseStatus;
use shade_protocol::utils::storage::SingletonStorage;
use crate::state::ItemStorage;

pub type QueryPermit = Permit<PermitMsg>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

impl InitCallback for InitMsg {
    const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SimpleConfig {
    pub address: HumanAddr,
    pub some_number: Uint128,
    pub other_data: String
}

impl SingletonStorage for SimpleConfig {
    const NAMESPACE: &'static [u8] = b"simple_config";
}


impl<'a> ItemStorage for SimpleConfig {
    const ITEM: Item<'a, Self> = Item::new("item_simple_config");
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    // Simple config
    SingletonWrite {data: SimpleConfig},
    SingletonRead {},
    ItemWrite {data: SimpleConfig},
    ItemRead {},

    // Large config
    // LargeSingletonWrite {},
    // LargeSingletonRead {},
    // LargeItemWrite {},
    // LargeItemRead {},

    // Fractioned config
    // FractionedSingletonWrite {},
    // FractionedSingletonRead {},
    // FractionedItemWrite {},
    // FractionedItemRead {},
}

impl HandleCallback for HandleMsg {
    const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    Answer{}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {

}

impl Query for QueryMsg {
    const BLOCK_SIZE: usize = 256;
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {

}