use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Create OpenAI assistant.
    CreateAssistant {
        name: String,
        llm_model: String,
        instruction: String,
    },

    /// Start a new chat with OpenAI assistant.
    NewChat { assistant_name: String },

    /// Continue an old chat with OpenAI assistant.
    ContinueChat {
        assistant_name: String,
        thread_id: String,
    },

    /// Upload file to OpenAI account.
    UploadFile { path: PathBuf, name: String },

    /// Attach file to the OpenAI assistant.
    AttachFile { id: String, assitant_name: String },
}
