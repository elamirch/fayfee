use serde_json;
use ureq;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    pub articles: Vec<Article>,
}
  
#[derive(Deserialize, Debug)]
pub struct Article {
    pub source: Source,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    #[serde(rename = "urlToImage")]
    pub url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub content: Option<String>,
}
  
#[derive(Deserialize, Debug)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

pub fn get_articles(url: &str) -> Result<Response , Box<dyn Error>> {
    let response_raw = ureq::get(url).call()?.into_string()?;
    let response: Response = serde_json::from_str(&response_raw)?;
    Ok(response)
}
