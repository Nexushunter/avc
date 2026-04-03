# 4) Review and CI Integration Flow

## Objective

Keep reviewer and CI workflows usable during transition while AVC becomes the primary operational surface.

## Scope

- policy gate and review visibility in CLI + compatibility outputs
- CI/check status ingestion and synthesis
- end-to-end review/merge validation in mixed workflows

## Deliverables

- unified review-gate visibility model
- CI/check ingestion pipeline to merge-readiness state
- compatibility flow validation for PR-centered teams

## Detailed Work Plan

### A. Review-Gate Visibility

- Render gate states and approval scopes in CLI summaries.
- Emit equivalent compatibility annotations for PR workflows.
- Keep reviewer questions answerable from one package status view.

### B. CI/Check Ingestion

- Define check status inputs and normalization model.
- Ingest and attach check outcomes to package lifecycle state.
- Synthesize merge-readiness state based on policy requirements.

### C. End-to-End Validation

- Validate full sequence on mixed workflows:
  - CLI-first execution + PR review compatibility;
  - deferred approvals and re-checks;
  - blocked merges due to missing/failed checks.

## Acceptance Criteria

- Review and CI state are visible and consistent across CLI and compatibility outputs.
- Merge-readiness state updates correctly on check/approval changes.
- Teams relying on PR review still have complete context.

## Dependencies

- `1-cli-command-surface-and-orchestration.md`
- `3-ledger-git-id-mapping-and-sync.md`

## Risks and Mitigations

- **Risk:** CI status semantics vary across providers.
  - **Mitigation:** normalize into small canonical state model.
- **Risk:** reviewers see conflicting status across tools.
  - **Mitigation:** single merge-readiness reducer used everywhere.

## Tickets (Ordered)

### LF-010 Review-gate integration in CLI and compatibility outputs

**Precedence:** 10  
**Depends on:** LF-003

Tasks:

- Add gate and approval scope rendering to package status surfaces.
- Include same state in compatibility metadata outputs.
- Validate visibility for deferred and conditional approvals.

Definition of done:

- Reviewer gate context is complete in both CLI and PR compatibility views.

### LF-011 CI/check status ingestion and merge-readiness synthesis

**Precedence:** 11  
**Depends on:** LF-007, LF-010

Tasks:

- Implement check normalization and attachment to package state.
- Build merge-readiness reducer using policy + check + approval states.
- Add integration tests for changing check states.

Definition of done:

- Merge-readiness updates correctly as checks pass/fail/retry.

### LF-012 End-to-end review + merge compatibility flow validation

**Precedence:** 12  
**Depends on:** LF-011

Tasks:

- Execute end-to-end validation scenarios for mixed CLI/PR workflows.
- Capture mismatches and close gaps in status propagation.
- Document validated operational path for pilot users.

Definition of done:

- End-to-end review and merge flows are validated and repeatable.
