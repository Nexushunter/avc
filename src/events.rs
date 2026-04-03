#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventEnvelope {
    pub event_id: String,
    pub event_type: String,
    pub actor: String,
    pub occurred_at: String,
    pub package_id: String,
    pub payload: Value,
    #[serde(default)]
    pub references: EventReferences,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventReferences {
    #[serde(default)]
    pub supersedes_event_id: Option<String>,
}

impl EventEnvelope {
    fn sort_key(&self) -> (&str, &str) {
        (&self.occurred_at, &self.event_id)
    }
}

pub fn append_events(events_path: &Path, mut new_events: Vec<EventEnvelope>) -> io::Result<()> {
    if new_events.is_empty() {
        return Ok(());
    }

    // Ensure deterministic append order for equivalent input sets.
    new_events.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));

    let existing_events = load_events(events_path)?;
    validate_existing_event_stream(&existing_events)?;
    validate_new_events(&existing_events, &new_events)?;

    if let Some(last_existing) = existing_events.last() {
        let first_new = &new_events[0];
        if first_new.sort_key() < last_existing.sort_key() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "new events violate append-only ordering against existing stream",
            ));
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(events_path)?;

    for event in new_events {
        let line = serde_json::to_string(&event).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("failed to serialize event '{}': {err}", event.event_id),
            )
        })?;
        writeln!(file, "{line}")?;
    }

    Ok(())
}

fn load_events(events_path: &Path) -> io::Result<Vec<EventEnvelope>> {
    if !events_path.exists() {
        return Ok(Vec::new());
    }

    let raw = fs::read_to_string(events_path)?;
    let mut events = Vec::new();
    for (line_no, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let event = serde_json::from_str::<EventEnvelope>(line).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid event at line {}: {err}", line_no + 1),
            )
        })?;
        events.push(event);
    }
    Ok(events)
}

fn validate_existing_event_stream(existing: &[EventEnvelope]) -> io::Result<()> {
    let mut seen_ids = HashSet::new();
    for window in existing.windows(2) {
        if window[1].sort_key() < window[0].sort_key() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "existing event stream is not ordered",
            ));
        }
    }
    for event in existing {
        if !seen_ids.insert(event.event_id.as_str()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "existing event stream has duplicate event id '{}'",
                    event.event_id
                ),
            ));
        }
    }
    Ok(())
}

fn validate_new_events(existing: &[EventEnvelope], new_events: &[EventEnvelope]) -> io::Result<()> {
    let mut known_ids: HashSet<&str> = existing.iter().map(|e| e.event_id.as_str()).collect();
    let mut seen_new_ids = HashSet::new();

    for event in new_events {
        if event.event_id.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "event id cannot be empty",
            ));
        }
        if !seen_new_ids.insert(event.event_id.as_str())
            || known_ids.contains(event.event_id.as_str())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("duplicate event id '{}'", event.event_id),
            ));
        }

        if let Some(superseded) = &event.references.supersedes_event_id
            && !known_ids.contains(superseded.as_str()) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "supersedes reference '{}' not found for event '{}'",
                        superseded, event.event_id
                    ),
                ));
            }

        known_ids.insert(event.event_id.as_str());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{EventEnvelope, EventReferences, append_events};
    use serde_json::json;
    use std::fs;

    fn event(id: &str, occurred_at: &str) -> EventEnvelope {
        EventEnvelope {
            event_id: id.to_string(),
            event_type: "test.event".to_string(),
            actor: "implementation-agent".to_string(),
            occurred_at: occurred_at.to_string(),
            package_id: "pkg-1".to_string(),
            payload: json!({"ok": true}),
            references: EventReferences::default(),
        }
    }

    #[test]
    fn appends_in_deterministic_sorted_order() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("events.ndjson");

        let unsorted = vec![
            event("evt-b", "2026-04-03T10:00:00Z"),
            event("evt-a", "2026-04-03T10:00:00Z"),
            event("evt-c", "2026-04-03T10:00:01Z"),
        ];

        append_events(&path, unsorted).expect("append events");

        let content = fs::read_to_string(path).expect("read events");
        let ids: Vec<String> = content
            .lines()
            .map(|line| {
                serde_json::from_str::<EventEnvelope>(line)
                    .expect("parse event line")
                    .event_id
            })
            .collect();

        assert_eq!(ids, vec!["evt-a", "evt-b", "evt-c"]);
    }

    #[test]
    fn rejects_new_event_that_breaks_append_order() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("events.ndjson");

        append_events(&path, vec![event("evt-existing", "2026-04-03T10:00:01Z")])
            .expect("append existing");

        let result = append_events(&path, vec![event("evt-older", "2026-04-03T10:00:00Z")]);
        assert!(result.is_err());
    }

    #[test]
    fn validates_supersedes_reference() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("events.ndjson");
        append_events(&path, vec![event("evt-base", "2026-04-03T10:00:00Z")])
            .expect("append existing");

        let mut superseding = event("evt-supersede", "2026-04-03T10:00:01Z");
        superseding.references = EventReferences {
            supersedes_event_id: Some("evt-base".to_string()),
        };
        append_events(&path, vec![superseding]).expect("append superseding event");

        let mut invalid = event("evt-invalid", "2026-04-03T10:00:02Z");
        invalid.references = EventReferences {
            supersedes_event_id: Some("evt-does-not-exist".to_string()),
        };
        let result = append_events(&path, vec![invalid]);
        assert!(result.is_err());
    }
}
