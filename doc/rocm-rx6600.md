# ROCm on AMD Radeon RX 6600 (gfx1032)

## Hardware

- **GPU**: AMD Radeon RX 6600 (Navi 23, RDNA2)
- **Architecture**: gfx1032
- **VRAM**: 8 GB GDDR6
- **Compute Units**: 28
- **CPU**: Intel i5-11400F
- **OS**: Ubuntu (kernel 6.8.0-100-generic)
- **ROCm**: 6.3.42134

## Support Status

gfx1032 is **not officially supported** by ROCm's HIP SDK. It exists in a gray area:

| Feature           | gfx1030 (RX 6800/6900) | gfx1032 (RX 6600) |
|-------------------|-------------------------|--------------------|
| Runtime detection | Supported               | Supported          |
| HIP SDK           | Supported               | **Not supported**  |
| rocBLAS           | Optimized kernels       | **Fallback only**  |

The officially supported RDNA2 target is `gfx1030` (Navi 21 — RX 6800, 6900, PRO W6800). Consumer RDNA2 cards are not first-class citizens.

However, gfx1030, gfx1031, and gfx1032 all share the **same RDNA2 ISA**. The differences are hardware config (CU count, memory bus, cache) not instruction support. This means gfx1030 code runs correctly on gfx1032.

## The Override Workaround

The universal fix is to tell HSA to report your GPU as gfx1030:

```bash
export HSA_OVERRIDE_GFX_VERSION=10.3.0
```

Format: `{MAJOR}.{MINOR}.{PATCH}` where gfx1030 = 10.3.0.

**This must be set in every context where ROCm is used** — shell sessions, systemd services, Docker containers, cron jobs. It is not persistent across reboots unless added to shell profile.

### Validated with this override

- PyTorch training (MNIST, 99% accuracy)
- Ollama LLM inference
- Stable Diffusion image generation via stable-diffusion.cpp
- llama.cpp inference

## Build Rules for gfx1032

**Always build for `gfx1030`, never `gfx1032`.** Reasoning:

1. If you build for `gfx1032`, your code has gfx1032 code objects
2. But ROCm's rocBLAS/hipBLAS only ship gfx1030 kernels
3. At runtime, even with the override, the linked libraries won't match
4. Result: `hipErrorNoBinaryForGpu` or `invalid device function`

Building for `gfx1030` + override = both your code AND ROCm libraries use gfx1030 = everything works.

## Environment Variables

| Variable                    | Value     | Purpose                                          |
|-----------------------------|-----------|--------------------------------------------------|
| `HSA_OVERRIDE_GFX_VERSION`  | `10.3.0`  | Override GPU arch to gfx1030 (**required**)      |
| `ROCR_VISIBLE_DEVICES`      | `0`       | Select which GPU ROCr sees (multi-GPU systems)   |
| `HIP_VISIBLE_DEVICES`       | `0`       | Select which GPU HIP sees                        |
| `ROCM_PATH`                 | `/opt/rocm` | Ensure ROCm install is found                   |
| `HSA_ENABLE_SDMA`           | `0`       | Disable SDMA if experiencing hangs (rarely needed) |

### Shell profile (~/.bashrc)

```bash
export HSA_OVERRIDE_GFX_VERSION=10.3.0
export ROCR_VISIBLE_DEVICES=0
export HIP_VISIBLE_DEVICES=0
export PATH=$PATH:/opt/rocm/bin
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/opt/rocm/lib:/opt/rocm/lib64
```

### Systemd services

```ini
[Service]
Environment="HSA_OVERRIDE_GFX_VERSION=10.3.0"
Environment="ROCR_VISIBLE_DEVICES=0"
```

## Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `CUDA error: invalid device function` | Kernels not compiled for GPU arch, or override not set | Set `HSA_OVERRIDE_GFX_VERSION=10.3.0`, build for `gfx1030` |
| `hipErrorNoBinaryForGpu` | Same as above | Same fix |
| `ROCk module is NOT loaded` | amdgpu kernel module not loaded | Check `sudo dkms status`, reinstall `amdgpu-dkms` |
| PIE linking error (`relocation R_X86_64_32 against .rodata`) | Missing PIC flag | Add `-DCMAKE_POSITION_INDEPENDENT_CODE=ON` |
| Segfault with multi-GPU | Known ggml issue | Use `ROCR_VISIBLE_DEVICES=0` |
| VRAM shows 0% usage | Wrong device index | Check `rocminfo` output, update device index |

## VRAM Considerations

8 GB is sufficient for:
- SD 1.5 models (~2 GB VRAM)
- SDXL with offloading (`--offload-to-cpu --vae-tiling`)
- Small-medium LLMs (7B quantized)

Not sufficient for:
- Large LLMs (13B+ without heavy quantization)
- Multiple models loaded simultaneously

## Verification Commands

```bash
# Check GPU detection
rocminfo | grep gfx

# Check ROCm version
/opt/rocm/bin/hipconfig --version

# Check DKMS module
sudo dkms status

# GPU stats
rocm-smi

# Test HIP with override
HSA_OVERRIDE_GFX_VERSION=10.3.0 rocminfo
```

## Ubuntu 24.04 Specific Notes

- ROCm 6.2+ supports Ubuntu 24.04 (noble)
- Kernel 6.8 GA is fully supported; avoid HWE kernels (6.13/6.14 break DKMS)
- Kernel micro-updates (6.8.0-41 → 6.8.0-44) can occasionally break ROCm
- If ROCm breaks after kernel update, boot previous kernel from GRUB
- User must be in `video` and `render` groups: `sudo usermod -a -G video,render $USER`

## Sources

- [ROCm 6.3.3 GPU Architecture Specs](https://rocm.docs.amd.com/en/docs-6.3.3/reference/gpu-arch-specs.html)
- [ROCm Issue #1797 — gfx1032 override](https://github.com/ROCm/ROCm/issues/1797)
- [ROCm Issue #1698 — RX 6600 XT support](https://github.com/ROCm/ROCm/issues/1698)
- [ROCm Issue #5069 — per-GPU GFX overrides](https://github.com/ROCm/ROCm/issues/5069)
- [stable-diffusion.cpp Issue #292 — ROCm on RX 6600](https://github.com/leejet/stable-diffusion.cpp/issues/292)
- [Ollama with RX 6600 XT](https://major.io/p/ollama-with-amd-radeon-6600xt/)
- [Step-by-step ROCm on Ubuntu with RX 6600](https://gist.github.com/furaar/ee05a5ef673302a8e653863b6eaedc90)
