# [Rust Ollama By Examples - 4 Chapters (Full Tutorial - Rust AI)](https://www.youtube.com/watch?v=OcH-zT5VNgM&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=1)

## Generation

1. Initialize ollama with `Ollama::default`.
2. Make a `GenerationRequest` with model and prompt. You can also supply system message.
3. Call `ollama.generate(req).await` with request.

You can also use `generate_stream` to get a streamed output.

## Context

*IMHO: this is so dumb.*

In the final response of a stream output there is a variable called `context`. You can reuse it for next requests so that model remembers the previous converstation.

*IMHO: models have limited context window. This is completely other separate problem. There are various techniques with various pros and cons. Context is not well documented in ollama, and actually it turns out that this is just an array of tokens.*

## Chatting

Chatting is done by storing messages in a vector. And then supplying that vector to `GenerationRequest`.

## Embeddings

Firstly, you need to load file and split it. Secondly, you just call `generate_embeddings` on ollama instance and supply a model and your segment.
