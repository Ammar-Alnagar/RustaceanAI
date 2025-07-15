use crate::Model;

pub struct Gemini;

impl Gemini {
    pub fn new() -> Self {
        Gemini
    }
}

impl Model for Gemini {
    fn generate(&self, _prompt: &str) -> String {
        "This is a response from Gemini.".to_string()
    }
}
