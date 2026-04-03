# Storage Investigation Proposal

## Purpose

Decide where Banana Ledger events should live for a single-repo pilot while preserving reviewer traceability, immutable lineage, and future scale.

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

## Evaluation Criteria

Score each option from 1 (poor) to 5 (strong):

- Reviewer timeline query latency (`P95` target under load).
- Write throughput during peak agent activity.
- Data integrity and immutability guarantees.
- Security controls (field access, redaction policy, encryption posture).
- Retention and archival flexibility.
- Operational complexity (oncall, backup/restore, migrations).
- Local developer ergonomics (branch workflows, debuggability).

## Method

1. Define representative workloads:
  - low volume: 1-2 agent runs per task;
  - medium volume: concurrent CI + agent activity;
  - high volume: multiple agents and retries in parallel.
2. Implement thin prototypes for Option A and Option B.
3. Run the same synthetic event traces through both prototypes.
4. Measure query and write performance, plus operational friction notes.
5. Draft a recommendation and migration path (including hybrid timing).

## Day-One Query Set (Must Pass)

- "Why is this change safe to merge?"
- "What agent/tool actions produced this diff?"
- "Which validations and approvals are still missing?"

## Exit Criteria

Investigation is complete when:

- a preferred option is selected with evidence;
- known tradeoffs are documented and accepted;
- a phased implementation path is defined for the single-repo pilot.

## Recommendation Template

Use this section after investigation:

- Recommended option:
- Why this option wins for pilot:
- Measured evidence summary:
- Risks and mitigations:
- Migration plan (next two phases):
- Re-evaluation trigger (when to revisit architecture):