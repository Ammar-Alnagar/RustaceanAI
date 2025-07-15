# Rust AI

This is a full and complete AI system thats built in pure RUST.

It will have options of using openai models , gemini models , openrouter , huggingface or even local models.

## Features

*   **Multiple AI Providers:** Switch between different AI providers on the fly.
*   **Local Models:** Use your own custom models.
*   **CLI, TUI, and API:** Interact with the system from the command line, a text-based user interface, or from other applications.
*   **Chat Mode:** Have a conversation with the AI.
*   **File Generation:** Generate entire files based on a prompt.
*   **Agents Mode:** Delegate tasks to a team of AI agents.

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
    A[User] -->|CLI, TUI, or API| B(Rust AI)
    B --> C{Mode?}
    C -->|Generate| D(Provider Selection)
    C -->|Chat| E(Chat Interface)
    C -->|File| F(File Generation)
    C -->|Agents| G(Agents Interface)
    D --> H{Provider?}
    H -->|OpenAI| I[OpenAI]
    H -->|Gemini| J[Gemini]
    H -->|OpenRouter| K[OpenRouter]
    H -->|HuggingFace| L[HuggingFace]
    H -->|Local| M[Local Model]
    I --> N[Generated Text]
    J --> N
    K --> N
    L --> N
    M --> N
    E --> N
    F --> N
    G --> N
    N --> A
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
