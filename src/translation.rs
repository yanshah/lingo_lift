#[allow(unused_imports)]
use reqwest::{Client, Response};

use serde_json::Value;

pub async fn translate_text(text: &str, source_lang: &str, target_lang: &str, api_key: &str) -> Result<String, reqwest::Error> {
    // Set up the HTTP client
    let client = Client::new();

    // Prepare the request URL
    let url = format!("https://api-free.deepl.com/v2/translate?auth_key={}&text={}&source_lang={}&target_lang={}",
                      api_key, text, source_lang, target_lang);

    // Make the POST request to DeepL API
    let response = client.post(&url).send().await?;

    // Parse the response JSON
    let response_json: Value = response.json().await?;
    // Extract translated text from the response
    let translated_text = response_json["translations"][0]["text"]
        .as_str()
        .unwrap_or("");

    Ok(translated_text.to_string())
}
