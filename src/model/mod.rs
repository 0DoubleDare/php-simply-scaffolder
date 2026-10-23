pub mod structs;
use bytes::Bytes;
use std::{error::Error, path::PathBuf};

use futures_util::{Stream, StreamExt, io::Cursor};
use reqwest::Url;
use zip::unstable::stream::ZipStreamReader;

pub async fn fetch_archive(
    target_dir: &PathBuf,
    url: &Url,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!("Скачивание архива");
    let response = reqwest::get(url.clone()).await?.error_for_status()?;

    let content = response.bytes().await?;
    let temp_file_path = target_dir.join(".temp_artifact_archive.zip");
    std::fs::write(&temp_file_path, content)?;

    Ok(temp_file_path)
}

pub async fn extract_archive(
    path_arhive: &PathBuf,
    target_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let archive = std::fs::File::open(path_arhive)?;

    let mut archive_reader = zip::ZipArchive::new(archive)?;

    for i in 0..archive_reader.len() {
        let mut file_in_archive = archive_reader.by_index(i)?;

        let outpath = match file_in_archive.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file_in_archive.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&outpath)?;
                }
            }

            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file_in_archive, &mut outfile)?;
        }
    }

    std::fs::remove_file(&path_arhive)?;

    Ok(())
}
