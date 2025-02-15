
use std::io::Seek;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to write")]
    IO(#[from] std::io::Error),
    #[error("Failed to download")]
    Download(#[from] reqwest::Error),
    #[error("Failed to get file from server")]
    BadStatus(reqwest::StatusCode),
}

pub fn download_and_open_file(url: &str, path: &std::path::Path) -> Result<std::fs::File, Error> {
    if std::fs::exists(&path).unwrap_or(false) {
        // File already exists, no need to download.
        let fd = std::fs::File::open(&path)?;
        return Ok(fd);
    }

    if let Some(dir) = path.parent() {
        // Create the dir that the file is downloaded in.
        if !std::fs::exists(&dir).unwrap_or(false) {
            std::fs::create_dir_all(&dir)?;
        }
    }

    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(Error::BadStatus(response.status()));
    }

    let body = response.text()?;

    let mut fd = std::fs::File::create_new(&path)?;
    std::io::copy(&mut body.as_bytes(), &mut fd)?;

    // Go to the first byte of the file, so that we can start reading from it.
    fd.seek(std::io::SeekFrom::Start(0))?;
    return Ok(fd);
}

