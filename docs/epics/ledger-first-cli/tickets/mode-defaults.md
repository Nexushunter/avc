# Mode Defaults

## Goal
Implement deterministic runtime mode resolution with `ledger-first` as default.

## Scope
- CLI mode flags (`--mode`, `--git-compatible`)
- config-backed default mode
- visible runtime mode in command output

## Tasks
- Define precedence: explicit flags -> config default -> built-in fallback.
- Validate mode values and error behavior.
- Emit active mode on each command invocation.

## Done When
- Mode selection is deterministic.
- Default mode is `ledger-first` with no flags.
- Help/docs and runtime output are aligned.
