use crate::Model;

pub struct OpenAI;

impl OpenAI {
    pub fn new() -> Self {
        OpenAI
    }
}

impl Model for OpenAI {
    fn generate(&self, _prompt: &str) -> String {
        "This is a response from OpenAI.".to_string()
    }
}
