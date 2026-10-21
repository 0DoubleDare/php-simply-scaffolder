use std::error::Error;

use reqwest::Url;

pub async fn get_arhive_from_url(url: Url) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url)
        .await?
        .error_for_status()?;
    let content = response.bytes().await?;

    

    Ok(())
}