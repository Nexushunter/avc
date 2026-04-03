use crate::config::RuntimeMode;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "avc", version, about = "Agentic version control CLI")]
pub struct Cli {
    /// Repository root path
    #[arg(long, default_value = ".")]
    pub repo: String,
    /// Output format: text or json
    #[arg(long, default_value = "text")]
    pub output: String,
    /// Include verbose debug details
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
    /// Force runtime mode for this invocation
    #[arg(long, value_enum)]
    pub mode: Option<CliMode>,
    /// Explicit toggle for git-compatible mode
    #[arg(long, default_value_t = false)]
    pub git_compatible: bool,
    /// Explicit toggle for fallback behavior
    #[arg(long, default_value_t = false)]
    pub fallback_mode: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliMode {
    #[value(name = "ledger-first")]
    LedgerFirst,
    #[value(name = "git-compatible")]
    GitCompatible,
}

impl From<CliMode> for RuntimeMode {
    fn from(value: CliMode) -> Self {
        match value {
            CliMode::LedgerFirst => RuntimeMode::LedgerFirst,
            CliMode::GitCompatible => RuntimeMode::GitCompatible,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Declare intent and create a new change package
    Plan {
        #[arg(long)]
        title: String,
        #[arg(long)]
        goal: Option<String>,
    },
    /// Execute agent workflows for an existing package
    Run {
        #[arg(long)]
        package: String,
        #[arg(long)]
        agent: Option<String>,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long, default_value_t = false)]
        validate: bool,
    },
    /// Apply policy checks and record approval decisions
    Approve {
        #[arg(long)]
        package: String,
        #[arg(long)]
        reviewer: String,
        #[arg(long)]
        scope: Option<String>,
        #[arg(long, default_value_t = false)]
        defer: bool,
    },
    /// Finalize an approved package
    Merge {
        #[arg(long)]
        package: String,
        #[arg(long)]
        target: Option<String>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}

pub fn execute_command(command: Commands, mode: RuntimeMode, fallback_mode: bool) {
    println!("runtime: mode={mode} fallback_mode={fallback_mode}");

    match command {
        Commands::Plan { title, goal } => {
            println!("plan: title={title} goal={goal:?}");
        }
        Commands::Run {
            package,
            agent,
            provider,
            validate,
        } => {
            println!(
                "run: package={package} agent={agent:?} provider={provider:?} validate={validate}"
            );
        }
        Commands::Approve {
            package,
            reviewer,
            scope,
            defer,
        } => {
            println!("approve: package={package} reviewer={reviewer} scope={scope:?} defer={defer}");
        }
        Commands::Merge {
            package,
            target,
            dry_run,
        } => {
            println!("merge: package={package} target={target:?} dry_run={dry_run}");
        }
    }
}
