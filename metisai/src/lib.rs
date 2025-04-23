use std::error::Error;
use dotenv::dotenv;

pub mod json_structs;

pub fn ai_message(message: &str) -> Result<String, Box<dyn Error>> {
    
    dotenv().ok();
    
    let session_id   = std::env::var("METIS_SESSION_ID")?;
    let api_key      = std::env::var("METIS_API_KEY")?;

    //Craft the URL
    let url = format!(
        "https://api.metisai.ir/api/v1/chat/session/{}/message",
        session_id
    );

    //Build the JSON payload
    let payload = format!(
        r#"{{ 
            "message": {{
                "content": "{}",
                "type": "USER"
            }}
        }}"#,
        message
    );

    //Send the POST
    let response = ureq::post(&url)
        .set("Authorization", &format!("Bearer {}", api_key))
        .set("Content-Type", "application/json")
        .send_string(&payload);

    

    let body = response?.into_string()?;

    println!("Hey{}\n", body);

    let provider_output: json_structs::ProviderOutput = serde_json::from_str(&body)?;

    // 2. Extract exactly what you need
    let output_message: String = provider_output.content;
    
    Ok(output_message)
}
