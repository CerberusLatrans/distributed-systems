use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HelloMessage {
    r#type: String,
    northeastern_username: String,
}

impl HelloMessage {
    pub fn new(username: &str) -> Self {
        Self {
            r#type: String::from("hello"),
            northeastern_username: String::from(username),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GuessMessage {
    r#type: String,
    id: String,
    word: String,
}

impl GuessMessage {
    pub fn new(word: String, id: String) -> Self {
        GuessMessage {
            r#type: String::from("guess"),
            id: id,
            word: word,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct StartMessage {
    r#type: String,
    pub id: String,
}

//{"type": "retry", "id": <string>, "guesses": [{ "word": <string>, "marks": <array> }, { "word": <string>, "marks": <array> }, ... ]}\n
#[derive(Deserialize, Serialize)]
pub struct RetryMessage {
    r#type: String,
    id: String,
    pub guesses: Vec<Score>,
}

#[derive(Deserialize, Serialize)]
pub struct Score {
    pub word: String,
    pub marks: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct ByeMessage {
    r#type: String,
    id: String,
    pub flag: String,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorMessage {
    r#type: String,
    pub message: String,
}