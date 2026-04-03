use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "avc", version, about = "Agentic version control CLI")]
struct Cli {
    /// Repository root path
    #[arg(long, default_value = ".")]
    repo: String,
    /// Output format: text or json
    #[arg(long, default_value = "text")]
    output: String,
    /// Include verbose debug details
    #[arg(long, default_value_t = false)]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
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
            println!(
                "approve: package={package} reviewer={reviewer} scope={scope:?} defer={defer}"
            );
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
