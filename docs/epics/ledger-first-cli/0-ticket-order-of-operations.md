# Ledger-First CLI Ticket Order

## Purpose

This file defines precedence across Phase 2 tickets using diagram-first planning.

## Execution Sequence

```mermaid
flowchart TD
  subgraph wave0 [Wave0]
    lf001[LF-001]
  end
  subgraph wave1 [Wave1]
    lf002[LF-002]
  end
  subgraph wave2 [Wave2]
    lf003[LF-003]
    lf005[LF-005]
  end
  subgraph wave3 [Wave3]
    lf004[LF-004]
    lf006[LF-006]
  end
  subgraph wave4 [Wave4]
    lf007[LF-007]
    lf010[LF-010]
  end
  subgraph wave5 [Wave5]
    lf008[LF-008]
    lf011[LF-011]
  end
  subgraph wave6 [Wave6]
    lf009[LF-009]
    lf012[LF-012]
  end
  subgraph wave7 [Wave7]
    lf013[LF-013]
  end
  subgraph wave8 [Wave8]
    lf014[LF-014]
  end
  subgraph wave9 [Wave9]
    lf015[LF-015]
  end

  wave0 --> wave1 --> wave2 --> wave3 --> wave4 --> wave5 --> wave6 --> wave7 --> wave8 --> wave9
```



## Dependency Graph

```mermaid
flowchart LR
  subgraph cliSurface [CliSurface]
    lf001[LF-001]
    lf002[LF-002]
    lf003[LF-003]
  end

  subgraph gitCompat [GitCompatibility]
    lf004[LF-004]
    lf005[LF-005]
    lf006[LF-006]
  end

  subgraph mapping [MappingSync]
    lf007[LF-007]
    lf008[LF-008]
    lf009[LF-009]
  end

  subgraph integrations [ReviewAndCI]
    lf010[LF-010]
    lf011[LF-011]
    lf012[LF-012]
  end

  subgraph rollout [Rollout]
    lf013[LF-013]
    lf014[LF-014]
    lf015[LF-015]
  end

  lf001 --> lf002 --> lf003
  lf002 --> lf004
  lf001 --> lf005
  lf004 --> lf006
  lf005 --> lf006
  lf003 --> lf007
  lf006 --> lf007
  lf007 --> lf008 --> lf009
  lf003 --> lf010
  lf007 --> lf011
  lf010 --> lf011 --> lf012
  lf009 --> lf013
  lf012 --> lf013
  lf013 --> lf014 --> lf015
```



## Ticket Legend

- `LF-001` Ledger-first command defaults and mode toggles
- `LF-002` Command orchestration runtime and shared execution context
- `LF-003` CLI output contracts for ledger-first operations
- `LF-004` Git artifact emitter for branch/commit generation
- `LF-005` PR metadata emitter and compatibility annotations
- `LF-006` Deterministic Git reconciliation checks
- `LF-007` Mapping index schema (`package`, `event`, `commit`, `pr`)
- `LF-008` Bidirectional lookup commands and APIs
- `LF-009` Drift detection and mapping repair flow
- `LF-010` Review-gate integration in CLI and compatibility outputs
- `LF-011` CI/check status ingestion and merge-readiness synthesis
- `LF-012` End-to-end review + merge compatibility flow validation
- `LF-013` Feature flags, rollout controls, and fallback mode
- `LF-014` Adoption and reliability metrics for transition
- `LF-015` Phase 2 readiness report and Phase 3 entry recommendation

