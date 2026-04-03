# 3) Reviewer Traceability Experience

## Objective

Make reviewer-facing lineage queries first-class, fast, and understandable using only sidecar data.

## Scope

- day-one query surfaces
- status/summary command experience
- human-readable and JSON output formats
- references from package to diff/commit/approval evidence

## Deliverables

- query rendering for:
  - "Why is this change safe to merge?"
  - "What agent/tool actions produced this diff?"
  - "Which validations and approvals are still missing?"
- `avc status --package <id>` command with concise summary output
- consistent references in output (`packageId`, `eventId`, `commitSha`, check names)

## Detailed Work Plan

### A. Query Data Model

- Define a derived in-memory view from `intent.json` + `events.ndjson`.
- Build reducers for:
  - latest lifecycle state,
  - approvals by scope,
  - gate/check completion map,
  - unresolved blockers.

### B. Query Output Contracts

- Text output:
  - one-screen summary,
  - unresolved blockers grouped by type.
- JSON output:
  - machine-readable fields for CI/reporting integrations.

### C. Command Surface

- Add/implement:
  - `avc status --package <id>`
  - optional focus filters (`--scope`, `--show-events`, `--show-missing`).
- Ensure query commands are read-only and do not mutate sidecar files.

### D. Reviewer Workflow Validation

- Run against at least three realistic package histories:
  - low-risk straightforward pass,
  - medium-risk deferred approval,
  - high-risk with rollback metadata requirement.
- Collect qualitative feedback: "Could reviewer decide merge safety in <2 minutes?"

## Acceptance Criteria

- All day-one questions are answerable from sidecar/index data alone.
- Query output clearly identifies missing approvals/checks.
- `avc status` succeeds on packages with >100 events without unusable latency.

## Dependencies

- `1-schema-storage-foundation.md`
- `2-cli-workflow-implementation.md`

## Risks and Mitigations

- **Risk:** query output is too verbose to be useful.
  - **Mitigation:** default concise mode with optional expanded views.
- **Risk:** missing references make trust difficult.
  - **Mitigation:** require source event ids and check refs in all summaries.

## Tickets (Ordered)

### SF-010 Reviewer read model reducers

**Precedence:** 10  
**Depends on:** SF-007, SF-008, SF-009

Tasks:

- Build reducers for lifecycle, approvals, gate states, and unresolved blockers.
- Derive view model from `intent.json` + `events.ndjson` + commit index.
- Validate reducer behavior against fixture histories.

Definition of done:

- Reducers produce consistent summaries for low/medium/high risk histories.

### SF-011 `avc status` query and output contracts

**Precedence:** 11  
**Depends on:** SF-010

Tasks:

- Implement `avc status --package <id>` text output and `--output json`.
- Add optional focus flags (`--scope`, `--show-events`, `--show-missing`).
- Ensure command is read-only and performs no sidecar mutation.

Definition of done:

- Day-one reviewer questions are answerable from `avc status`.
- Output includes package, event, commit, and check references.
