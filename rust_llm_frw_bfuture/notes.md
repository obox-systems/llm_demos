# [Rust LLM Frameworks have a Bright Future](https://www.youtube.com/watch?v=fIFUnYNuYbc&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=5)

## Basic ecosystem of LLM crates

It's divided into 3 layers:

1. High-level LLM libraries: kalosm, mistral.rs, llm-client, etc.
2. Deap Learning libraries: candle, JAX, Torch, TensorFlow, Burn.
3. Low-level libraries for math: CUDA, OpenCL, and many-many others. (Out of scope of Rust and LLMs).

## Kalosm

### Features

- Basic inference.
- Local and remote models.
- Image generation.
- RAG.
- Direct integration with SurrealDB.
- Audio transcription.

### Workflow

1. Download/get model. E.g.: `LLama::new_chat().await`.
2. Make a chat: `Chat::builder(model).....build()`.
3. Run with `all_text().await`. You will get a message + it will be automatically added to chat.

### Structured generation

- Just add a `Schema` and `Parse` derives.
- In order to use it, use type `Task::builder_for::<YourMomSorryModel>` and supply task purpose.
- After that by calling `run().await` you will get a value in the specified type.

### Problems of structured generation

- You can add constraints to fields. E.g.: `#[parse(range = 20..99)]` - this means that an integer will be in rage [20; 99).
  You can also add regex constraint.

## Candle

- Crate from Hugging Face.
- A lot of LLM crates use Candle under the hood.
- It's possible to use Candle directly, but it will be too verbose.

## Mistral.rs

- More verbose than Kalosm.
- Manual management of "channels".

## llm-client

- Bridge to `llama.cpp`

## Burn

- Meta-framework. For machine learning.
- Supports a lot of backends.

## Rust VS Python

- While Rust is more performant.
- They all use C-based frameworks inside.

Why Rust than Python:

- Language isomorphism: one language for every project - very nice and convinient.
- Memory safety, etc.
