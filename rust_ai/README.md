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

## Flowcharts

### High-Level Overview

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

### API Request Flow

```mermaid
sequenceDiagram
    participant User
    participant Actix
    participant AI
    participant Provider

    User->>Actix: POST /generate
    Actix->>AI: generate(provider, prompt)
    AI->>Provider: generate(prompt)
    Provider-->>AI: "Generated Text"
    AI-->>Actix: "Generated Text"
    Actix-->>User: 200 OK
```

### CLI Request Flow

```mermaid
graph TD
    A[User] -->|`cargo run`| B(Clap)
    B --> C{Parse Arguments}
    C --> D[AI]
    D --> E{Provider?}
    E -->|OpenAI| F[OpenAI]
    E -->|Gemini| G[Gemini]
    E -->|OpenRouter| H[OpenRouter]
    E -->|HuggingFace| I[HuggingFace]
    E -->|Local| J[Local Model]
    F --> K[Generated Text]
    G --> K
    H --> K
    I --> K
    J --> K
    K --> L[stdout]
    L --> A
```

## Documentation

For more detailed documentation, see the `docs` directory.
