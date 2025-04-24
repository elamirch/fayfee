pub mod json_structs;
use dotenv::dotenv;
use ureq;

pub fn tg_message(
    text: impl Into<String>
) -> Result<json_structs::SendMessageResponse, Box<dyn std::error::Error>> {
    
    dotenv().ok();
    let token   = std::env::var("TELEGRAM_API")?;
    let chat_id   = std::env::var("CHANNEL_ID")?;

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    let body = serde_json::json!({
        "chat_id": chat_id,
        "text": text.into(),
        "parse_mode": "HTML",
    });

    let resp = ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_string(&body.to_string())?;

    let text = resp.into_string()?;
    let result = serde_json::from_str(&text)?;
    Ok(result)
}