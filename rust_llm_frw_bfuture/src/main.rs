use std::path::Path;

use anyhow::Context;
use clap::Parser;
use cli_args::{CliArgs, CliCommand};
use dialoguer::Input;
use kalosm_language::task::Task;
use kalosm_language::{chat::Chat, prelude::TextStream};
use kalosm_llama::{Llama, LlamaSource};
use kalosm_sample::ParserExt;
use kalosm_sample::{Parse, Schema};
use serde::Serialize;
use tokio::fs::read_to_string;

mod cli_args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let llm = Llama::builder()
        .with_source(LlamaSource::tiny_llama_1_1b_chat())
        .build()
        .await
        .context("unable to create LLama LLM")?;

    let args = CliArgs::parse();

    match args.command {
        CliCommand::Chat => chat(llm).await?,

        CliCommand::ExtractExperiment { file } => extract_experiment(llm, file).await?,
    }

    Ok(())
}

async fn chat(llm: Llama) -> anyhow::Result<()> {
    // WTF: why whould chat consume LLM??????????????? What if I want to have several chats?

    let mut chat = Chat::builder(llm)
        .with_system_prompt("You are a helpful asistant.")
        .build();

    loop {
        let input: String = Input::new()
            .with_prompt("User")
            .interact_text()
            .context("unable to get input from console")?;

        if input == "/q" {
            break;
        }

        let response = chat.add_message(input).all_text().await;

        println!("Assistant: {}", response);
    }

    Ok(())
}

#[derive(Schema, Parse, Clone, Debug, Serialize)]
struct Experiment {
    materials: Vec<String>,
    methods: Vec<String>,
    results: Vec<String>,
}

async fn extract_experiment(llm: Llama, file: impl AsRef<Path>) -> anyhow::Result<()> {
    let task = Task::builder_for::<Experiment>("Your goal is to read and analyze a research paper. Extract information about the experimant that was went in this paper.").build();

    let source = read_to_string(file)
        .await
        .context("unable to read the file")?;

    let response: Experiment = task
        .run(source, &llm)
        .await
        .context("unable to process file with LLM")?;

    println!("{}", serde_yaml::to_string(&response)?);

    Ok(())
}
