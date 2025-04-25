use newsapi::{get_articles};
// use metisai::{ai_message};
use telegramapi::{tg_message};
use chrono::Local;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let categories = ["technology", "health"];
    let date = Local::now().date_naive().to_string();

    for category in categories.iter() {
        let response = get_articles(&date, "relevancy", "en", category)?;
        
        for article in &response.articles {
            let description = article
                                    .description
                                    .as_deref().unwrap_or("");
            
            // //Refine description
            // let description = ai_message(description)?;

            //Build the message
            let message = format!(
                "<b>{}</b>\n\n{}\n\nSource: <a href=\"{}\">{}</a> | #{}",
                article.title,
                description,
                article.url,
                article.source.name,
                category
            );

            //Send the message to the Telegram channel
            let _ = tg_message(message);
        }
    }

    Ok(())
}