# Ledger-First CLI Epic

## Context

This epic defines the actionable body of work for **Phase 2: Ledger-First CLI (transition)** from the storage investigation.

Primary source: `docs/proposals/storage-investigation.md`

## Phase 2 Objective

Make AVC CLI the primary workflow interface while keeping Git as a compatibility output layer, with reliable bidirectional mapping between ledger ids and Git ids.

## Workstream Breakdown

Detailed workstreams live in `docs/epics/ledger-first-cli/`:

0. `docs/epics/ledger-first-cli/0-ticket-order-of-operations.md`
1. `docs/epics/ledger-first-cli/1-cli-command-surface-and-orchestration.md`
2. `docs/epics/ledger-first-cli/2-git-compatibility-adapter-layer.md`
3. `docs/epics/ledger-first-cli/3-ledger-git-id-mapping-and-sync.md`
4. `docs/epics/ledger-first-cli/4-review-and-ci-integration-flow.md`
5. `docs/epics/ledger-first-cli/5-rollout-guardrails-and-adoption.md`

Each file contains:

- objective and scope;
- detailed task plan;
- acceptance criteria;
- dependencies;
- risks and mitigations.

## Workstreams and Steps

### 1) CLI Command Surface and Orchestration

Steps:
- define runtime mode defaults and toggles;
- centralize command runtime context and error mapping;
- enforce stable text/json output envelopes.

### 2) Git Compatibility Adapter Layer

Steps:
- emit deterministic branch/commit artifacts from ledger state;
- emit PR compatibility metadata for review handoff;
- validate generated artifacts with reconciliation checks.

### 3) Ledger-Git ID Mapping and Sync

Steps:
- define mapping schema and update semantics;
- implement bidirectional lookup interfaces;
- detect and repair mapping drift.

### 4) Review and CI Integration Flow

Steps:
- surface gate and approval visibility in CLI and compatibility outputs;
- ingest and normalize check status into merge-readiness;
- validate end-to-end mixed CLI/PR review flows.

### 5) Rollout Guardrails and Adoption

Steps:
- implement feature flags and fallback controls;
- capture adoption and reliability metrics;
- produce Phase 2 readiness recommendation.

## Ticket Breakdown

Ticket docs are step-derived and do not use workstreams as tickets.

- `docs/epics/ledger-first-cli/tickets/index.md`

## Milestones

- **M1: CLI-first command contract stable**
  - Done when command behavior and outputs are ledger-first by default, with compatibility toggles documented.
- **M2: Git compatibility adapter operational**
  - Done when branches/commits/PR metadata can be emitted from ledger state deterministically.
- **M3: Mapping reliability proven**
  - Done when ledger-to-Git and Git-to-ledger lookups are complete for active package flows.
- **M4: Review and CI flow integrated**
  - Done when approvals, checks, and merge readiness are visible in both CLI and Git-compatible surfaces.
- **M5: Transition readiness decision**
  - Done when adoption metrics and rollback criteria support continued progression to Phase 3.

## Deliverables

- Updated CLI command contract and runtime defaults for ledger-first operation.
- Git compatibility adapter implementation and documentation.
- Durable mapping index between `change_package_id`, event ids, commit shas, and PR refs.
- Review/CI integration path with clear policy gate visibility.
- Phase 2 transition report with adoption metrics and readiness recommendation.

## Exit Criteria

Phase 2 is complete when all conditions hold:

- developers can use AVC commands as the primary path without direct Git command dependency for common flows;
- Git artifacts are generated as compatibility outputs from ledger state;
- mapping integrity checks pass for all pilot packages;
- review and CI workflows remain usable for teams still centered on PR processes;
- operational metrics show acceptable latency and failure rates under target workloads.

## Risks and Mitigations

- **Risk:** dual-source confusion between ledger and Git states.
  - **Mitigation:** declare ledger as source of truth and treat Git as derived output.
- **Risk:** compatibility adapter produces inconsistent Git artifacts.
  - **Mitigation:** deterministic generation and reconciliation checks.
- **Risk:** team adoption stalls due to workflow changes.
  - **Mitigation:** staged rollout, fallback mode, and clear migration docs.
