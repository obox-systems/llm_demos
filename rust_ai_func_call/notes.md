# [Rust AI Function Calling - Full Tutorial (with async-openai)](https://www.youtube.com/watch?v=2M0PSijLnis&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=2)

## `async-openai`

This video uses crate `async-openai`. It's features are:

- Async support.
- A lot of verbose types for OpenAI API.
- Uses builder pattern.

## Function calling

Function calling in LLMs is implemented this way:

1. A user message is sent along with available functions for LLM.
2. LLM will generate Tool Call chat messages (which are not real chat messages).
3. Host should provide Tool Output messages.
4. LLM then sends back a correct message.

Tool Calls and Tool Output messages are linked with unique ids. Meaning, every Tool Call has an id, and then this id is used back in Tool Output messages.

## Simple chatting

In order to make a simple chat, you would do this:

1. Make an OpenAI client with `Client::default`.
2. To make a chat, make a vector of messages and construct `CreateChatCompletionRequest`.
3. Then you would call a `chat_client.create(req).await` to get the response.

## Sending tools

Firstly, you need to define tools: it should consist of a name, description, and a list of parameters. All parameters are encoded as a JSON object which is a schema of parameters.

In `async-openai` functions are made with `ChatCompletionToolArgs` and `FunctionObject`. Those functions are send in `CreateChatCompletionRequest`.

To retrieve tool calls, they are in the first "choice" object in the field "tool_calls".

Then you need to process that tool_cals (manually or with custom provided utility functions). Then you send the output with `ToolResponse` messages into the chat history vector.
