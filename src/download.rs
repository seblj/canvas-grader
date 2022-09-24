use std::{io::Write, path::Path};

pub async fn download_file<P: AsRef<Path>>(url: &str, fname: &str, dir: P) -> Result<(), anyhow::Error> {
    let response = reqwest::get(url).await?;
    let mut dest = std::fs::File::create(dir.as_ref().join(fname))?;
    dest.write_all(&response.bytes().await?)?;
    Ok(())
}
