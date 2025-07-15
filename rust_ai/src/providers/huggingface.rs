use crate::Model;

pub struct HuggingFace;

impl HuggingFace {
    pub fn new() -> Self {
        HuggingFace
    }
}

impl Model for HuggingFace {
    fn generate(&self, _prompt: &str) -> String {
        "This is a response from HuggingFace.".to_string()
    }
}
