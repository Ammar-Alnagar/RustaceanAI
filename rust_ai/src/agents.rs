use crate::{AI, Provider};

pub struct Maestro {
    ai: AI,
}

impl Maestro {
    pub fn new() -> Self {
        let provider = Provider::OpenAI;
        let ai = AI::new(provider);
        Maestro { ai }
    }

    pub fn delegate(&self, task: &str) -> String {
        self.ai.generate(task)
    }
}

pub struct Agent {
    role: String,
    ai: AI,
}

impl Agent {
    pub fn new(role: &str, provider: Provider) -> Self {
        let ai = AI::new(provider);
        Agent {
            role: role.to_string(),
            ai,
        }
    }

    pub fn execute(&self, task: &str) -> String {
        let prompt = format!("You are a {} agent. {}", self.role, task);
        self.ai.generate(&prompt)
    }
}

pub struct Team {
    agents: Vec<Agent>,
}

impl Team {
    pub fn new() -> Self {
        let mut agents = Vec::new();
        agents.push(Agent::new("Planner", Provider::OpenAI));
        agents.push(Agent::new("Executor", Provider::OpenAI));
        agents.push(Agent::new("Critic", Provider::OpenAI));
        Team { agents }
    }

    pub fn delegate(&self, task: &str) -> String {
        let mut conversation = Vec::new();
        conversation.push(format!("Task: {}", task));

        for agent in &self.agents {
            let response = agent.execute(&conversation.join("\n"));
            conversation.push(format!("{}: {}", agent.role, response));
        }

        conversation.join("\n")
    }
}
