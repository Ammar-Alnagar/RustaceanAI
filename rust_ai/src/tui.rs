use cursive::views::{Dialog, EditView, SelectView, TextView};
use cursive::Cursive;
use crate::{AI, Provider};

enum Mode {
    Generate,
    Chat,
    File,
    Agents,
}

pub fn run() {
    let mut siv = cursive::default();

    let mut select = SelectView::new()
        .item("Generate", Mode::Generate)
        .item("Chat", Mode::Chat)
        .item("File", Mode::File)
        .item("Agents", Mode::Agents);

    let select = select.on_submit(|s, mode| {
        s.pop_layer();
        match mode {
            Mode::Generate => show_provider_select(s),
            Mode::Chat => {
                s.set_user_data(Provider::OpenAI); // Default to OpenAI for chat
                show_chat(s)
            }
            Mode::File => show_file_generation(s),
            Mode::Agents => show_agents(s),
        }
    });

    siv.add_layer(Dialog::around(select).title("Select a mode"));

    siv.run();
}

fn show_provider_select(s: &mut Cursive) {
    let mut select = SelectView::new()
        .item("OpenAI", Provider::OpenAI)
        .item("Gemini", Provider::Gemini)
        .item("OpenRouter", Provider::OpenRouter)
        .item("HuggingFace", Provider::HuggingFace)
        .item("Local", Provider::Local);

    let select = select.on_submit(|s, provider| {
        s.set_user_data(provider.clone());
        s.pop_layer();
        show_prompt(s, provider.clone());
    });

    s.add_layer(Dialog::around(select).title("Select a provider"));
}

fn show_prompt(s: &mut Cursive, provider: Provider) {
    s.add_layer(
        Dialog::new()
            .title("Enter a prompt")
            .content(EditView::new().on_submit(move |s, prompt| {
                s.pop_layer();
                show_generation(s, provider.clone(), prompt.to_string());
            }))
            .button("Back", |s| {
                s.pop_layer();
            }),
    );
}

fn show_agents(s: &mut Cursive) {
    s.add_layer(
        Dialog::new()
            .title("Enter a task for the agents")
            .content(EditView::new().on_submit(move |s, task| {
                s.pop_layer();
                let team = crate::agents::Team::new();
                let response = team.delegate(task);
                s.add_layer(
                    Dialog::around(TextView::new(response))
                        .title("Agents Response")
                        .button("Ok", |s| {
                            s.pop_layer();
                        }),
                );
            }))
            .button("Back", |s| {
                s.pop_layer();
            }),
    );
}

fn show_file_generation(s: &mut Cursive) {
    s.add_layer(
        Dialog::new()
            .title("Enter an output file path")
            .content(EditView::new().on_submit(move |s, output| {
                s.pop_layer();
                show_provider_select_for_file(s, output.to_string());
            }))
            .button("Back", |s| {
                s.pop_layer();
            }),
    );
}

fn show_provider_select_for_file(s: &mut Cursive, output: String) {
    let mut select = SelectView::new()
        .item("OpenAI", Provider::OpenAI)
        .item("Gemini", Provider::Gemini)
        .item("OpenRouter", Provider::OpenRouter)
        .item("HuggingFace", Provider::HuggingFace)
        .item("Local", Provider::Local);

    let select = select.on_submit(move |s, provider| {
        s.pop_layer();
        show_prompt_for_file(s, provider.clone(), output.clone());
    });

    s.add_layer(Dialog::around(select).title("Select a provider"));
}

fn show_prompt_for_file(s: &mut Cursive, provider: Provider, output: String) {
    s.add_layer(
        Dialog::new()
            .title("Enter a prompt")
            .content(EditView::new().on_submit(move |s, prompt| {
                s.pop_layer();
                let ai = AI::new(provider.clone());
                let response = ai.generate(prompt);
                std::fs::write(&output, response).unwrap();
                s.add_layer(
                    Dialog::around(TextView::new(format!("File '{}' generated successfully!", output)))
                        .title("Success")
                        .button("Ok", |s| {
                            s.pop_layer();
                        }),
                );
            }))
            .button("Back", |s| {
                s.pop_layer();
            }),
    );
}

fn show_chat(s: &mut Cursive) {
    if s.user_data::<Vec<String>>().is_none() {
        s.set_user_data(Vec::<String>::new());
    }

    let history = s.user_data::<Vec<String>>().unwrap().clone();
    let mut conversation = String::new();
    for message in &history {
        conversation.push_str(message);
        conversation.push('\n');
    }

    s.add_layer(
        Dialog::new()
            .title("Chat")
            .content(
                TextView::new(conversation)
            )
            .content(
                EditView::new().on_submit(move |s, message| {
                    let provider = s.user_data::<Provider>().unwrap().clone();
                    let history = s.user_data::<Vec<String>>().unwrap();
                    history.push(format!("User: {}", message));

                    let ai = AI::new(provider);
                    let response = ai.chat(history);
                    history.push(format!("AI: {}", response));

                    s.pop_layer();
                    show_chat(s);
                }),
            )
            .button("Back", |s| {
                s.pop_layer();
            }),
    );
}

fn show_generation(s: &mut Cursive, provider: Provider, prompt: String) {
    let ai = AI::new(provider);
    let response = ai.generate(&prompt);

    s.add_layer(
        Dialog::around(TextView::new(response))
            .title("Generated Text")
            .button("Back", |s| {
                s.pop_layer();
            }),
    );
}
