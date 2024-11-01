use std::path::Path;

use anyhow::{bail, Context};
use clap::Parser;
use cli_args::{CliArgs, CliCommand};
use dialoguer::Input;
use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage},
        embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest},
    },
    Ollama,
};
use tokio::fs::read_to_string;

mod cli_args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    match args.command {
        CliCommand::Chat => {
            chat(args.model_name).await?;
        }

        CliCommand::Embeddings { file } => {
            generate_embeddings(file, args.model_name).await?;
        }
    }

    anyhow::Ok(())
}

async fn chat(model_name: impl Into<String>) -> anyhow::Result<()> {
    let ollama = Ollama::default();
    let model_name = model_name.into();

    println!("Pulling model...");

    ollama
        .pull_model(model_name.clone(), false)
        .await
        .context("unable to download ollama model")?;

    println!("Model pulled! Have fun!");

    let mut messages = vec![ChatMessage::system("You are a kind LLM assistant.".into())];

    loop {
        let input: String = Input::new()
            .with_prompt("User")
            .interact_text()
            .context("unable to get input from console")?;

        if input == "/q" {
            break;
        }

        messages.push(ChatMessage::user(input));

        let result = ollama
            .send_chat_messages(ChatMessageRequest::new(
                model_name.clone(),
                messages.clone(),
            ))
            .await
            .context("unable to send messages to Ollama")?;

        if let Some(message) = result.message {
            println!("Assistant: {}", message.content);
        } else {
            bail!("ollama didn't return any message");
        }
    }

    Ok(())
}

async fn generate_embeddings(
    file_path: impl AsRef<Path>,
    model_name: impl Into<String>,
) -> anyhow::Result<()> {
    let model_name = model_name.into();

    eprintln!("Reading file...");

    let source = read_to_string(file_path)
        .await
        .context("unable to read file")?;

    eprintln!("Reading done!");

    let ollama = Ollama::default();

    eprintln!("Pulling model...");

    ollama
        .pull_model(model_name.clone(), false)
        .await
        .context("unable to download ollama model")?;

    eprintln!("Model pulled! Generating embeddings...");

    let result = ollama
        .generate_embeddings(GenerateEmbeddingsRequest::new(
            model_name.into(),
            EmbeddingsInput::Single(source),
        ))
        .await
        .context("unable to generate embeddings from ollama")?;

    eprintln!("Embeddings are generated!");

    println!("{:?}", result.embeddings);

    Ok(())
}
