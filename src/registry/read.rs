use crate::tezos::*;
use super::*;

pub async fn read_package_path(name: &str) -> Result<Option<String>, surf::Error> {
    Ok(get_bigmap_entry(name, CHAIN_ID, BIGMAP_ID).await?.map(|entry: BigmapEntry| {
        match entry {
            BigmapEntry::Pair((_, path)) => path.string
        }
    }))
}
