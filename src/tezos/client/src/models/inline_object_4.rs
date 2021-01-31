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
pub struct InlineObject4 {
    #[serde(rename = "protocol_data")]
    pub protocol_data: crate::models::HelpersPreapplyBlockProtocolData,
    #[serde(rename = "operations")]
    pub operations: Vec<Vec<crate::models::NextOperation>>,
}

impl InlineObject4 {
    pub fn new(protocol_data: crate::models::HelpersPreapplyBlockProtocolData, operations: Vec<Vec<crate::models::NextOperation>>) -> InlineObject4 {
        InlineObject4 {
            protocol_data,
            operations,
        }
    }
}


