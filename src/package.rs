use crate::fetch::download;
use std::error::Error;
use crate::registry::read::read_package_path;
use anyhow::{Result, bail, anyhow};

async fn download_package(package_name: &str) -> Result<()> {
    Ok(download(&match read_package_path(package_name).await.map_err(|e| anyhow!(e))? {
        Some(path) => path,
        None => bail!("No Such Package: {}", package_name)
            // TODO make errors a bit nicer
    }, package_name).await.map_err(|_| anyhow!("Download Failed"))?)
}
