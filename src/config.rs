use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvcConfig {
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

pub fn ensure_and_load_config(repo_root: &Path) -> io::Result<AvcConfig> {
    ensure_config_exists(repo_root)?;
    load_config(repo_root)
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
    write_config(&config_path, &AvcConfig::default())?;
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
