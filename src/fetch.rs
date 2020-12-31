use async_std::{prelude::*, fs::File, path::Path, io::{BufWriter, BufReader, copy}};
use std::error::Error;
use anyhow::{Result, bail, anyhow};

// inspired by rust-wget
pub async fn download<P: AsRef<Path>>(url: &str, file_path: P) -> Result<()> {
    Ok(copy(
        &mut BufReader::new(surf::get(url).await.map_err(|e| anyhow!(e))?.take_body().into_reader()),
        &mut BufWriter::new(File::create(file_path).await?)
    ).await.map(|_| ())?)
}

#[async_std::test]
async fn dumb_fetch_test() -> Result<()> {
    const TEST_URL: &str = "https://upload.wikimedia.org/wikipedia/en/9/95/Test_image.jpg";
    const TEST_PATH: &str = "./test_image.jpg";

    download(TEST_URL, TEST_PATH).await
}
