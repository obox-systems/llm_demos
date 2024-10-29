# [Rust genai - Multi-AI Providers Client](https://www.youtube.com/watch?v=uqGso3JD3eE&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=7&t=623s)

## Description

This is a crate with unified access to different LLM providers: OpenAI, Geminim, etc.

## Workflow

1. Make a `Client`.
2. Make a `ChatRequest`..
3. Run with `exec_chat_stream` and specify model and `ChatRequest`.

API keys are retrieved from environment variables.

## IMHO

It's not the 100% right way to use models with only model name. Typically two strings are needed (or types): AI provider (OpenAI, Mistral, Gemini, etc.) and a model name. What both OpenAI and Mistral present a model with the same name? What will `genai` do?
