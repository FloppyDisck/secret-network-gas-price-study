use flexible_permits::permit::Permit;
use schemars::JsonSchema;
use secret_toolkit::utils::{HandleCallback, InitCallback, Query};
use serde::{Deserialize, Serialize};
use shade_protocol::utils::generic_response::ResponseStatus;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PermitMsg {
    pub key: String,
}

pub type QueryPermit = Permit<PermitMsg>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

impl InitCallback for InitMsg {
    const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    SetViewingKey {
        key: String,
        padding: Option<String>
    },
    UseViewingKey {
        key: String,
        padding: Option<String>
    },
    BlockPermitKey {
        key: String,
        padding: Option<String>
    },
    UsePermit {
        permit: QueryPermit,
        padding: Option<String>
    }
}

impl HandleCallback for HandleMsg {
    const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    SetViewingKey { status: ResponseStatus },
    UseViewingKey { status: ResponseStatus },
    BlockPermitKey { status: ResponseStatus },
    UsePermit { status: ResponseStatus }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ViewingKey { address: String, key: String },
    Permit { permit: QueryPermit }
}

impl Query for QueryMsg {
    const BLOCK_SIZE: usize = 256;
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    ViewingKey { status: ResponseStatus },
    Permit { status: ResponseStatus }
}