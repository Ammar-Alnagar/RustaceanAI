use eframe::egui;

#[derive(Default)]
pub struct RustAIApp {
    prompt: String,
    provider: String,
    output: String,
}

impl eframe::App for RustAIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust AI");

            ui.horizontal(|ui| {
                ui.label("Provider: ");
                ui.text_edit_singleline(&mut self.provider);
            });

            ui.label("Prompt: ");
            ui.text_edit_multiline(&mut self.prompt);

            if ui.button("Generate").clicked() {
                // This is where we would call the AI provider
                self.output = format!("Generated text for prompt: {}", self.prompt);
            }

            ui.label("Output: ");
            ui.text_edit_multiline(&mut self.output);
        });
    }
}
