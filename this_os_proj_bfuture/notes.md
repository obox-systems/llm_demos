# [this open source project has a bright future](https://www.youtube.com/watch?v=jib1wjgIaa4&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=6)

## Description

- Written by 2 previous Docker employees.
- Manages LLMs like Doecker containers.
- Not written in Rust.
- Simple CLI.
- Can run HTTP server with LLM API.
- Like a package manager for LLMs.
- Create new models using templates.
- Has a lot of integrations, like: LSP servers, code analysis/review, Obsidian.

## Templates

It's like a Dockerfile for models.

You can even make a new model out of a model from Hugging Face.

## IMHO

1. There is nothing special and nothing new to ollama.
2. I doubt that this is practical to manage LLMs with a Modelfile (like a Dockerfile).
3. Other projects that "have integration with ollama" - that's so doesn't make sence. There is nothing special, all these projects just require an LLM connection, they can use any API. And the most important thing with those projects is not the ollama, but the algorithms and prompts that have good results.
4. Typically, local models are stupider than others.
5. Local models require a lot of computational power.
6. Ollama is just a frontend for `llama.cpp`, and that's all.
