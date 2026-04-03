# 5) Observability and Performance Baseline

## Objective

Measure whether sidecar storage is viable for Phase 1 by collecting baseline performance and operational metrics across representative workloads.

## Scope

- instrumentation for write/query performance
- sidecar growth and conflict tracking
- low/medium/high synthetic workload runs
- pilot readiness report inputs

## Deliverables

- metric collection plan and metric definitions
- benchmark harness or scripted workload runner
- captured baseline metrics for three workload tiers
- pilot report section with recommendation signals and thresholds

## Detailed Work Plan

### A. Metrics Definition

- Define and capture:
  - event write latency (`P50`, `P95`)
  - reviewer query latency (`P50`, `P95`)
  - sidecar size growth per package
  - merge conflict frequency for sidecar files
  - command failure rates by exit code

### B. Workload Profiles

- Low: 1-2 agent runs per package, minimal retries.
- Medium: concurrent CI + agent activity on active branch workflows.
- High: multiple parallel agent runs with retries and scoped approvals.

### C. Benchmark Execution

- Create deterministic test fixture packages.
- Run workload scripts and record metrics consistently.
- Snapshot sidecar file counts/sizes before and after runs.

### D. Analysis and Decision Support

- Compare observed metrics against Phase 1 viability expectations.
- Identify hotspots:
  - slow query paths,
  - large event files,
  - conflict-prone files.
- Propose short-term mitigations and long-term migration triggers.

## Acceptance Criteria

- Metrics captured for low, medium, and high workloads.
- `P95` reviewer query and write latencies are reported with context.
- Sidecar growth trend documented with at least one package lifecycle sample.
- Findings are consumable in pilot readiness review.

## Dependencies

- `2-cli-workflow-implementation.md`
- `3-reviewer-traceability-experience.md`
- `4-safety-governance-controls.md`

## Risks and Mitigations

- **Risk:** benchmarks do not reflect real workflows.
  - **Mitigation:** include at least one real branch/PR flow in measurement set.
- **Risk:** missing instrumentation hides bottlenecks.
  - **Mitigation:** instrument command boundaries and critical file I/O paths.

## Tickets (Ordered)

### SF-014 Metrics instrumentation and collection

**Precedence:** 14  
**Depends on:** SF-009, SF-011, SF-013

Tasks:

- Instrument command boundaries and key write/read paths.
- Capture `P50/P95` for write and reviewer-query latency.
- Record sidecar growth and command failure rates by exit code.

Definition of done:

- Metrics are emitted consistently for all command flows.
- Baseline collection can run without manual metric patching.

### SF-015 Workload execution + pilot readiness report

**Precedence:** 15  
**Depends on:** SF-014

Tasks:

- Execute low/medium/high workload suites.
- Collect and summarize observed bottlenecks and conflict patterns.
- Produce pilot readiness report with recommendation and migration triggers.

Definition of done:

- Report includes measured results for all workload tiers.
- Recommendation is evidence-backed and actionable.
