# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

sdx — a Rust CLI/server wrapping `sd-cli` from stable-diffusion.cpp (edition 2024).

Two modes:
- `sdx generate` — CLI txt2img calling sd-cli as subprocess
- `sdx serve` — OpenAI-compatible HTTP API that calls sd-cli per request

Model selection via TOML config at `~/.config/sdx/config.toml`.

## Build Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Single test: `cargo test <test_name>`
- Check (fast compile check): `cargo check`
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Coverage: `cargo tarpaulin`

## Architecture

- `src/main.rs` — thin CLI wrapper, delegates to lib
- `src/lib.rs` — re-exports modules
- `src/error.rs` — `thiserror` error enum
- `src/config.rs` — TOML config loading, `AppConfig` + `ModelConfig`
- `src/sd_cli.rs` — `GenerateArgs` builder, subprocess spawning
- `src/cli.rs` — clap subcommands (generate, serve, models)
- `src/api.rs` — OpenAI-compatible request/response types
- `src/server.rs` — axum HTTP server, routes, GPU mutex

## Rust Coding Rules

### Code Limits
- File: 300-500 lines max | Function: 50 lines max | Line: 100-120 chars (80 for comments)
- Parameters: 5 max per function | Nesting: 3 max, use early returns

### Naming
- Predicates: `is_empty()`, `has_permissions()`, `can_connect()`
- Mutating: imperative verbs (`clear()`, `save()`)
- Constructors: `new()`, `from_*()`, `parse_*()`
- No redundant prefixes (use `Config` not `SdCliConfig` inside `sd_cli` module)

### Module Exports
Keep submodules private, re-export types at module level via `lib.rs`.

### Style
- Flat over nested: early returns, avoid deep nesting
- Iterators over loops: functional chains preferred
- Method chaining: builder pattern for fluent APIs
- Explicit over implicit: no magic booleans, use enums/structs
- Sensible defaults: `Default` trait

### Import Order
1. std → 2. external crates → 3. crate → 4. super/self

### Forbidden
- `.unwrap()` in library code (use `.expect()`, `unwrap_or*`, or `?`)
- `panic!()` for recoverable errors
- Wildcard imports (except `use super::*` in tests)
- `dbg!()`, `todo!()` in committed code
- Magic numbers, hardcoded URLs, silent failures

### Error Handling
- `Result<T>` consistently, not `Option<T>` for errors
- `thiserror` for custom error types
- Always propagate with context via `?`

### Architecture Patterns
- Library-first: `lib.rs` reuse, `main.rs` is thin wrapper
- Service returns data: never print in service layer
- Two-level run pattern: `run()` creates deps, `run_with()` is testable

### Testing
- Write unit → write test → repeat (no test debt)
- Mock boundaries: file system, HTTP, external processes
- Inline `#[cfg(test)]` for private fn tests; separate `tests.rs` for module tests
- Exclude integration wrappers with `#[cfg(not(tarpaulin_include))]`

### Before Finishing
```bash
cargo fmt && cargo clippy && cargo test
```
