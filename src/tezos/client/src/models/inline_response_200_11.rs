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
pub struct InlineResponse20011 {
    #[serde(rename = "first")]
    pub first: i32,
    #[serde(rename = "last")]
    pub last: i32,
}

impl InlineResponse20011 {
    pub fn new(first: i32, last: i32) -> InlineResponse20011 {
        InlineResponse20011 {
            first,
            last,
        }
    }
}


