use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{error::Error, fmt};
use surf::{StatusCode, Url};

use super::api;
use super::forge::get_script_expr;

#[derive(Debug, PartialEq)]
pub struct RpcApiInfo(Url);

impl Default for RpcApiInfo {
    fn default() -> Self {
        Self(
            "https://delphinet.smartpy.io/chains/main/blocks/head"
                .parse()
                .unwrap(),
        )
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

pub async fn get_bigmap_entry<T: DeserializeOwned>(
    node: &RpcApiInfo,
    entry: &str,
    big_map_id: u32,
) -> Result<Option<T>, surf::Error> {
    match surf::get(format!(
        "{}/context/big_maps/{}/{}",
        node,
        big_map_id,
        get_script_expr(entry)
    ))
    .await?
    .body_json()
    .await
    {
        Ok(e) => Ok(Some(e)),
        Err(e) => match e.status() {
            // TODO this is what is returned by surf as a "generic" error, an actual 500 will
            // look like a "no package found" if it occurs
            StatusCode::InternalServerError => Ok(None),
            _ => Err(e),
        },
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ArgString {
    pub string: String,
}

// correctly typing all possible variations
// of these would be a nightmare tbh
// for now, just what we need
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "prim", content = "args")]
pub enum BigmapEntry {
    Pair((ArgString, ArgString)),
}

#[async_std::test]
async fn basic_bigmap_test() -> Result<(), Box<dyn Error>> {
    use crate::registry::*;
    const TEST_PACKAGE_HASH: &str = "exprtgUM4vqybphPru7cgDqJSjJrWd6tYpDwgivFjn1vKVpARJv1ZN";
    const TEST_PACKAGE: &str = "test_package_name";

    assert_eq!(TEST_PACKAGE_HASH, get_script_expr(TEST_PACKAGE));

    let entry: Option<BigmapEntry> =
        get_bigmap_entry(&RpcApiInfo::default(), TEST_PACKAGE, BIGMAP_ID).await?;

    assert_eq!(
        Some(BigmapEntry::Pair((
            ArgString {
                string: "tz1hd85kqYEcuG9JyVMYE3EGBukVEJqXAfNr".into()
            },
            ArgString {
                string: "test_package_path".into()
            }
        ))),
        entry
    );

    Ok(())
}
