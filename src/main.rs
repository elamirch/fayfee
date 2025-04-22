use colour::{green, yellow};
use dotenv::dotenv;
use newsapi::{get_articles, Response};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let url = format!(
        "{}?q={}&from={}&sortBy={}&apiKey={}",
        std::env::var("API_ENDPOINT")?,
        "Apple",
        "2025-04-20",
        "popularity",
        std::env::var("API_KEY")?
    );
    println!("{}", url);
    let response = get_articles(&url)?;
    render_articles(&response);
    Ok(())
}

fn render_articles(response: &Response) {
    for article in &response.articles {
        yellow!(" > {}\n", article.title);
        green!(" . {}\n\n", article.description
                                .as_ref().
                                unwrap_or(&"No description".to_string()));
    }
}