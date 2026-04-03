# 4) Safety and Governance Controls

## Objective

Enforce non-negotiable safety guarantees for sidecar-based event persistence, approvals, and merge operations.

## Scope

- fail-closed behavior from `.avc/conf.json`
- immutable events and supersedes semantics
- policy gate enforcement before approve/merge
- rollback metadata requirements for high-risk changes

## Deliverables

- centralized write guard that evaluates config and security level before persistence
- immutable event append strategy with superseding-event support
- policy gate evaluation engine shared by `approve` and `merge`
- negative test suite proving unsafe paths are blocked

## Detailed Work Plan

### A. Fail-Closed Config Guard

- Build one gatekeeper function called by all write commands.
- Validate:
  - config exists,
  - `securityLevel` is allowed,
  - required policy keys are present.
- Return explicit exit codes for config/security failures.

### B. Event Immutability Enforcement

- Disallow in-place mutation of existing events.
- Implement superseding event pattern:
  - new event references superseded `eventId`,
  - original event preserved.
- Add integrity checks for append order and event references.

### C. Policy Gate Enforcement

- Define gate states: `passed`, `failed`, `missing`, `waived`.
- Require all mandatory gates before approval.
- Re-validate gate state before merge.
- Record rationale for deferred approval or blocked merge.

### D. Rollback Metadata Policy

- If risk tier is high (or configured tiers), require rollback fields before merge:
  - rollback owner,
  - rollback strategy,
  - rollback trigger criteria.
- Emit actionable error messages when missing.

## Acceptance Criteria

- Attempts to write with invalid/missing config are blocked.
- Event files remain append-only under all command paths.
- Approval and merge are blocked when mandatory gates are unmet.
- High-risk merges fail without rollback metadata.

## Dependencies

- `1-schema-storage-foundation.md`
- `2-cli-workflow-implementation.md`
- `docs/cli/config-spec.md`

## Risks and Mitigations

- **Risk:** policy logic duplicated across commands.
  - **Mitigation:** shared policy evaluation module.
- **Risk:** users bypass safeguards through manual file edits.
  - **Mitigation:** verify invariants on every command entrypoint.

## Tickets (Ordered)

### SF-012 Immutable/supersedes invariant enforcement

**Precedence:** 12  
**Depends on:** SF-003, SF-005

Tasks:

- Enforce append-only writes for event streams.
- Implement superseding-event validation (target exists, references are valid).
- Add integrity checks executed on command entry.

Definition of done:

- In-place event mutation attempts are rejected.
- Superseding events preserve full event lineage.

### SF-013 Rollback metadata + policy gate hard enforcement

**Precedence:** 13  
**Depends on:** SF-008, SF-009, SF-012

Tasks:

- Enforce rollback metadata requirements for configured risk tiers.
- Re-validate mandatory gates at approval and merge boundaries.
- Produce actionable, specific policy failure messages.

Definition of done:

- High-risk merges fail without rollback metadata.
- Approval/merge are blocked when required gates are missing or failed.
