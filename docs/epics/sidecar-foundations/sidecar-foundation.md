# Side Foundation Epic

## Context

This epic defines the actionable body of work for **Phase 1: Git-Compatible Sidecar** from the storage investigation.

Primary source: `docs/proposals/storage-investigation.md`

## Phase 1 Objective

Deliver a production-usable pilot where `avc plan/run/approve/merge` can execute end-to-end in one repository with sidecar-backed lineage and reviewer traceability.

## Workstream Breakdown

Detailed bodies of work now live in `docs/epics/sidecar-foundations/`:

1. `docs/epics/sidecar-foundations/0-ticket-order-of-operations.md`
2. `docs/epics/sidecar-foundations/1-schema-storage-foundation.md`
3. `docs/epics/sidecar-foundations/2-cli-workflow-implementation.md`
4. `docs/epics/sidecar-foundations/3-reviewer-traceability-experience.md`
5. `docs/epics/sidecar-foundations/4-safety-governance-controls.md`
6. `docs/epics/sidecar-foundations/5-observability-performance-baseline.md`

Each file contains:

- objective and scope;
- detailed task plan;
- acceptance criteria;
- dependencies;
- risks and mitigations.

## Milestones

- **M1: Config + schema locked**
  - Done when config validation and sidecar schemas are stable and documented.
- **M2: Command path wired**
  - Done when all four commands produce valid sidecar events and references.
- **M3: Reviewer queries usable**
  - Done when the day-one query set is answerable on real branch/PR flows.
- **M4: Pilot readiness report**
  - Done when metrics, risks, and migration triggers are collected and reviewed.

## Deliverables

- `docs/cli/vision.md` aligned with implemented command behavior.
- `docs/cli/command-spec.md` and `docs/cli/config-spec.md` validated against runtime behavior.
- A sample package fixture under `.avc/packages/<id>/` demonstrating full lifecycle.
- Pilot report with:
  - performance metrics;
  - known friction points;
  - recommendation to continue sidecar or trigger hybrid/external transition.

## Exit Criteria

Phase 1 is complete when all conditions hold:

- every command (`plan`, `run`, `approve`, `merge`) is executable in sequence;
- sidecar artifacts are generated deterministically and pass schema validation;
- day-one reviewer queries are answered from sidecar/index data alone;
- fail-closed security behavior is validated by negative tests;
- pilot metrics are captured for at least one low, one medium, and one high workload.

## Risks and Mitigations

- **Risk:** sidecar merge conflicts become noisy.
  - **Mitigation:** partition events by package id and keep append-only event files small.
- **Risk:** security-level behavior is inconsistently applied.
  - **Mitigation:** centralize event-write guard and add integration tests per security level.
- **Risk:** query latency regresses as packages grow.
  - **Mitigation:** maintain by-commit indexes and cap expensive full scans in CLI commands.