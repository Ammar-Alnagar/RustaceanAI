use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::{Parser, Subcommand};
use rust_ai::{gui, tui, AI, Provider};
use serde::Deserialize;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the web server
    Web,
    /// Runs the TUI
    Tui,
    /// Runs the GUI
    Gui,
    /// Runs the CLI
    Cli {
        #[clap(short, long)]
        provider: String,
        #[clap(short = 'P', long)]
        prompt: String,
    },
    /// Generates a file
    File {
        #[clap(short, long)]
        provider: String,
        #[clap(short = 'P', long)]
        prompt: String,
        #[clap(short, long)]
        output: String,
    },
}

#[derive(Deserialize)]
struct AIRequest {
    provider: String,
    prompt: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/generate")]
async fn generate(req_body: web::Json<AIRequest>) -> impl Responder {
    let provider = match req_body.provider.as_str() {
        "openai" => Provider::OpenAI,
        "gemini" => Provider::Gemini,
        "openrouter" => Provider::OpenRouter,
        "huggingface" => Provider::HuggingFace,
        "local" => Provider::Local,
        _ => return HttpResponse::BadRequest().body("Invalid provider"),
    };
    let ai = AI::new(provider);
    let response = ai.generate(&req_body.prompt);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Web => {
            HttpServer::new(|| App::new().service(hello).service(generate))
                .bind(("127.0.0.1", 8080))?
                .run()
                .await
        }
        Commands::Tui => {
            tui::run();
            Ok(())
        }
        Commands::Gui => {
            let app = gui::RustAIApp::default();
            let native_options = eframe::NativeOptions::default();
            eframe::run_native("Rust AI", native_options, Box::new(|_cc| Box::new(app))).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        }
        Commands::Cli { provider, prompt } => {
            let provider = match provider.as_str() {
                "openai" => Provider::OpenAI,
                "gemini" => Provider::Gemini,
                "openrouter" => Provider::OpenRouter,
                "huggingface" => Provider::HuggingFace,
                "local" => Provider::Local,
                _ => {
                    println!("Invalid provider");
                    return Ok(());
                }
            };
            let ai = AI::new(provider);
            let response = ai.generate(prompt);
            println!("{}", response);
            Ok(())
        }
        Commands::File {
            provider,
            prompt,
            output,
        } => {
            let provider = match provider.as_str() {
                "openai" => Provider::OpenAI,
                "gemini" => Provider::Gemini,
                "openrouter" => Provider::OpenRouter,
                "huggingface" => Provider::HuggingFace,
                "local" => Provider::Local,
                _ => {
                    println!("Invalid provider");
                    return Ok(());
                }
            };
            let ai = AI::new(provider);
            let response = ai.generate(prompt);
            std::fs::write(output, response)?;
            Ok(())
        }
    }
}
