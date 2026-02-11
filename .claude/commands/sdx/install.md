---
description: Build and install sdx to /usr/local/bin
allowed-tools: Bash(cargo build:*), Bash(sudo cp:*), Bash(sudo install:*), Bash(rm:*), Bash(sdx --version), Edit
---

First, bump the pre-release version in `Cargo.toml` before installing:
- Read the current `version` field in `Cargo.toml`
- If it already has a `-pre.N` suffix, increment N (e.g. `0.1.0-pre.3` → `0.1.0-pre.4`)
- If it has no pre suffix, append `-pre.1` (e.g. `0.1.0` → `0.1.0-pre.1`)
- Use the Edit tool to update the version in `Cargo.toml`

Then build a release binary and install it:
1. Run `cargo build --release` from the project root
2. Run `sudo install -m 755 target/release/sdx /usr/local/bin/sdx`
3. Remove any stale copies (e.g. `~/.cargo/bin/sdx`) if they exist

If the install succeeds, verify by running `sdx --version` and show the output.
If the install fails, show the error and suggest a fix.
