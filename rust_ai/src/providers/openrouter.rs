use crate::Model;

pub struct OpenRouter;

impl OpenRouter {
    pub fn new() -> Self {
        OpenRouter
    }
}

impl Model for OpenRouter {
    fn generate(&self, _prompt: &str) -> String {
        "This is a response from OpenRouter.".to_string()
    }

    fn chat(&self, _history: &[String]) -> String {
        "This is a chat response from OpenRouter.".to_string()
    }
}
