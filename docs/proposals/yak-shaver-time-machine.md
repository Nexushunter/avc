# Yak Shaver Time Machine

## Q&A Session: Change-Package-First Modeling

### Q1) What is the core idea of this model?

Use a `ChangePackage` as the central object. Each package bundles intent, execution summary, patch, validation evidence, and approval metadata in one portable unit.

### Q2) What are the modeling layers?

- **Package Contract Layer**: strict schema for package fields and lifecycle status.
- **Intent Layer**: the "why" and acceptance contract.
- **Execution Summary Layer**: condensed lineage rather than full event stream.
- **Patch + Evidence Layer**: diffs, test outcomes, benchmarks, security checks.
- **Release Layer**: merge, rollback token, and post-merge verification hooks.

### Q3) How does this handle multiple providers?

Provider traces are normalized into a compact summary section:

- `provider_id`
- `capabilities_used`
- `decision_points`
- `confidence/uncertainty notes`
Raw traces remain optional attachments for debugging or audits.

### Q4) What are the strongest benefits?

- Pragmatic and easy to adopt in existing Git/PR workflows.
- Lower storage and cognitive load than full event-graph models.
- Keeps review centered on a single auditable object.

### Q5) What are the main drawbacks?

- Less detail for deep forensic investigation.
- Reconstructing exact agent execution can be harder.
- Summary quality depends on adapter quality and honesty.

### Q6) What does approval look like in this model?

Reviewer signs off on package completeness and risk fit:

1. intent is clear,
2. evidence meets tier requirements,
3. rollback plan exists,
4. unresolved uncertainties are explicit.

### Q7) Best fit?

Teams that need fast adoption and useful governance without a large platform investment.

### Open Questions

- Which evidence fields should be mandatory at each risk tier?
- When should a package be escalated to full event-graph capture?