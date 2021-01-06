use async_std::prelude::*;
use std::error::Error;
use blake2::{VarBlake2b, digest::{Update, VariableOutput}};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use surf::{StatusCode, Url};
use std::fmt;


pub struct TransactionData {}
pub struct SerializedTransaction {}

#[derive(Debug, PartialEq)]
pub struct RpcApiInfo(Url);

impl Default for RpcApiInfo {
    fn default() -> Self {
        Self ("https://delphinet.smartpy.io/chains/main/blocks/head".parse().unwrap())
    }
}

impl AsRef<str> for RpcApiInfo {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for RpcApiInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ArgString {
    pub string: String
}

// correctly typing all possible variations
// of these would be a nightmare tbh
// for now, just what we need
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "prim", content = "args")]
pub enum BigmapEntry {
    Pair((ArgString, ArgString)),
}

pub async fn get_bigmap_entry<T: DeserializeOwned>(node: &RpcApiInfo, entry: &str, big_map_id: u32) -> Result<Option<T>, surf::Error> {
    match surf::get(format!("{}/context/big_maps/{}/{}",
                            node,
                            big_map_id,
                            get_script_expr(entry)
    )).await?.body_json().await {
        Ok(e) => Ok(Some(e)),
        Err(e) => match e.status() {
            // TODO this is what is returned by surf as a "generic" error, an actual 500 will
            // look like a "no package found" if it occurs
            StatusCode::InternalServerError => Ok(None),
            _ => Err(e)
        }
    }
}

/// Get Script Expression
///
/// Used for big map indexing
/// this impl is transcribed from go-tezos
/// https://github.com/goat-systems/go-tezos/blob/master/forge/forge.go#L1236
fn get_script_expr(index: &str) -> String {
    bs58::encode([
        // prefix "expr"
        [13u8, 44u8, 64u8, 27u8].as_ref(),
        &{
            let mut dig = VarBlake2b::new(32).unwrap();
            dig.update([
                // type confirmation bytes?
                [0x05, 0x01].as_ref(),
                // length as bytes, minimum width (should never panic as long as format! does correct hex)
                &hex::decode(format!("{:x}", index.len())).unwrap(),
                // value bytes
                index.as_bytes()
            ].concat());
            dig.finalize_boxed()
        }
    ].concat())
        .with_check()
        .into_string()
}

#[async_std::test]
async fn basic_bigmap_test() -> Result<(), Box<dyn Error>> {
    use crate::registry::*;
    const TEST_PACKAGE_HASH: &str = "exprtgUM4vqybphPru7cgDqJSjJrWd6tYpDwgivFjn1vKVpARJv1ZN";
    const TEST_PACKAGE: &str = "test_package_name";

    assert_eq!(TEST_PACKAGE_HASH, get_script_expr(TEST_PACKAGE));

    let entry: Option<BigmapEntry> = get_bigmap_entry(&RpcApiInfo::default(), TEST_PACKAGE, BIGMAP_ID).await?;

    assert_eq!(
        Some(BigmapEntry::Pair((
            ArgString { string: "tz1hd85kqYEcuG9JyVMYE3EGBukVEJqXAfNr".into() },
            ArgString { string: "test_package_path".into() }
        ))),
        entry
    );

    Ok(())
}

#[test]
fn script_expr() {
    // taken from the tezos Go implementation tests
    assert_eq!(
        "expruGmscHLuUazE7d79EepWCnDuPJreo8R87wsDGUgKAuH4E5ayEj",
        get_script_expr("Tezos Tacos Nachos")
    )
}
