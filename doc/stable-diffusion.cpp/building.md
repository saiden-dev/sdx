# Building stable-diffusion.cpp with ROCm/HIP

## Prerequisites

```bash
# System packages
sudo apt install cmake ninja-build git
sudo apt install linux-headers-$(uname -r) linux-modules-extra-$(uname -r)

# ROCm (6.2+ for Ubuntu 24.04)
# See https://rocm.docs.amd.com/projects/install-on-linux/
sudo amdgpu-install --usecase=rocmdev

# Verify
rocminfo | grep gfx
rocm-smi
```

## Build for RX 6600 (gfx1032 → target gfx1030)

**Always target `gfx1030`**, not `gfx1032`. See [rocm-rx6600.md](../rocm-rx6600.md) for why.

### Quick build

```bash
cd ~/Projects/stable-diffusion.cpp
mkdir -p build && cd build

cmake .. \
  -DSD_HIPBLAS=ON \
  -DAMDGPU_TARGETS=gfx1030 \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_POSITION_INDEPENDENT_CODE=ON

make -j$(nproc)
```

### Using the sdx build script

```bash
# From the sdx repo root
./build.sh           # configure + build + install sdx
./build.sh --clean   # wipe build dir first, then build
```

Environment overrides:
- `SD_CPP_DIR` — path to stable-diffusion.cpp (default: `~/Projects/stable-diffusion.cpp`)
- `AMDGPU_TARGET` — GPU target (default: `gfx1030`)
- `JOBS` — parallel build jobs (default: `$(nproc)`)

### Running after build

```bash
# REQUIRED: set the GFX override
export HSA_OVERRIDE_GFX_VERSION=10.3.0

# Direct
./build/bin/sd-cli -m model.safetensors -p "a cat" -o output.png

# Via sdx (sd-cli is embedded in the binary)
sdx generate --model dreamshaper -p "a cat" -o output.png
```

## CMake Flags Reference

### Backend selection (pick one)

| Flag | Backend | Notes |
|------|---------|-------|
| `-DSD_HIPBLAS=ON` | ROCm/HIP | AMD GPUs, requires ROCm |
| `-DSD_VULKAN=ON` | Vulkan | Any GPU with Vulkan 1.3+, no ROCm needed |
| `-DSD_CUBLAS=ON` | CUDA | NVIDIA GPUs |
| (none) | CPU only | Slowest, always works |

### GPU architecture targets

| Flag | Purpose |
|------|---------|
| `-DAMDGPU_TARGETS=gfx1030` | AMD GPU architecture for HIP |
| `-DGPU_TARGETS=gfx1030` | Alternative/additional arch flag |

Multiple targets: `-DAMDGPU_TARGETS="gfx1030;gfx1100"` (semicolon-separated).

### Build options

| Flag | Default | Purpose |
|------|---------|---------|
| `-DCMAKE_BUILD_TYPE=Release` | Debug | Optimization level |
| `-DCMAKE_POSITION_INDEPENDENT_CODE=ON` | OFF | Fix PIE linking errors on some systems |
| `-DCMAKE_C_COMPILER=clang` | cc | Use ROCm clang (optional but recommended) |
| `-DCMAKE_CXX_COMPILER=clang++` | c++ | Use ROCm clang++ |
| `-G Ninja` | Makefiles | Use Ninja build system (faster) |

## Vulkan Alternative

If ROCm is too much trouble, Vulkan works without any ROCm installation:

```bash
cmake .. -DSD_VULKAN=ON -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)

# No HSA_OVERRIDE needed
./build/bin/sd-cli -m model.safetensors -p "a cat" -o output.png
```

Pros:
- No ROCm needed, no override env vars
- Works on any GPU with Vulkan 1.3+ (RX 6600 has excellent RADV support)
- Simpler setup

Cons:
- Potentially slower than HIP for some operations
- Less mature for compute workloads

## Known Build Issues

### PIE linking failure

```
relocation R_X86_64_32 against `.rodata.str1.1' can not be used when making a PIE object
```

Fix: Add `-DCMAKE_POSITION_INDEPENDENT_CODE=ON` to cmake.

### Wrong compiler version

If system clang doesn't match ROCm's expected version, use ROCm's compilers:

```bash
cmake .. \
  -DCMAKE_C_COMPILER=/opt/rocm/bin/clang \
  -DCMAKE_CXX_COMPILER=/opt/rocm/bin/clang++ \
  -DSD_HIPBLAS=ON \
  ...
```

### Stale build cache

If switching between GPU targets or backends, always clean first:

```bash
rm -rf build && mkdir build && cd build
```

Or use `./build.sh --clean`.

## Output Binaries

After building:
- `build/bin/sd-cli` — CLI image generation tool
- `build/bin/sd-server` — HTTP server (AUTOMATIC1111 + OpenAI compatible API)

## Embedding in sdx

The sdx `build.rs` copies `sd-cli` from the build directory into the Rust binary at compile time via `include_bytes!`. When sdx runs without a configured `sd_cli_path`, it extracts the embedded binary to `~/.cache/sdx/sd-cli`.

The source path defaults to `~/Projects/stable-diffusion.cpp/build/bin/sd-cli` and can be overridden with the `SD_CLI_BIN` env var during `cargo install`:

```bash
SD_CLI_BIN=/path/to/sd-cli cargo install --path .
```

## Sources

- [stable-diffusion.cpp README](https://github.com/leejet/stable-diffusion.cpp)
- [stable-diffusion.cpp Issue #292 — ROCm on AMD](https://github.com/leejet/stable-diffusion.cpp/issues/292)
- [stable-diffusion.cpp Issue #320 — PIE linking](https://github.com/leejet/stable-diffusion.cpp/issues/320)
- [DeepWiki — stable-diffusion.cpp building](https://deepwiki.com/leejet/stable-diffusion.cpp/4-building-and-installation)
