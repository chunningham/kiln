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
pub struct Reveal {
    #[serde(rename = "kind")]
    pub kind: Kind,
    #[serde(rename = "source")]
    pub source: crate::models::Model007PsDelph1ContractId,
    #[serde(rename = "nonce")]
    pub nonce: i32,
    #[serde(rename = "public_key")]
    pub public_key: crate::models::SignaturePublicKey,
    #[serde(rename = "result")]
    pub result: crate::models::Model007PsDelph1OperationAlphaOperationResultReveal,
}

impl Reveal {
    pub fn new(kind: Kind, source: crate::models::Model007PsDelph1ContractId, nonce: i32, public_key: crate::models::SignaturePublicKey, result: crate::models::Model007PsDelph1OperationAlphaOperationResultReveal) -> Reveal {
        Reveal {
            kind,
            source,
            nonce,
            public_key,
            result,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "reveal")]
    Reveal,
}

