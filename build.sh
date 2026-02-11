#!/usr/bin/env bash
set -euo pipefail

# ── Config ───────────────────────────────────────────────────────────
SD_CPP_DIR="${SD_CPP_DIR:-$HOME/Projects/stable-diffusion.cpp}"
BUILD_DIR="$SD_CPP_DIR/build"
# Always target gfx1030 for RX 6600 (gfx1032) — ROCm libraries only
# ship gfx1030 kernels. Use HSA_OVERRIDE_GFX_VERSION=10.3.0 at runtime.
# See doc/rocm-rx6600.md for details.
GPU_TARGET="${AMDGPU_TARGET:-gfx1030}"
JOBS="${JOBS:-$(nproc)}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ── Helpers ──────────────────────────────────────────────────────────
red()   { printf '\033[1;31m%s\033[0m\n' "$*"; }
green() { printf '\033[1;32m%s\033[0m\n' "$*"; }
cyan()  { printf '\033[1;36m%s\033[0m\n' "$*"; }
info()  { cyan "=> $*"; }
ok()    { green "=> $*"; }
die()   { red "ERROR: $*" >&2; exit 1; }

# ── Preflight checks ────────────────────────────────────────────────
[ -d "$SD_CPP_DIR" ]              || die "stable-diffusion.cpp not found at $SD_CPP_DIR"
[ -f "$SD_CPP_DIR/CMakeLists.txt" ] || die "no CMakeLists.txt in $SD_CPP_DIR"
command -v cmake &>/dev/null      || die "cmake not found"
command -v make  &>/dev/null      || die "make not found"

info "stable-diffusion.cpp: $SD_CPP_DIR"
info "GPU target: $GPU_TARGET"
info "Build jobs: $JOBS"

# ── Clean build directory ────────────────────────────────────────────
if [ "${1:-}" = "--clean" ] || [ "${1:-}" = "-c" ]; then
    info "Cleaning build directory..."
    rm -rf "$BUILD_DIR"
fi

mkdir -p "$BUILD_DIR"

# ── Configure ────────────────────────────────────────────────────────
info "Configuring with SD_HIPBLAS=ON, AMDGPU_TARGETS=$GPU_TARGET..."
cmake -S "$SD_CPP_DIR" -B "$BUILD_DIR" \
    -DSD_HIPBLAS=ON \
    -DAMDGPU_TARGETS="$GPU_TARGET" \
    -DGPU_TARGETS="$GPU_TARGET" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_POSITION_INDEPENDENT_CODE=ON

# ── Build ────────────────────────────────────────────────────────────
info "Building with $JOBS jobs..."
cmake --build "$BUILD_DIR" -j"$JOBS"

# ── Verify sd-cli ────────────────────────────────────────────────────
SD_CLI="$BUILD_DIR/bin/sd-cli"
[ -x "$SD_CLI" ] || die "sd-cli binary not found after build"
ok "sd-cli built: $SD_CLI ($(du -h "$SD_CLI" | cut -f1))"

# ── Install sdx with embedded binary ────────────────────────────────
if [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
    info "Installing sdx with embedded sd-cli..."
    source "$HOME/.cargo/env" 2>/dev/null || true
    cargo install --path "$SCRIPT_DIR" --force
    ok "sdx installed: $(which sdx 2>/dev/null || echo ~/.cargo/bin/sdx)"
fi

ok "Done! Remember to set: export HSA_OVERRIDE_GFX_VERSION=10.3.0"
