use crate::fetch::download;
use std::error::Error;
use crate::registry::read::read_package_path;
use anyhow::{Result, bail, anyhow};
use async_std::path::Path;

async fn download_package<P: AsRef<Path>>(package_name: &str, package_dir: P) -> Result<()> {
    Ok(download(&match read_package_path(package_name).await.map_err(|e| anyhow!(e))? {
        Some(path) => path,
        None => bail!("No Such Package: {}", package_name)
            // TODO make errors a bit nicer
    }, package_dir.as_ref().join(package_name)).await.map_err(|_| anyhow!("Download Failed"))?)
}
