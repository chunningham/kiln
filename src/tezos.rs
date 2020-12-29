use async_std::prelude::*;
use std::error::Error;
use blake2::{VarBlake2b, digest::{Update, VariableOutput}};

pub struct TransactionData {}
pub struct SerializedTransaction {}


const DEFAULT_NODE: &str = "https://delphinet.smartpy.io";

async fn get_latest_block_id(chain_id: &str) -> Result<String, surf::Error> {
    let res_body = surf::get(format!("{}/chains/{}/blocks?length=1", DEFAULT_NODE, chain_id)).await?.take_body();
    let blocks: Vec<Vec<String>> = res_body.into_json().await?;
    Ok(blocks.first().unwrap().first().unwrap().to_owned())
}

async fn get_bigmap_entry<T>(entry: &str, chain_id: &str, big_map_id: u32) -> Result<T, surf::Error> {
    let block_id = get_latest_block_id(chain_id).await?;
    let res = surf::get(format!("{}/block_id/context/big_maps/{}", DEFAULT_NODE, big_map_id)).await?;


    todo!()
}

/// Get Script Expression
///
/// Used for big map indexing
/// frustratingly, seems totally undocumented
/// this impl is transcribed from go-tezos
/// https://github.com/goat-systems/go-tezos/blob/519084001470cc6fb17c2d79010bec47da4489ff/forge/forge.go#L1236
fn get_script_expr(index: &str) -> String {
    bs58::encode([
        // prefix "expr"
        [13u8, 44u8, 64u8, 27u8].as_ref(),
        &{
            let mut dig = VarBlake2b::new(32).unwrap();
            dig.update([
                // type confirmation bytes?
                [5u8, 1u8].as_ref(),
                // length as bytes
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
async fn basic_block_id_test() -> Result<(), Box<dyn Error>>{
    use crate::registry::CHAIN_ID;

    let _block_id = get_latest_block_id(CHAIN_ID).await?;
    Ok(())
}

#[async_std::test]
async fn basic_bigmap_test() -> Result<(), Box<dyn Error>> {
    use crate::registry::*;
    const TEST_PACKAGE: &str = "test_package_name";

    let mut res = surf::get(format!("{}/chains/{}/blocks/head/context/big_maps/{}/{}", DEFAULT_NODE, CHAIN_ID, BIGMAP_ID, get_script_expr(TEST_PACKAGE))).await?;

    println!("{}", res.body_string().await?);

    assert!(false);
    Ok(())
}

#[test]
fn script_expr() {
    // taken from the tezos Go implementation
    let index = "Tezos Tacos Nachos";
    println!("{:?}", [
                // type confirmation bytes?
                [5u8, 1u8].as_ref(),
                // length num in bytes
                &hex::decode(format!("{:x}", index.len())).unwrap(),
                // value bytes
                index.as_bytes()
            ].concat());
    assert_eq!(
        "expruGmscHLuUazE7d79EepWCnDuPJreo8R87wsDGUgKAuH4E5ayEj",
        get_script_expr("Tezos Tacos Nachos")
    )
}
