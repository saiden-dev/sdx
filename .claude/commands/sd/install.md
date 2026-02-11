---
description: Cargo install simple-diffusion globally
allowed-tools: Bash(cargo install:*)
---

Run `source "$HOME/.cargo/env" 2>/dev/null; cargo install --path .` from the project root to install simple-diffusion globally.

If the install succeeds, verify by running `simple-diffusion --version` and show the output.
If the install fails, show the error and suggest a fix.
