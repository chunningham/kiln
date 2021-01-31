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
pub struct Model007PsDelph1BlockHeaderAlphaSignedContents {
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "proof_of_work_nonce")]
    pub proof_of_work_nonce: String,
    #[serde(rename = "seed_nonce_hash", skip_serializing_if = "Option::is_none")]
    pub seed_nonce_hash: Option<crate::models::CycleNonce>,
    #[serde(rename = "signature")]
    pub signature: crate::models::Signature,
}

impl Model007PsDelph1BlockHeaderAlphaSignedContents {
    pub fn new(priority: i32, proof_of_work_nonce: String, signature: crate::models::Signature) -> Model007PsDelph1BlockHeaderAlphaSignedContents {
        Model007PsDelph1BlockHeaderAlphaSignedContents {
            priority,
            proof_of_work_nonce,
            seed_nonce_hash: None,
            signature,
        }
    }
}


