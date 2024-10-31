use std::{thread::ThreadId, time::Duration};

use anyhow::{bail, Context};
use async_openai::{
    config::OpenAIConfig,
    types::{
        AssistantObject, CreateAssistantRequest, CreateAssistantRequestArgs,
        CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequest, MessageContent,
        MessageRole, RunStatus, ThreadObject,
    },
    Client,
};
use clap::Parser;
use cli_args::{CliArgs, CliCommand};
use dialoguer::Input;
use tokio::time::sleep;

mod cli_args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let client = Client::default();

    match args.command {
        CliCommand::CreateAssistant {
            name,
            llm_model,
            instruction,
        } => create_assistant(&client, name, llm_model, instruction).await?,

        CliCommand::NewChat { assistant_name } => {
            let thread = client
                .threads()
                .create(CreateThreadRequest::default())
                .await?;

            let asst = load_assistant(&client, &assistant_name).await?;

            loop {
                let input: String = Input::new()
                    .with_prompt("User")
                    .interact_text()
                    .context("unable to get input from console")?;

                if input == "/q" {
                    break;
                }

                client
                    .threads()
                    .messages(&thread.id)
                    .create(
                        CreateMessageRequestArgs::default()
                            .role(MessageRole::User)
                            .content(input)
                            .build()?,
                    )
                    .await?;

                let run = client
                    .threads()
                    .runs(&thread.id)
                    .create(
                        CreateRunRequestArgs::default()
                            .assistant_id(&asst.id)
                            .build()?,
                    )
                    .await?;

                loop {
                    let run = client.threads().runs(&thread.id).retrieve(&run.id).await?;

                    match run.status {
                        RunStatus::Completed => {
                            println!(
                                "Assistant: {}",
                                get_last_message(&client, &thread.id).await?
                            );
                            break;
                        }

                        RunStatus::Queued | RunStatus::InProgress => {}

                        other => {
                            println!("Error while run: {:?}", other);
                            break;
                        }
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
        CliCommand::ContinueChat {
            assistant_name,
            thread_id,
        } => {
            let thread = client.threads().retrieve(&thread_id).await?;

            let asst = load_assistant(&client, &assistant_name).await?;
        }
    }

    Ok(())
}

async fn create_assistant(
    client: &Client<OpenAIConfig>,
    name: String,
    llm_model: String,
    instruction: String,
) -> anyhow::Result<()> {
    println!("Creating assistant...");

    let request = CreateAssistantRequestArgs::default()
        .name(name)
        .model(llm_model)
        .instructions(instruction)
        .build()?;

    client.assistants().create(request).await?;

    println!("Assistant successfully created!");

    Ok(())
}

async fn load_assistant(
    client: &Client<OpenAIConfig>,
    name: impl AsRef<str>,
) -> anyhow::Result<AssistantObject> {
    println!("Loading assistant...");

    let assistants = client.assistants().list(&[("limit", "100")]).await?;

    let asst_obj = assistants
        .data
        .into_iter()
        .find(|a| a.name.as_ref().map(|s| s.as_str()) == Some(name.as_ref()));

    if let Some(asst) = asst_obj {
        println!("Assistant successfully loaded!");
        Ok(asst)
    } else {
        bail!("Couldn't find assistant")
    }
}

async fn get_last_message(
    client: &Client<OpenAIConfig>,
    thread_id: impl AsRef<str>,
) -> anyhow::Result<String> {
    let messages = client
        .threads()
        .messages(thread_id.as_ref())
        .list(&[("limit", "1")])
        .await?;

    Ok(
        match messages
            .data
            .into_iter()
            .next()
            .unwrap()
            .content
            .into_iter()
            .next()
            .unwrap()
        {
            MessageContent::Text(t) => t.text.value,
            _ => bail!("unsupported message type"),
        },
    )
}
