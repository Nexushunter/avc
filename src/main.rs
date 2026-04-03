mod cli;
mod config;
mod events;

use clap::Parser;
use cli::{Cli, execute_command};
use config::{RuntimeMode, ensure_and_load_config};
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

    let config = ensure_and_load_config(&repo_root)?;
    let mode = resolve_runtime_mode(&cli, config.default_runtime_mode());
    let fallback_mode = resolve_fallback_mode(&cli, config.allow_fallback_mode());

    execute_command(cli.command, mode, fallback_mode);
    Ok(())
}

fn resolve_runtime_mode(cli: &Cli, config_default_mode: RuntimeMode) -> RuntimeMode {
    if cli.git_compatible {
        return RuntimeMode::GitCompatible;
    }

    if let Some(mode) = cli.mode {
        return mode.into();
    }

    config_default_mode
}

fn resolve_fallback_mode(cli: &Cli, config_allow_fallback_mode: bool) -> bool {
    cli.fallback_mode || config_allow_fallback_mode
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
