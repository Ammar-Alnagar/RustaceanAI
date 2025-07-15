pub mod agents;
pub mod providers;
pub mod tui;
pub mod gui;
pub mod new_feature;

#[derive(Clone, PartialEq, Debug)]
pub enum Provider {
    OpenAI,
    Gemini,
    OpenRouter,
    HuggingFace,
    Local,
}

pub trait Model {
    fn generate(&self, prompt: &str) -> String;
    fn chat(&self, history: &[String]) -> String;
}

pub enum AIModel {
    OpenAI(providers::openai::OpenAI),
    Gemini(providers::gemini::Gemini),
    OpenRouter(providers::openrouter::OpenRouter),
    HuggingFace(providers::huggingface::HuggingFace),
    Local(providers::local::Local),
}

impl Model for AIModel {
    fn generate(&self, prompt: &str) -> String {
        match self {
            AIModel::OpenAI(model) => model.generate(prompt),
            AIModel::Gemini(model) => model.generate(prompt),
            AIModel::OpenRouter(model) => model.generate(prompt),
            AIModel::HuggingFace(model) => model.generate(prompt),
            AIModel::Local(model) => model.generate(prompt),
        }
    }

    fn chat(&self, history: &[String]) -> String {
        match self {
            AIModel::OpenAI(model) => model.chat(history),
            AIModel::Gemini(model) => model.chat(history),
            AIModel::OpenRouter(model) => model.chat(history),
            AIModel::HuggingFace(model) => model.chat(history),
            AIModel::Local(model) => model.chat(history),
        }
    }
}

pub struct AI {
    model: AIModel,
}

impl AI {
    pub fn new(provider: Provider) -> Self {
        let model = match provider {
            Provider::OpenAI => AIModel::OpenAI(providers::openai::OpenAI::new()),
            Provider::Gemini => AIModel::Gemini(providers::gemini::Gemini::new()),
            Provider::OpenRouter => AIModel::OpenRouter(providers::openrouter::OpenRouter::new()),
            Provider::HuggingFace => AIModel::HuggingFace(providers::huggingface::HuggingFace::new()),
            Provider::Local => AIModel::Local(providers::local::Local::new()),
        };
        AI { model }
    }

    pub fn generate(&self, prompt: &str) -> String {
        self.model.generate(prompt)
    }

    pub fn chat(&self, history: &[String]) -> String {
        self.model.chat(history)
    }
}
