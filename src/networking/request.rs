use reqwest::Url;
use std::error::Error;

pub async fn url_request(link: &str) -> Result<String, Box<dyn Error + Send + Sync>> {

    let url = Url::parse(link)?;
    let response = reqwest::get(url).await?;
    return Ok(response.text().await?);
}