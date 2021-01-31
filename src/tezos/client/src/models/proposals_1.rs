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
pub struct Proposals1 {
    #[serde(rename = "kind")]
    pub kind: Kind,
    #[serde(rename = "source")]
    pub source: crate::models::SignaturePublicKeyHash,
    #[serde(rename = "period")]
    pub period: i32,
    #[serde(rename = "proposals")]
    pub proposals: Vec<crate::models::ProtocolHash>,
    #[serde(rename = "metadata")]
    pub metadata: serde_json::Value,
}

impl Proposals1 {
    pub fn new(kind: Kind, source: crate::models::SignaturePublicKeyHash, period: i32, proposals: Vec<crate::models::ProtocolHash>, metadata: serde_json::Value) -> Proposals1 {
        Proposals1 {
            kind,
            source,
            period,
            proposals,
            metadata,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "proposals")]
    Proposals,
}

