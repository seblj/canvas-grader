use std::{io::Write, path::Path};

pub async fn download_file(url: &str, fname: &str, dir: &Path) -> Result<(), anyhow::Error> {
    let response = reqwest::get(url).await?;
    let mut dest = std::fs::File::create(dir.join(fname))?;
    dest.write_all(&response.bytes().await?)?;
    Ok(())
}
