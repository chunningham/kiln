use async_std::{prelude::*, fs::File, path::Path, io::{BufWriter, BufReader, copy}};
use std::error::Error;

// inspired by rust-wget
pub async fn download<P: AsRef<Path>>(url: &str, file_path: P) -> Result<(), Box<dyn Error>> {
    let mut res = surf::get(url).await?;

    // let buflen: usize = match res.header("Content-Length") {
    //     Some(len_header) => len_header.parse().map_error(|_| surf::Error::from_str(res.status(), "No Content Length"))?;
    //     None => 1024
    // };

    let mut file_writer = BufWriter::new(File::create(file_path).await?);
    let mut body_reader = BufReader::new(res.take_body().into_reader());

    copy(&mut body_reader, &mut file_writer).await?;

    Ok(())
}

#[async_std::test]
async fn dumb_fetch_test() -> Result<(), Box<dyn Error>> {
    const TEST_URL: &str = "https://upload.wikimedia.org/wikipedia/en/9/95/Test_image.jpg";
    const TEST_PATH: &str = "./test_image.jpg";

    download(TEST_URL, TEST_PATH).await
}
