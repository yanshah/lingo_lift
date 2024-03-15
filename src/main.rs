use std::env;
use cli_clipboard;
mod text_processing; // Import the text_processing module
mod translation; // Import the text_processing module



#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <text to translate> <source language> <target language>", args[0]);
        return;
    }

    let text_to_translate = &args[1];
    let source_language = &args[2];
    let target_language = &args[3];

    // Validate the text
    if !text_processing::validate_text(text_to_translate) {
        println!("The text to translate is invalid or empty!");
        return;
    }

    let preprocess_text = text_processing::preprocess_text(text_to_translate);

    println!("Text to translate: {}", preprocess_text);
    println!("Source language: {}", source_language);
    println!("Target language: {}", target_language);

    // Call the translate function

    // Retrieve OpenAI API key from environment variable
    let api_key = match env::var("DEEPL_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error:  DeepL API_KEY key not found in environment variable DEEPL_API_KEY");
            return;
        }
    };

    // Call translation logic

    let translated_text = translation::translate_text(&preprocess_text, source_language, target_language, &api_key).await;
    match translated_text {
        Ok(translated_text) => {
            println!("Translated text: {}", translated_text);


            let the_string = translated_text;
            cli_clipboard::set_contents(the_string.to_owned()).unwrap();
            assert_eq!(cli_clipboard::get_contents().unwrap(), the_string);


        }
        Err(err) => {
            println!("Error translating text: {}", err);
        }
    }
}
