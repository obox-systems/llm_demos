use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Chat with LLM.
    Chat,
    /// Summarize a document.
    Summarize { file_path: PathBuf },
}
