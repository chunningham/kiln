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
pub struct InlineObject10 {
    #[serde(rename = "script")]
    pub script: crate::models::Micheline007PsDelph1MichelsonV1Expression,
    #[serde(rename = "storage")]
    pub storage: crate::models::Micheline007PsDelph1MichelsonV1Expression,
    #[serde(rename = "input")]
    pub input: crate::models::Micheline007PsDelph1MichelsonV1Expression,
    /// Decimal representation of a positive big number
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "chain_id")]
    pub chain_id: crate::models::ChainId,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::Model007PsDelph1ContractId>,
    #[serde(rename = "payer", skip_serializing_if = "Option::is_none")]
    pub payer: Option<crate::models::Model007PsDelph1ContractId>,
    /// Decimal representation of a big number
    #[serde(rename = "gas", skip_serializing_if = "Option::is_none")]
    pub gas: Option<String>,
    #[serde(rename = "entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<crate::models::Unistring>,
}

impl InlineObject10 {
    pub fn new(script: crate::models::Micheline007PsDelph1MichelsonV1Expression, storage: crate::models::Micheline007PsDelph1MichelsonV1Expression, input: crate::models::Micheline007PsDelph1MichelsonV1Expression, amount: String, chain_id: crate::models::ChainId) -> InlineObject10 {
        InlineObject10 {
            script,
            storage,
            input,
            amount,
            chain_id,
            source: None,
            payer: None,
            gas: None,
            entrypoint: None,
        }
    }
}


