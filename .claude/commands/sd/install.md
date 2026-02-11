---
description: Cargo install sdx globally
allowed-tools: Bash(cargo install:*), Edit
---

First, bump the pre-release version in `Cargo.toml` before installing:
- Read the current `version` field in `Cargo.toml`
- If it already has a `-pre.N` suffix, increment N (e.g. `0.1.0-pre.3` → `0.1.0-pre.4`)
- If it has no pre suffix, append `-pre.1` (e.g. `0.1.0` → `0.1.0-pre.1`)
- Use the Edit tool to update the version in `Cargo.toml`

Then run `source "$HOME/.cargo/env" 2>/dev/null; cargo install --path .` from the project root to install sdx globally.

If the install succeeds, verify by running `sdx --version` and show the output.
If the install fails, show the error and suggest a fix.
