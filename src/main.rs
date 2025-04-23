use colour::{green, yellow};
use newsapi::{get_articles, json_structs::Response};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let response = get_articles("Apple", "2025-04-20", "popularity")?;
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