# Building sd-cli for macOS (Apple Silicon)

## Prerequisites

- macOS with Apple Silicon (M1/M2/M3/M4)
- Xcode Command Line Tools: `xcode-select --install`
- CMake: `brew install cmake`

## Build sd-cli

```bash
git clone https://github.com/leejet/stable-diffusion.cpp
cd stable-diffusion.cpp
git submodule update --init --recursive
cmake -B build -DSD_METAL=ON
cmake --build build --config Release
```

## Upload to GitHub release

```bash
cp build/bin/sd-cli sd-cli-macos-aarch64
gh release upload sd-cli-v0.1.0 sd-cli-macos-aarch64 --repo saiden-dev/sdx
```

## Verify

```bash
gh release view sd-cli-v0.1.0 --repo saiden-dev/sdx --json assets --jq '.assets[].name'
```

Expected output:

```
sd-cli-linux-x86_64
sd-cli-macos-aarch64
```
