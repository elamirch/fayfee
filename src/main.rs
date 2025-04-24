use newsapi::{get_articles, json_structs::Response};
use metisai::{ai_message};
use telegramapi::{tg_message};
use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = get_articles("Iran", "2025-04-23", "popularity")?;
    
    //Check if the title is appropriate for the channel
    let status = ai_message(article.title);

    for article in &response.articles {
        if(status != "No") {
            let message = format!(
                "{}\n\n{}\nSource: <a href=\"{}\">{}</a>",
                article.title,
                article.description.as_ref().unwrap_or(&"".to_string()),
                article.url,
                article.source.name
            );
        }
    }

    Ok(())
}