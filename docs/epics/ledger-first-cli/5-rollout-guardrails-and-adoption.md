# 5) Rollout Guardrails and Adoption

## Objective

Roll out ledger-first CLI safely with clear fallback controls, measurable adoption signals, and a concrete Phase 3 readiness decision.

## Scope

- feature flags and fallback behavior
- transition adoption/reliability metrics
- Phase 2 report and recommendation

## Deliverables

- rollout controls for progressive enablement
- adoption and reliability metric dashboard/report inputs
- formal Phase 2 readiness report for Phase 3 entry decision

## Detailed Work Plan

### A. Rollout Controls

- Add feature flags for ledger-first defaults and compatibility emissions.
- Define fallback mode behavior and operator runbook.
- Ensure safe rollback to compatibility-first behavior if needed.

### B. Metrics and Evaluation

- Track adoption and reliability signals:
  - command success/failure rates by mode;
  - reconciliation drift frequency;
  - reviewer completion time and confidence;
  - merge readiness false-positive/false-negative rates.
- Define thresholds for acceptable transition health.

### C. Readiness and Decision

- Produce transition report summarizing:
  - delivered capabilities,
  - known constraints,
  - operational metrics,
  - unresolved risks.
- Recommend continue/hold/rollback and Phase 3 entry timing.

## Acceptance Criteria

- Rollout can be enabled/disabled safely via documented controls.
- Transition metrics are captured for representative team workflows.
- Phase 2 recommendation is evidence-backed and decision-ready.

## Dependencies

- `3-ledger-git-id-mapping-and-sync.md`
- `4-review-and-ci-integration-flow.md`

## Risks and Mitigations

- **Risk:** rollout toggles are unclear or inconsistent.
  - **Mitigation:** single source of truth config + runbook.
- **Risk:** adoption metrics miss human workflow pain.
  - **Mitigation:** combine telemetry with structured reviewer feedback.

## Tickets (Ordered)

### LF-013 Feature flags, rollout controls, and fallback mode

**Precedence:** 13  
**Depends on:** LF-009, LF-012

Tasks:

- Implement flag-driven rollout and fallback logic.
- Document operational procedures for enable/disable and rollback.
- Validate fallback behavior in integration scenarios.

Definition of done:

- Rollout and rollback paths are controlled, tested, and documented.

### LF-014 Adoption and reliability metrics for transition

**Precedence:** 14  
**Depends on:** LF-013

Tasks:

- Implement metrics capture for adoption/reliability indicators.
- Build summary artifacts for periodic rollout review.
- Validate metric quality and signal usefulness.

Definition of done:

- Metrics are available and trustworthy for transition decisions.

### LF-015 Phase 2 readiness report and Phase 3 entry recommendation

**Precedence:** 15  
**Depends on:** LF-014

Tasks:

- Compile readiness report with findings and risk posture.
- Make recommendation for Phase 3 entry criteria/timing.
- Document required follow-on work for unresolved gaps.

Definition of done:

- Report is complete, actionable, and supports go/no-go decision.
