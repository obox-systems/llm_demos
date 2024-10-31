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

    /// Use LLM to extract experiments in the paper.
    ExtractExperiment { file: PathBuf },
}
