# Storage Investigation Proposal

## Purpose

Decide where Banana Ledger events should live for a single-repo pilot while preserving reviewer traceability, immutable lineage, and future scale.

## Current Direction

Decision: choose **repo sidecar metadata** as the pilot architecture.

Long-term product goal: evolve toward a CLI that can **replace Git** for day-to-day developer workflows, with Git compatibility as a transitional mode.

## Decision Question

Should lifecycle events be stored as:

1. repo sidecar metadata, or
2. external event store with commit/PR references?

## Scope

- In scope:
  - storage architecture for lifecycle events and references;
  - read performance for reviewer-facing queries;
  - write behavior under high-volume agent runs;
  - operational and security implications.
- Out of scope:
  - full production rollout;
  - provider adapter internals;
  - incident replay tooling beyond minimum feasibility checks.

## Candidate Architectures

### Option A: Repo Sidecar Metadata

Store event data in versioned files (for example, under `docs/` or `.avc/`) linked to commits and PRs.

Pros:

- Simple mental model; everything is in Git.
- Easy branch-local experimentation.
- Low infrastructure overhead at pilot start.

Cons:

- Poor write scalability with many events.
- Harder cross-branch/global queries.
- Potential repo bloat and merge conflict pressure.

### Option B: External Event Store

Store events in a dedicated database/stream store; keep commit sha and PR id references in event records.

Pros:

- Better write throughput and query flexibility.
- Stronger separation of storage lifecycle and retention controls.
- Better fit for future multi-repo expansion.

Cons:

- More infrastructure and operational burden.
- Requires ingestion reliability and idempotency handling.
- Two-system workflow may reduce transparency without good tooling.

### Option C: Hybrid (Likely Evolution Path)

Persist canonical events externally, but keep lightweight summary artifacts in repo for local transparency and review context.

Pros:

- Balances performance with developer visibility.
- Enables phased migration.
- Keeps pull-request ergonomics strong.

Cons:

- Highest design complexity if adopted too early.
- Needs strict contract between summary and source-of-truth event records.

## Decision Summary

- Selected option for pilot: **Option A (Repo Sidecar Metadata)**.
- Why now: best developer ergonomics, low operational overhead, and fastest path to validating reviewer-traceability UX.
- Not selected for pilot:
  - Option B adds infrastructure cost before we validate core workflows.
  - Option C remains a likely migration pattern after pilot evidence.

## Evaluation Criteria

Score each option from 1 (poor) to 5 (strong):

- Reviewer timeline query latency (`P95` target under load).
- Write throughput during peak agent activity.
- Data integrity and immutability guarantees.
- Security controls (field access, redaction policy, encryption posture).
- Retention and archival flexibility.
- Operational complexity (oncall, backup/restore, migrations).
- Local developer ergonomics (branch workflows, debuggability).

## Method (Updated for Selected Direction)

1. Define representative workloads:
  - low volume: 1-2 agent runs per task;
  - medium volume: concurrent CI + agent activity;
  - high volume: multiple agents and retries in parallel.
2. Implement Option A with a strict sidecar schema (under `.avc/`).
3. Validate day-one reviewer queries on real branch/PR flows.
4. Measure file growth, merge friction, and query responsiveness.
5. Define explicit migration triggers to external/hybrid storage.

## Day-One Query Set (Must Pass)

- "Why is this change safe to merge?"
- "What agent/tool actions produced this diff?"
- "Which validations and approvals are still missing?"

## Exit Criteria

Investigation is complete when:

- pilot architecture is selected and documented;
- known tradeoffs are documented and accepted;
- a phased implementation path is defined for the single-repo pilot.

## Git-Replacement Migration Path

### Phase 1: Git-Compatible Sidecar (now)

- Store lifecycle events in repo sidecars.
- Continue using commits/branches/PRs as current transport and review primitives.
- Ensure all ledger links reference commit sha and change-package id.

### Phase 2: Ledger-First CLI (transition)

- Introduce CLI commands as primary user interface (`avc plan`, `avc run`, `avc approve`, `avc merge`).
- Generate Git artifacts as compatibility outputs, not as the primary source-of-truth.
- Maintain bidirectional mapping between ledger ids and Git ids.

### Phase 3: Git-Optional Runtime (replacement target)

- Make CLI-native operations first-class without requiring Git semantics.
- Keep Git export/import adapters for interoperability.
- Treat Git as one backend format among several, rather than the central model.

## Re-Evaluation Triggers

Move from repo sidecar to hybrid/external when one or more conditions are met:

- sidecar data growth causes noticeable repo performance issues;
- merge conflicts in sidecar files become frequent;
- reviewer query latency exceeds agreed `P95` targets;
- multi-repo coordination becomes a primary use case;
- security/retention policies require stronger central enforcement.

## Recommendation Template (For Future Revisit)

Use this section after investigation:

- Recommended option:
- Why this option wins for pilot:
- Measured evidence summary:
- Risks and mitigations:
- Migration plan (next two phases):
- Re-evaluation trigger (when to revisit architecture):