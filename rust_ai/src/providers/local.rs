use crate::Model;

pub struct Local;

impl Local {
    pub fn new() -> Self {
        Local
    }
}

impl Model for Local {
    fn generate(&self, _prompt: &str) -> String {
        "This is a response from a local model.".to_string()
    }

    fn chat(&self, _history: &[String]) -> String {
        "This is a chat response from a local model.".to_string()
    }
}
