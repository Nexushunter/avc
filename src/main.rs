mod cli;
mod config;

use clap::Parser;
use cli::{execute_command, Cli};
use config::ensure_and_load_config;
use std::path::PathBuf;
use std::process;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(err.exit_code());
    }
}

fn run() -> Result<(), MainError> {
    let cli = Cli::parse();
    let repo_root = PathBuf::from(&cli.repo);

    ensure_and_load_config(&repo_root)?;
    execute_command(cli.command);
    Ok(())
}

#[derive(Debug)]
enum MainError {
    Config(std::io::Error),
}

impl MainError {
    fn exit_code(&self) -> i32 {
        match self {
            Self::Config(_) => 3,
        }
    }
}

impl std::fmt::Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(err) => write!(f, "{err}"),
        }
    }
}

impl From<std::io::Error> for MainError {
    fn from(value: std::io::Error) -> Self {
        Self::Config(value)
    }
}
