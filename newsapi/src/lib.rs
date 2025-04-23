use serde_json;
use ureq;
use std::error::Error;
use dotenv::dotenv;

pub mod json_structs;

pub fn get_articles(
            query: &str, from: &str,
            sort_by: &str
        ) -> Result<json_structs::Response , Box<dyn Error>> {

    //Load .env
    dotenv().ok();

    //Craft the url
    let url = format!(
        "{}?q={}&from={}&sortBy={}&apiKey={}",
        std::env::var("API_ENDPOINT")?,
        query,
        from,
        sort_by,
        std::env::var("API_KEY")?
    );
    
    //Get response
    let response_raw = ureq::get(&url).call()?.into_string()?;

    //Decode response
    let response: json_structs::Response = serde_json::from_str(&response_raw)?;

    //Return response
    Ok(response)
}
