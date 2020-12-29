use async_std::prelude::*;
use std::error::Error;

pub struct TransactionData {}
pub struct SerializedTransaction {}


const DEFAULT_NODE: &str = "https://delphinet.smartpy.io";
}

pub async fn serialize_transaction(tx: TransactionData, chain: Option<ChainData>, node: Option<Node>) -> Future<Result<SeralizedTransaction, Error>> {
    todo!()
}

pub async fn broadcast_transaction(tx: SerializedTransaction, node: Option<Node>) ->Future<Result<(), Error>> {
    todo!()
}
