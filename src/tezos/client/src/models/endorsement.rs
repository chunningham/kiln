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
pub struct Endorsement {
    #[serde(rename = "kind")]
    pub kind: Kind,
    #[serde(rename = "level")]
    pub level: i32,
}

impl Endorsement {
    pub fn new(kind: Kind, level: i32) -> Endorsement {
        Endorsement {
            kind,
            level,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "endorsement")]
    Endorsement,
}

