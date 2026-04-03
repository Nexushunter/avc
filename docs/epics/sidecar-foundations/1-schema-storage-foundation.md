# 1) Schema and Storage Foundation

## Objective

Define stable, versioned sidecar data contracts and config validation so all AVC commands can persist events deterministically and safely.

## Scope

- `.avc/conf.json` contract and validation behavior
- package-level sidecar schemas
- commit index schema
- schema migration/versioning policy

## Deliverables

- JSON schema document for `.avc/conf.json` (aligned to `docs/cli/config-spec.md`)
- schema definitions for:
  - `.avc/packages/<id>/intent.json`
  - `.avc/packages/<id>/events.ndjson` event line contract
  - `.avc/index/by-commit/<sha>.json`
- schema compatibility rules for `schemaVersion`
- sample fixtures for one complete package lifecycle

## Detailed Work Plan

### A. Config Contract Enforcement

- Define required keys, defaults, and strict enum validation:
  - `securityLevel`
  - `eventPersistence.failClosed`
  - policy gate toggles
- Implement config-loading sequence:
  1. discover repo root;
  2. load `.avc/conf.json`;
  3. validate and normalize;
  4. block writes on validation failure.
- Define explicit error mapping to CLI exit codes.

### B. Package Schema Design

- Define `intent.json` fields:
  - `packageId`, `title`, `goal`, `constraints`, `acceptance`, `riskTier`, timestamps.
- Define `events.ndjson` line format:
  - base envelope: `eventId`, `eventType`, `actor`, `occurredAt`, `packageId`, `payload`, `references`.
- Define policy for append-only writes and superseding events.
- Define canonical event ordering guarantees and tie-breakers.

### C. Commit Index Schema

- Define minimal commit lookup object:
  - `commitSha`, `packageIds[]`, `updatedAt`, `lifecycleSummary`.
- Define write/update strategy for index consistency on merge.
- Define index rebuild behavior from package files if index is stale.

### D. Compatibility and Evolution

- Lock initial `schemaVersion` behavior for v1.
- Define forward-compatibility rule: unknown fields allowed, unknown required fields rejected.
- Define migration approach (offline migration command in later phase).

## Acceptance Criteria

- Invalid config blocks all event writes (fail-closed behavior proven).
- All three schema targets are validated in tests/fixtures.
- Two independent runs with same inputs produce schema-valid equivalent outputs.
- Sidecar docs and runtime behavior are aligned.

## Dependencies

- `docs/cli/config-spec.md`
- `docs/cli/command-spec.md`
- Rust CLI crate in `avc/`

## Risks and Mitigations

- **Risk:** schema churn breaks command implementations.
  - **Mitigation:** freeze v1 fields and use additive changes only.
- **Risk:** index drift from package data.
  - **Mitigation:** add deterministic index rebuild path.

## Tickets (Ordered)

### SF-001 Config contract + fail-closed loader

**Precedence:** 1  
**Depends on:** none

Tasks:

- Implement config discovery and parse path for `.avc/conf.json`.
- Validate required fields and enums (`securityLevel`, policy toggles, fail-closed).
- Return config/security-specific exit codes on failure.

Definition of done:

- Invalid or missing config blocks all write commands.
- Behavior matches `docs/cli/config-spec.md`.

### SF-002 Sidecar schema v1

**Precedence:** 2  
**Depends on:** SF-001

Tasks:

- Finalize v1 field sets for `intent.json`, event envelope lines, and commit index.
- Define schema constraints and required/optional fields.
- Add fixture files for a valid package lifecycle.

Definition of done:

- Schemas are documented and used by runtime validation.
- Fixture data passes schema checks.

### SF-003 Event append writer + ordering guarantees

**Precedence:** 3  
**Depends on:** SF-002

Tasks:

- Build append-only writer for `events.ndjson`.
- Enforce event ordering rules and deterministic timestamp tie-break handling.
- Support superseding metadata references in event envelope.

Definition of done:

- Events are appended without in-place mutation.
- Same inputs produce equivalent ordered event sequences.

### SF-004 Commit index update + rebuild strategy

**Precedence:** 4  
**Depends on:** SF-002, SF-003

Tasks:

- Implement write/update path for `.avc/index/by-commit/<sha>.json`.
- Define index rebuild routine from package artifacts.
- Validate index consistency after merge/update paths.

Definition of done:

- Commit index reflects package lifecycle references.
- Rebuild restores consistent state from package files alone.