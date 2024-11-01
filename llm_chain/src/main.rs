use std::path::Path;

use anyhow::Context;
use clap::Parser;
use cli_args::{CliArgs, CliCommand};
use dialoguer::Input;
use llm_chain::{chains::conversation::Chain, executor, parameters, prompt, step::Step};
use tokio::fs::read_to_string;

mod cli_args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    match args.command {
        CliCommand::Chat => chat().await?,
        CliCommand::Summarize { file_path } => summarize(file_path).await?,
    }

    Ok(())
}

async fn chat() -> anyhow::Result<()> {
    let exec = executor!()?;

    let mut chain = Chain::new(prompt!(system: "You are a helpful assistant."))?;

    loop {
        let input: String = Input::new()
            .with_prompt("User")
            .interact_text()
            .context("unable to get input from console")?;

        if input == "/q" {
            break;
        }

        let result = chain
            .send_message(
                Step::for_prompt_template(prompt!(user: &input)),
                &parameters!(),
                &exec,
            )
            .await?;

        println!("{}", result);
    }

    Ok(())
}

async fn summarize(file_path: impl AsRef<Path>) -> anyhow::Result<()> {
    let exec = executor!()?;

    let source = read_to_string(file_path).await?;

    let map_prompt = Step::for_prompt_template(prompt!(
        "You are a bot for summarizing scientific articles.",
        "Summarize this article into bullet points:\n{{text}}"
    ));

    let reduce_prompt = Step::for_prompt_template(prompt!(
        "You are a bot that summarizes text that came from scientific articles.",
        "Please combine the articles below into one summary as bullet points:\n{{text}}"
    ));

    let docs = vec![parameters!(source)];

    // IMHO: It's such a dumb decisions to name all chains as `Chain` and not `ConversationChain`
    // or `MapReduceChain`.
    let chain = llm_chain::chains::map_reduce::Chain::new(map_prompt, reduce_prompt);

    eprintln!("Summarizing...");

    let result = chain.run(docs, parameters!(), &exec).await?;

    println!("{}", result);

    Ok(())
}
