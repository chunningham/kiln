use crate::tezos::*;
use crate::package::*;
use crate::error::Error;

pub async fn write_package_data(package: Package, chain: Option<ChainData>, node: Option<Node>) -> Future<Result<(), Error>> {
    let register_tx = get_package_update_tx(package, chain, node).await?;
    broadcast_transaction(register_tx, node)
}

pub async fn get_package_update_tx(package: Package, chain: Option<ChainData>, node: Option<Node>) -> Future<Result<SerializedTransaction, Error>> {
    serialize_transaction(get_package_update_tx_data(package, chain), node)
}

pub fn get_package_update_tx_data(package: Package, chain: Option<ChainData>) -> TransactionData {
    todo!()
}
