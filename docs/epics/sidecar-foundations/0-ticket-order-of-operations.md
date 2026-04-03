# Sidecar Foundations Ticket Order

## Purpose

This file defines precedence across all Phase 1 tickets using diagrams as the primary planning format.

## Execution Sequence

```mermaid
flowchart TD
  subgraph wave0 [Wave0]
    sf001[SF-001]
  end
  subgraph wave1 [Wave1]
    sf002[SF-002]
  end
  subgraph wave2 [Wave2]
    sf003[SF-003]
    sf005[SF-005]
  end
  subgraph wave3 [Wave3]
    sf004[SF-004]
    sf006[SF-006]
    sf012[SF-012]
  end
  subgraph wave4 [Wave4]
    sf007[SF-007]
  end
  subgraph wave5 [Wave5]
    sf008[SF-008]
  end
  subgraph wave6 [Wave6]
    sf009[SF-009]
  end
  subgraph wave7 [Wave7]
    sf010[SF-010]
    sf013[SF-013]
  end
  subgraph wave8 [Wave8]
    sf011[SF-011]
  end
  subgraph wave9 [Wave9]
    sf014[SF-014]
  end
  subgraph wave10 [Wave10]
    sf015[SF-015]
  end

  wave0 --> wave1 --> wave2 --> wave3 --> wave4 --> wave5 --> wave6 --> wave7 --> wave8 --> wave9 --> wave10
```



## Dependency Graph

```mermaid
flowchart LR
  subgraph foundation [Foundation]
    sf001[SF-001]
    sf002[SF-002]
    sf003[SF-003]
    sf004[SF-004]
  end

  subgraph cliFlow [CLIFlow]
    sf005[SF-005]
    sf006[SF-006]
    sf007[SF-007]
    sf008[SF-008]
    sf009[SF-009]
  end

  subgraph reviewer [ReviewerExperience]
    sf010[SF-010]
    sf011[SF-011]
  end

  subgraph safety [SafetyGovernance]
    sf012[SF-012]
    sf013[SF-013]
  end

  subgraph observability [Observability]
    sf014[SF-014]
    sf015[SF-015]
  end

  sf001 --> sf002 --> sf003 --> sf004
  sf001 --> sf005
  sf002 --> sf005
  sf003 --> sf006
  sf006 --> sf007 --> sf008 --> sf009
  sf007 --> sf010
  sf008 --> sf010
  sf009 --> sf010
  sf010 --> sf011
  sf003 --> sf012
  sf005 --> sf012
  sf008 --> sf013
  sf009 --> sf013
  sf012 --> sf013
  sf009 --> sf014
  sf011 --> sf014
  sf013 --> sf014
  sf014 --> sf015
```



## Ticket Legend

- `SF-001` Config contract + fail-closed loader
- `SF-002` Sidecar schema v1
- `SF-003` Event append writer + ordering guarantees
- `SF-004` Commit index update + rebuild strategy
- `SF-005` CLI core context + error/exit-code mapping
- `SF-006` `avc plan` persistence flow
- `SF-007` `avc run` execution event flow
- `SF-008` `avc approve` gate and decision flow
- `SF-009` `avc merge` finalization + index update flow
- `SF-010` Reviewer read model reducers
- `SF-011` `avc status` query and output contracts
- `SF-012` Immutable/supersedes invariant enforcement
- `SF-013` Rollback metadata + policy enforcement
- `SF-014` Metrics instrumentation and collection
- `SF-015` Workload runs + pilot readiness report

