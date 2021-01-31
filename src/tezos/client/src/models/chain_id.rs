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
pub struct ChainId {
    #[serde(rename = "invalid_utf8_string")]
    pub invalid_utf8_string: Vec<i32>,
}

impl ChainId {
    pub fn new(invalid_utf8_string: Vec<i32>) -> ChainId {
        ChainId {
            invalid_utf8_string,
        }
    }
}


