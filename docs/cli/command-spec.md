# AVC CLI Command Spec

## Scope

This document specifies command-line behavior for:

- `avc plan`
- `avc run`
- `avc approve`
- `avc merge`

It targets the Git-compatible pilot while preserving a ledger-first model.

## Conventions

- Required argument notation: `<value>`
- Optional argument notation: `[value]`
- Repeated flags: may be passed multiple times
- Output defaults to human-readable text unless `--output json` is provided
- AVC references are agent-agnostic: no provider/vendor is treated as implicit default.

## Default Resolution Policy

When optional flags are omitted, defaults resolve in this order:

1. explicit CLI flag value;
2. repo config value from `.avc/config.json` (if defined for that field);
3. built-in command default from this spec.

If no default exists in any layer, behavior is:

- scalar fields: `null`/unset;
- repeatable fields: empty list;
- booleans: `false`.

## Global Flags


| Flag                | Type                 | Default           | Description                    |
| ------------------- | -------------------- | ----------------- | ------------------------------ |
| `--repo <path>`     | string               | current directory | Repository root                |
| `--output <format>` | enum (`text`,`json`) | `text`            | Output format                  |
| `--no-color`        | boolean              | `false`           | Disable colored output         |
| `--verbose`         | boolean              | `false`           | Include debug details          |
| `--trace-id <id>`   | string               | auto-generated    | Correlation id for logs/events |


## Security-Level Behavior

Before writing any event, the CLI reads `.avc/config.json.securityLevel`.

Allowed values:

- `full`: full payload persistence
- `redacted`: sensitive fields removed/masked before persistence
- `summary_only`: only minimal safe metadata persisted

Failure mode:

- if `.avc/config.json` is missing, unreadable, or has invalid `securityLevel`, command exits with a fail-closed security config error.

## Command: `avc plan`

Create a change package and initialize lifecycle state.

### Usage

```shell
avc plan --title <title> [flags]
```

### Flags


| Flag                      | Type                         | Required | Description                                  |
| ------------------------- | ---------------------------- | -------- | -------------------------------------------- |
| `--title <text>`          | string                       | yes      | Human-readable intent title                  |
| `--goal <text>`           | string                       | no       | Primary objective                            |
| `--constraint <text>`     | string (repeatable)          | no       | Constraints (policy, scope, time)            |
| `--acceptance <text>`     | string (repeatable)          | no       | Acceptance criteria                          |
| `--risk <tier>`           | enum (`low`,`medium`,`high`) | no       | Initial risk tier                            |
| `--branch <name>`         | string                       | no       | Preferred branch name in Git-compatible mode |
| `--idempotency-key <key>` | string                       | no       | De-duplicate repeated invocations            |


### Default behavior when omitted

- `--goal`: defaults to the same value as `--title`.
- `--constraint`: defaults to empty list.
- `--acceptance`: defaults to empty list.
- `--risk`: defaults to `medium`.
- `--branch`: defaults to `cursor/<slug-of-title>`.
- `--idempotency-key`: defaults to auto-generated deterministic key from intent payload.

### Example

```shell
avc plan \
  --title "Add provider-agnostic adapter contract" \
  --goal "Unify event mapping across providers" \
  --constraint "No breaking schema changes" \
  --acceptance "All provider adapters emit canonical event types" \
  --risk medium
```

### Success Output (text)

```text
Created change package: cp_123
Lifecycle: planned
Sidecar: .avc/packages/cp_123/
```

## Command: `avc run`

Execute one or more agent workflows for an existing change package.

### Usage

```shell
avc run --package <id> [flags]
```

### Flags


| Flag                | Type                | Required | Description                           |
| ------------------- | ------------------- | -------- | ------------------------------------- |
| `--package <id>`    | string              | yes      | Change package id                     |
| `--agent <name>`    | string              | no       | Agent profile identifier              |
| `--provider <name>` | string              | no       | Provider identifier                   |
| `--model <name>`    | string              | no       | Model selection                       |
| `--parallel <n>`    | integer             | no       | Concurrent runs for exploration       |
| `--max-steps <n>`   | integer             | no       | Execution step cap                    |
| `--tool <name>`     | string (repeatable) | no       | Allowlisted tools                     |
| `--validate`        | boolean             | no       | Run validation checks after execution |
| `--dry-run`         | boolean             | no       | Simulate without file modifications   |


### Default behavior when omitted

- `--agent`: defaults to configured runtime agent profile for the current repo/environment.
- `--provider`: defaults to configured provider adapter for the current repo/environment.
- `--model`: defaults to the selected provider adapter's configured default model.
- `--parallel`: defaults to `1`.
- `--max-steps`: defaults to `50`.
- `--tool`: defaults to policy-allowed tool set from config; if none defined, no additional tool restriction beyond built-ins.
- `--validate`: defaults to `false`.
- `--dry-run`: defaults to `false`.

