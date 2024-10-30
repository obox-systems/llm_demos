# [Build A Full Stack Chatbot in Rust (feat. Leptos & Rustformers)](https://www.youtube.com/watch?v=vAjle3c9Xqc&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=14)

## Leptos

- Install wasm target (`rustup target add wasm32-unknown-unknown`).
- Create a project with from Leptos template.
- With Leptos you can change code in realtime and you will have a hot-reload.
- Leptos can be a full-stack framework:
  - Backend is `ssr`: server-side rendering.
  - Frontend is `cst`: client-side rendering.

## Frontend

- For better looking you can use Tailwind CSS.
- Components are:
  - App Component - core component:
    - Chat component - chat history.
    - Prompt - text area with "send" button.
- Chat history is a state of the chat component, it stores a vector of messages.
- To manage this state Leptos signals are used.

## Backend

- When user clicks "Submit", a request is send to the backend.
- This request accepts conversation (a vector of messages).
- Rustformers accept a low level chat templates.

## Rustformers usage

1. Load model.
2. Start a session with the model.
3. Make a random device and `InferenceRequest`.
4. Pass all of that to `session.infer(...)`.

The infer will use a callback and provide either an `InferredToken` (generation in process) or a `EotToken` (end of text).

## Back to frontend

- After you got a response from model, add it to the chat history and update the history component.
