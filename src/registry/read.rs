use crate::tezos::*;
use crate::package::*;
use crate::error::Error;

pub async fn read_package_data(name: String, chain: Option<ChainData>) -> Future<Result<Option<Package>, Error>> {
    todo!()
}
