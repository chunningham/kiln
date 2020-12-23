use crate::error::Error;

pub struct TransactionData {}
pub struct SerializedTransaction {}

pub struct ChainData {}
pub struct Node {}

pub impl Default for ChainData {
    fn default() -> ChainData {
        ChainData {}
    }
}

pub async fn serialize_transaction(tx: TransactionData, chain: Option<ChainData>, node: Option<Node>) -> Future<Result<SeralizedTransaction, Error>> {
    todo!()
}

pub async fn broadcast_transaction(tx: SerializedTransaction, node: Option<Node>) ->Future<Result<(), Error>> {
    todo!()
}
