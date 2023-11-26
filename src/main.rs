use serde_json::Value;
use std::io;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("please enter your question:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let api_url = "https://api.cohere.ai/v1/chat";
    let api_key = "API_KEY"; // Replace with your actual API key
    let payload = format!(
        r#"
        {{
            "message": "{}",
            "connectors": [{{"id": "web-search"}}]

        }}
        "#,
        user_input.trim()
    );
    

    let response = reqwest::Client::new()
        .post(api_url)
        .header("accept", "application/json")
        .header("authorization", format!("Bearer {}", api_key))
        .header("content-type", "application/json")
        .body(payload)
        .send()
        .await?;

    let response_text = response.text().await?;
    let parsed: Value = serde_json::from_str(&response_text).unwrap();
    if let Some(text) = parsed.get("text") {
        if let Some(text_str) = text.as_str() {
            println!("{}", text_str);
        }
    }

    Ok(())
}