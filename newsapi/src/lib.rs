use serde_json;
use ureq;
use std::error::Error;
use dotenv::dotenv;

pub mod json_structs;

pub fn get_articles(
            from: &str, sort_by: &str,
            language: &str, category: &str
        ) -> Result<json_structs::Response , Box<dyn Error>> {

    //Load .env
    dotenv().ok();
    
    //Craft the url
    let url = format!(
        "{}?from={}&sortBy={}&apiKey={}&language={}&category={}",
        std::env::var("NEWSAPI_ENDPOINT")?,
        from,
        sort_by,
        std::env::var("NEWSAPI_KEY")?,
        language,
        category
    );

    //Get response
    let response_raw = ureq::get(&url).call()?.into_string()?;

    //Decode response
    let response: json_structs::Response = serde_json::from_str(&response_raw)?;

    //Return response
    Ok(response)
}