### Example

```shell
avc run \
  --package cp_123 \
  --agent implementation-agent \
  --provider provider-adapter-a \
  --model default-capability-model \
  --validate
```

### Success Output (text)

```text
Package: cp_123
Execution: completed
Lifecycle: proposed
Validation: passed
```

## Command: `avc approve`

Apply policy checks and record human approval decisions.

### Usage

```shell
avc approve --package <id> --reviewer <id> [flags]
```

### Flags


| Flag                     | Type                | Required | Description                                  |
| ------------------------ | ------------------- | -------- | -------------------------------------------- |
| `--package <id>`         | string              | yes      | Change package id                            |
| `--reviewer <id>`        | string              | yes      | Reviewer identity                            |
| `--scope <pattern>`      | string (repeatable) | no       | Approval scope (path/domain)                 |
| `--condition <text>`     | string (repeatable) | no       | Required condition for merge                 |
| `--require-check <name>` | string (repeatable) | no       | Named gate that must pass                    |
| `--defer`                | boolean             | no       | Record deferred decision instead of approval |
| `--note <text>`          | string              | no       | Freeform reviewer rationale                  |


### Default behavior when omitted

- `--scope`: defaults to full package scope.
- `--condition`: defaults to empty list.
- `--require-check`: defaults to policy-required checks for package risk tier.
- `--defer`: defaults to `false`.
- `--note`: defaults to empty string.

### Example

```shell
avc approve \
  --package cp_123 \
  --reviewer alice \
  --scope "auth/**" \
  --condition "Monitor error-rate for 30m post-merge" \
  --require-check "security-scan"
```

### Success Output (text)

```text
Package: cp_123
Approval: granted
Lifecycle: approved
Scope: auth/**
```

## Command: `avc merge`

Merge an approved change package and emit final references.

### Usage

```shell
avc merge --package <id> [flags]
```

### Flags


| Flag                     | Type                         | Required | Description                          |
| ------------------------ | ---------------------------- | -------- | ------------------------------------ |
| `--package <id>`         | string                       | yes      | Change package id                    |
| `--strategy <name>`      | enum (`ff`,`merge`,`squash`) | no       | Merge strategy (Git-compatible mode) |
| `--target <ref>`         | string                       | no       | Target branch/ref                    |
| `--release <id>`         | string                       | no       | Optional release identifier          |
| `--attach-runtime-hooks` | boolean                      | no       | Register runtime observation hooks   |
| `--dry-run`              | boolean                      | no       | Validate merge preconditions only    |


### Default behavior when omitted

- `--strategy`: defaults to `squash`.
- `--target`: defaults to `main`.
- `--release`: defaults to unset (auto-linked if external release id is provided by pipeline).
- `--attach-runtime-hooks`: defaults to `false`.
- `--dry-run`: defaults to `false`.

### Example

```shell
avc merge \
  --package cp_123 \
  --strategy squash \
  --target main \
  --attach-runtime-hooks
```

### Success Output (text)

```text
Package: cp_123
Merge: completed
Lifecycle: merged
Commit: 24cef9b
```

## Exit Codes


| Code | Meaning                                                        |
| ---- | -------------------------------------------------------------- |
| `0`  | Success                                                        |
| `1`  | Generic runtime failure                                        |
| `2`  | Invalid command usage or argument validation failure           |
| `3`  | Config error (`.avc/config.json` missing/invalid)              |
| `4`  | Security-level enforcement failure (fail-closed write blocked) |
| `5`  | Package not found                                              |
| `6`  | Lifecycle precondition not met (invalid state transition)      |
| `7`  | Policy gate failure                                            |
| `8`  | Approval required/missing                                      |
| `9`  | Merge conflict or merge strategy failure                       |
| `10` | Provider/agent execution failure                               |
| `11` | Validation/checks failed                                       |
| `12` | Storage I/O failure (sidecar read/write/index)                 |


## Lifecycle Preconditions

- `avc plan`: no existing package required.
- `avc run`: package must exist in `planned`, `proposed`, or `deferred`.
- `avc approve`: package must be at least `proposed`.
- `avc merge`: package must be `approved` and all required checks passed.

## Sidecar Artifacts (Pilot)

- `.avc/packages/<id>/intent.json`
- `.avc/packages/<id>/events.ndjson`
- `.avc/packages/<id>/artifacts/*`
- `.avc/index/by-commit/<sha>.json`

## JSON Output Contract (Minimum)

All commands with `--output json` return:

```json
{
  "ok": true,
  "command": "plan",
  "packageId": "cp_123",
  "lifecycle": "planned",
  "references": {
    "commit": null,
    "branch": "cursor/provider-adapter-contract"
  },
  "traceId": "tr_abc123"
}
```

