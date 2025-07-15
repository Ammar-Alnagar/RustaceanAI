use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rust_ai::{AI, Provider};
use serde::Deserialize;

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
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(generate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
