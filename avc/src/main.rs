use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AvcConfig {
    schema_version: String,
    security_level: SecurityLevel,
    event_persistence: EventPersistenceConfig,
    lifecycle: LifecycleConfig,
    policy: PolicyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SecurityLevel {
    Full,
    Redacted,
    SummaryOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventPersistenceConfig {
    fail_closed: bool,
    allowed_security_levels: Vec<SecurityLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LifecycleConfig {
    immutable_events: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PolicyConfig {
    require_gates_before_approve: bool,
    require_gates_before_merge: bool,
    require_rollback_metadata_for_risk_tiers: Vec<RiskTier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RiskTier {
    Low,
    Medium,
    High,
}

impl Default for AvcConfig {
    fn default() -> Self {
        Self {
            schema_version: "1".to_string(),
            security_level: SecurityLevel::Redacted,
            event_persistence: EventPersistenceConfig::default(),
            lifecycle: LifecycleConfig::default(),
            policy: PolicyConfig::default(),
        }
    }
}

impl Default for EventPersistenceConfig {
    fn default() -> Self {
        Self {
            fail_closed: true,
            allowed_security_levels: vec![
                SecurityLevel::Full,
                SecurityLevel::Redacted,
                SecurityLevel::SummaryOnly,
            ],
        }
    }
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            immutable_events: true,
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            require_gates_before_approve: true,
            require_gates_before_merge: true,
            require_rollback_metadata_for_risk_tiers: vec![RiskTier::High],
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let repo_root = PathBuf::from(&cli.repo);

    if let Err(err) = ensure_config_exists(&repo_root).and_then(|_| load_config(&repo_root)) {
        eprintln!("error: {err}");
        process::exit(3);
    }

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

fn ensure_config_exists(repo_root: &Path) -> io::Result<()> {
    let avc_dir = repo_root.join(".avc");
    let config_path = avc_dir.join("config.json");

    if config_path.exists() {
        return Ok(());
    }

    println!(
        "No AVC config found at '{}'.",
        config_path.to_string_lossy()
    );
    print!("Initialize a fresh AVC repo config now? [y/N]: ");
    io::stdout().flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    let consent = matches!(response.trim().to_ascii_lowercase().as_str(), "y" | "yes");

    if !consent {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "AVC requires .avc/config.json to run.",
        ));
    }

    fs::create_dir_all(&avc_dir)?;
    let default_config = AvcConfig::default();
    write_config(&config_path, &default_config)?;
    println!(
        "Created '{}'. You can edit security and policy settings anytime.",
        config_path.to_string_lossy()
    );

    Ok(())
}

fn load_config(repo_root: &Path) -> io::Result<AvcConfig> {
    let config_path = repo_root.join(".avc").join("config.json");
    let raw = fs::read_to_string(&config_path)?;
    serde_json::from_str::<AvcConfig>(&raw).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Invalid AVC config at '{}': {err}",
                config_path.to_string_lossy()
            ),
        )
    })
}

fn write_config(config_path: &Path, config: &AvcConfig) -> io::Result<()> {
    let serialized = serde_json::to_string_pretty(config).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to serialize AVC config: {err}"),
        )
    })?;
    fs::write(config_path, format!("{serialized}\n"))
}
