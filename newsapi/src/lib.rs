use serde_json;
use ureq;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub response: Results,
}
  
#[derive(Deserialize, Debug)]
pub struct Results {
    pub results: Vec<Article>,
}
  
#[derive(Deserialize, Debug)]
pub struct Article {
    pub webTitle: String,
    pub webUrl: String,
}



pub fn get_articles(url: &str) -> Result<Response , Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Response = serde_json::from_str(&response)?;
    Ok(articles)
}
