# Banana Ledger of Doom

## Q&A Session: Graph-First Modeling

### Q1) What is the core idea of this model?

Treat version control as an event graph first, with commits as one output artifact. Every lifecycle action (intent creation, planning, tool call, test run, approval, merge) is an append-only event node.

### Q2) What are the modeling layers?

- **Intent Layer**: goals, constraints, acceptance criteria.
- **Execution Layer**: agent steps, provider details, tool invocations, retries.
- **Artifact Layer**: diffs, test reports, generated files, commit references.
- **Governance Layer**: policy decisions, approvals, exceptions, audit signatures.
- **Runtime Layer**: deployment and post-merge telemetry, linked back to the same lineage.

### Q3) How does this handle multiple providers?

Use a provider adapter that maps raw provider events into canonical event types (`ModelRequest`, `ToolInvocation`, `ReasoningCheckpoint`, `Failure`). Keep provider-specific fields in a namespaced payload (`provider.raw.`*) to avoid schema lock-in.

### Q4) What are the strongest benefits?

- Maximum traceability across the full lifecycle.
- Easier replay and root-cause analysis after incidents.
- Natural support for multi-agent collaboration and forks.

### Q5) What are the main drawbacks?

- Storage and indexing overhead grows quickly.
- Query complexity can become high without strict conventions.
- Requires clear retention policies for sensitive prompt/tool data.

### Q6) What does approval look like in this model?

Approval is an event edge from reviewer identity to a specific graph checkpoint, not just a PR status bit. This allows "approved for low-risk files only" or "approved with mandatory post-merge monitor."

### Q7) Best fit?

Teams that value forensic-level lineage, compliance-heavy workflows, and long-lived agent automation.

### Open Questions

- Should lifecycle events be immutable forever, or can redaction events supersede sensitive nodes?
- How much runtime telemetry should be attached before the graph becomes too noisy?