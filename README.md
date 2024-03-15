# Rust Translation CLI

This is a command-line translation tool written in Rust that utilizes the DeepL API to translate text between different languages.

## Prerequisites

Before running this application, you need to obtain an API key from DeepL. You can sign up for a DeepL API key [here](https://www.deepl.com/pro-account/) if you don't have one already.

Once you have your API key, export it as an environment variable named `DEEPL_API_KEY`:

## bash
export DEEPL_API_KEY=YOUR_DEEPL_API_KEY



## Usage
cargo run '<text_to_translate>' <source_language> <target_language> 
For example, to translate "Hello, world!" from English to German, you would run:
cargo run  'hello world' en de

## Output
The result is already copied to the clipboard, so you can paste it wherever you need it.




