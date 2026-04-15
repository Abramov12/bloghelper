
use std::fs;

#[derive(Debug, Deserialize)]
struct OllamaResopnse {
    response: String,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    prompt: String,
    model: String,
    theme:String
}

pub fn make_a_post(theme: &str) -> String{
    let prompt: String = std::fs::   
}