// text_processing.rs

pub fn validate_text(text: &str) -> bool {
    !text.trim().is_empty()
}

pub fn preprocess_text(text: &str) -> String {
    text.trim().to_string()
}
