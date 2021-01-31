/*
 * Tezos RPC
 *
 * Tezos client RPC API.
 *
 * The version of the OpenAPI document: 7.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotRunning {
    #[serde(rename = "status")]
    pub status: Status,
}

impl NotRunning {
    pub fn new(status: Status) -> NotRunning {
        NotRunning {
            status,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "not_running")]
    NotRunning,
}

