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

pub async fn broadcast_transaction(tx: SerializedTransaction, node: Option<Node>) ->Future<Result<(), Error>> {
    todo!()
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
}
