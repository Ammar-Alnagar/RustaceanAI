# Rust AI

This is a full and complete AI system thats built in pure RUST.

It will have options of using openai models , gemini models , openrouter , huggingface or even local models.

## Features

*   **Multiple AI Providers:** Switch between different AI providers on the fly.
*   **Local Models:** Use your own custom models.
*   **CLI and API:** Interact with the system from the command line or from other applications.

## Usage

### CLI

```bash
cargo run -- --provider <provider> --prompt <prompt>
```

### API

The API is available at `http://127.0.0.1:8080`.

#### `POST /generate`

Request:

```json
{
    "provider": "<provider>",
    "prompt": "<prompt>"
}
```

Response:

```
<generated text>
```

## Flowchart

```mermaid
graph TD
    A[User] -->|CLI or API| B(Rust AI)
    B --> C{Provider?}
    C -->|OpenAI| D[OpenAI]
    C -->|Gemini| E[Gemini]
    C -->|OpenRouter| F[OpenRouter]
    C -->|HuggingFace| G[HuggingFace]
    C -->|Local| H[Local Model]
    D --> I[Generated Text]
    E --> I
    F --> I
    G --> I
    H --> I
    I --> A
```
