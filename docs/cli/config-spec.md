# AVC Config Spec

## Purpose

Define the configuration contract for `.avc/config.json` so CLI behavior, policy enforcement, and documentation remain consistent.

## File Location

- Required path: `.avc/config.json`
- Read timing: before any event persistence
- Failure mode: fail closed if missing or invalid

## Current Schema (v1)

```json
{
  "schemaVersion": "1",
  "securityLevel": "redacted",
  "eventPersistence": {
    "failClosed": true,
    "allowedSecurityLevels": ["full", "redacted", "summary_only"]
  },
  "lifecycle": {
    "immutableEvents": true
  },
  "policy": {
    "requireGatesBeforeApprove": true,
    "requireGatesBeforeMerge": true,
    "requireRollbackMetadataForRiskTiers": ["high"]
  }
}
```

## Field Definitions

### `schemaVersion`

- Type: string
- Required: yes
- Current value: `"1"`
- Purpose: allows future config migrations and backward compatibility checks.

### `securityLevel`

- Type: string enum
- Required: yes
- Allowed values:
  - `full`
  - `redacted`
  - `summary_only`
- Purpose: determines how event payloads are persisted.

Behavior by value:

- `full`: write full payload content.
- `redacted`: mask/remove sensitive fields before write.
- `summary_only`: write minimal metadata only.

### `eventPersistence.failClosed`

- Type: boolean
- Required: yes
- Recommended default: `true`
- Purpose: if `true`, block persistence on invalid/missing config or invalid security-level state.

### `eventPersistence.allowedSecurityLevels`

- Type: array of string enums
- Required: yes
- Purpose: explicit allowlist for accepted `securityLevel` values.
- Constraint: must include at least one valid level.

### `lifecycle.immutableEvents`

- Type: boolean
- Required: yes
- Expected value: `true`
- Purpose: guarantees append-only event model; updates should be represented by new superseding events.

### `policy.requireGatesBeforeApprove`

- Type: boolean
- Required: yes
- Default: `true`
- Purpose: enforce policy gate checks before approval events can be recorded.

### `policy.requireGatesBeforeMerge`

- Type: boolean
- Required: yes
- Default: `true`
- Purpose: enforce policy gate checks before merge operations.

### `policy.requireRollbackMetadataForRiskTiers`

- Type: array of risk tiers
- Required: yes
- Allowed values: `low`, `medium`, `high`
- Typical default: `["high"]`
- Purpose: require rollback metadata for specific risk categories.

## Validation Rules

- `.avc/config.json` must exist and be valid JSON.
- `schemaVersion` must be recognized by the CLI.
- `securityLevel` must be in `eventPersistence.allowedSecurityLevels`.
- If `eventPersistence.failClosed` is `true`, any violation returns a config/security error and no event is written.
- `lifecycle.immutableEvents` must be `true` in v1.

## Example Profiles

### Strict Security

```json
{
  "schemaVersion": "1",
  "securityLevel": "summary_only",
  "eventPersistence": {
    "failClosed": true,
    "allowedSecurityLevels": ["summary_only", "redacted"]
  },
  "lifecycle": {
    "immutableEvents": true
  },
  "policy": {
    "requireGatesBeforeApprove": true,
    "requireGatesBeforeMerge": true,
    "requireRollbackMetadataForRiskTiers": ["medium", "high"]
  }
}
```

### Developer-Friendly

```json
{
  "schemaVersion": "1",
  "securityLevel": "redacted",
  "eventPersistence": {
    "failClosed": true,
    "allowedSecurityLevels": ["full", "redacted", "summary_only"]
  },
  "lifecycle": {
    "immutableEvents": true
  },
  "policy": {
    "requireGatesBeforeApprove": true,
    "requireGatesBeforeMerge": true,
    "requireRollbackMetadataForRiskTiers": ["high"]
  }
}
```

## Compatibility Notes

- v1 assumes sidecar storage under `.avc/`.
- Future versions may add storage backend config while preserving `securityLevel` semantics.
- CLI commands should surface actionable errors when config validation fails.

