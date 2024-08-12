use crate::command::CommandExecutor;
use crate::config::{load_default_config, Config};
use crate::error::HumorResult;
use clap::Parser;
use std::path::PathBuf;

mod command;
mod config;
mod error;

/// A tool to execute commands based on YAML configuration files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the humor.yaml file
    #[arg(short, long, default_value = "humor.yaml")]
    config: PathBuf,

    /// Commands to execute
    #[arg(required = true)]
    commands: Vec<String>,
}

fn main() -> HumorResult<()> {
    let args = Args::parse();

    // Load default config
    let default_config = load_default_config()?;

    // Load user config
    let user_config = Config::load(&args.config)?;

    // Merge configs
    let config = Config::merge_configs(default_config, user_config)?;

    // Create command executor
    let executor = CommandExecutor::new(&config);

    // Execute command
    executor.execute(&args.commands)
}
