# Runtime Context

## Goal
Create a shared command runtime context used by all core commands.

## Scope
- repo root/config loading
- trace id propagation
- centralized error/exit-code mapping

## Tasks
- Define a runtime context object and initialization path.
- Move repeated command setup into shared utilities.
- Standardize command error conversion and exit code behavior.

## Done When
- `plan/run/approve/merge/status` all use shared context.
- Error handling behavior is consistent across commands.
