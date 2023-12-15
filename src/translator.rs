use serde_json::json;
use std::env;

#[path = "./language.rs"]
mod language;

#[derive(serde::Deserialize)]
pub struct Response {
    choices: Vec<Choice>,
}

#[derive(serde::Deserialize)]
pub struct Choice {
    message: ChoiceMessage,
}

#[derive(serde::Deserialize)]
pub struct ChoiceMessage {
    content: String,
}

pub fn translate(message: &String) -> String {
    let client = reqwest::blocking::Client::new();
    let request = format!("https://api.openai.com/v1/chat/completions");
    let model = "gpt-3.5-turbo";
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let message: Response = client
        .post(request)
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": format!("Translate the Japanese text to English: \"{}\". Send only result.", message),
                }
            ]
        }))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", api_key),
        )
        .send()
        .unwrap()
        .json()
        .unwrap();
    let out = &message
        .choices
        .iter()
        .map(|x| x.message.content.clone())
        .collect::<Vec<String>>()
        .join("\n");
    return out.to_string();
}
