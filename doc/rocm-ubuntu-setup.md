# ROCm Setup on Ubuntu 24.04 (Kernel 6.8)

## System Requirements

- Ubuntu 24.04 (noble) with kernel 6.8 GA
- AMD GPU (RDNA2 or newer recommended)
- User in `video` and `render` groups

## Installation

### 1. Prerequisites

```bash
sudo apt install linux-headers-$(uname -r) linux-modules-extra-$(uname -r)
sudo apt install python3-setuptools python3-wheel
sudo apt install libzstd-dev mesa-common-dev python3-dev zlib1g-dev  # prevent dep issues
sudo apt install cmake ninja-build git
```

### 2. User groups

```bash
sudo usermod -a -G video,render $USER
# Reboot or log out/in for group changes to take effect
```

### 3. Install ROCm

```bash
sudo apt update

# Download installer (example for ROCm 6.3.1)
wget https://repo.radeon.com/amdgpu-install/6.3.1/ubuntu/noble/amdgpu-install_6.3.60301-1_all.deb
sudo apt install ./amdgpu-install_6.3.60301-1_all.deb
sudo apt update

# Full development stack
sudo amdgpu-install --usecase=rocmdev

# Or runtime only
sudo amdgpu-install --usecase=rocm
```

### 4. Library paths

```bash
sudo tee /etc/ld.so.conf.d/rocm.conf <<EOF
/opt/rocm/lib
/opt/rocm/lib64
EOF
sudo ldconfig
```

### 5. Shell environment (~/.bashrc)

```bash
# ROCm paths
export PATH=$PATH:/opt/rocm/bin
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/opt/rocm/lib:/opt/rocm/lib64

# RX 6600 override (gfx1032 → gfx1030)
export HSA_OVERRIDE_GFX_VERSION=10.3.0
export ROCR_VISIBLE_DEVICES=0
export HIP_VISIBLE_DEVICES=0
```

### 6. Verification

```bash
sudo dkms status          # amdgpu DKMS module installed
rocminfo                  # lists GPU agents
rocminfo | grep gfx       # shows gfx arch
rocm-smi                  # GPU stats
/opt/rocm/bin/hipconfig --version  # ROCm version
```

## Known Issues

### Kernel updates break ROCm

Ubuntu kernel micro-updates (e.g., 6.8.0-41 → 6.8.0-44) can break ROCm.

Fix: Boot previous kernel from GRUB, pin it if needed.

### DKMS fails on newer kernels

The `amdgpu-dkms` package does not build on kernels 6.13 or 6.14. Stay on 6.8 GA.

### Dependency conflicts during install

ROCm installation can fail with unmet dependencies for `libzstd-dev`, `mesa-common-dev`, etc.

Fix: Install them manually before running `amdgpu-install` (included in prerequisites above).

### Secure Boot

Either:
- Enable Secure Boot support during DKMS installation (sign the module), or
- Disable Secure Boot in BIOS before installing

### BIOS: disable iGPU

If you have an integrated GPU alongside the discrete AMD card, disable the iGPU in BIOS to avoid conflicts with ROCm device enumeration.

## Packages Summary

| Package | Purpose |
|---------|---------|
| `linux-headers-$(uname -r)` | Kernel headers for DKMS |
| `linux-modules-extra-$(uname -r)` | Extra kernel modules (amdgpu) |
| `amdgpu-install` | ROCm meta-installer |
| `rocm-dev` | ROCm development libraries |
| `rocm-smi` | GPU monitoring |
| `hip-runtime-amd` | HIP runtime |
| `hipblas` | HIP BLAS library |

## Sources

- [ROCm System Requirements](https://rocm.docs.amd.com/projects/install-on-linux/en/latest/reference/system-requirements.html)
- [ROCm Installation Prerequisites](https://rocm.docs.amd.com/projects/install-on-linux/en/latest/install/prerequisites.html)
- [ROCm AMDGPU Installer for Ubuntu](https://rocm.docs.amd.com/projects/install-on-linux/en/docs-6.3.1/install/install-methods/amdgpu-installer/amdgpu-installer-ubuntu.html)
- [ROCm Issue #3662 — Ubuntu 24.04 incompatibility](https://github.com/ROCm/ROCm/issues/3662)
- [ROCm Issue #4619 — DKMS on kernel 6.13/6.14](https://github.com/ROCm/ROCm/issues/4619)
- [ROCm Issue #3701 — amdgpu-dkms on kernel 6.8.0-44](https://github.com/ROCm/ROCm/issues/3701)
- [nktice/AMD-AI — ROCm setup for Ubuntu 24.04](https://github.com/nktice/AMD-AI)
- [psygreg/rocm-ubuntu — Automated RDNA2/3 installer](https://github.com/psygreg/rocm-ubuntu)
