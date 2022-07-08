use std::error::Error;
use colour::{ yellow , green };
use newsapi::{Response, get_articles};
use dotenv::dotenv;
use eframe::*;

fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();

    let url = format!("{}{}", "https://content.guardianapis.com/search?api-key=", std::env::var("API_KEY")?);
    let articles = get_articles(&url)?;
    
    render_articles(&articles);
    Ok(())
}

fn render_articles(articles: &Response) {
    for i in &articles.response.results {
        yellow!(" > {}\n", i.webTitle);
        green!(" . {}\n\n", i.webUrl);

    }
}

