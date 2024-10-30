# [Learn Rust OpenAI API - Building AI Buddy from Scratch!!!](https://www.youtube.com/watch?v=PHbCmIckV20&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=13)

## LLM connection (`async-openai`)

1. Make an `OpenAIConfig`.
2. Make a `Client`.

## Assistants

Assistants are an OpenAI features for making LLM agents: they will have a system message, tools, files, etc.

- Make an OpenAI assistant with `CreateAssistantRequest`: you need to supply a model, name, tools it can use.
- Introduce loading and creation of assistans.
- Remove assistants.

## Threads

Threads are chat threads.

So per OpenAi API you have several assistants. Each assistant has several threads, and each thread has several messages.

This also means that OpenAI will handle chat management and storing for you.

## CLI

- It's a simple REPL like interface.
- There are special items - commands that start with `/`.
- Other input is treated as a user message to LLM.

## File upload

You need to manually transfer files to OpenAI asisstance. However, this is a bit tedious.

At first you need to upload to "Files" resources, and after that you can add them to assistants.
