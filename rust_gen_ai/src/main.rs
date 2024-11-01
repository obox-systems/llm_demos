use anyhow::Context;
use clap::Parser;
use cli_args::CliArgs;
use dialoguer::Input;
use genai::{
    chat::{ChatMessage, ChatRequest, MessageContent},
    Client,
};

mod cli_args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let client = Client::default();

    let mut messages = vec![ChatMessage::system("You are a helpful assistant.")];

    loop {
        let input: String = Input::new()
            .with_prompt("User")
            .interact_text()
            .context("unable to get input from console")?;

        if input == "/q" {
            break;
        }

        messages.push(ChatMessage::user(input));

        let request = ChatRequest::new(messages.clone());

        let response = client
            .exec_chat(&args.model_name, request, None)
            .await
            .context("unable to send messages to LLM")?;

        if let Some(MessageContent::Text(message)) = response.content {
            println!("Assistant: {}", message);

            messages.push(ChatMessage::assistant(message));
        } else {
            println!("No response.");
        }
    }

    Ok(())
}
