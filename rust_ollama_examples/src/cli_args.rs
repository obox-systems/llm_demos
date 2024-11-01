use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,

    #[arg(short)]
    pub model_name: String,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Chat with an LLM model.
    Chat,

    /// Generate embedding of a file.
    Embeddings { file: PathBuf },
}
