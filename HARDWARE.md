# Hardware Notes

## Mac Mini M4 Pro (2024) — Recommended for Local LLMs

### Top Configuration

| Spec | Max Available |
|------|---------------|
| **CPU** | M4 Pro 14-core (10P + 4E) @ 4.5GHz |
| **GPU** | 20-core |
| **Neural Engine** | 16-core |
| **RAM** | 64GB unified memory (BTO) |
| **Storage** | 8TB SSD (BTO) |
| **Price** | ~$2,200 (64GB/512GB) to ~$4,500 (maxed) |

### Display Support

- **Max monitors**: 3 simultaneous
- **Max resolution**: 3× 6K @ 60Hz, or 1× 8K + 1× 6K

#### Ports
- **Front**: 2× USB-C (10Gb/s), 3.5mm headphone
- **Back**: 3× Thunderbolt 5 (120Gb/s), HDMI 2.1, Gigabit Ethernet

### Why Good for LLMs

- **Unified memory** — GPU has full access to all 64GB RAM
- **Can run 70B parameter models** (quantized)
- Excellent `llama.cpp` and Ollama Metal support
- Low power, silent, tiny form factor

### Stock Configurations

| Model | CPU | GPU | RAM | Storage | Price |
|-------|-----|-----|-----|---------|-------|
| M4 Base | 10-core | 10-core | 16GB | 256GB | $599 |
| M4 | 10-core | 10-core | 16GB | 512GB | $799 |
| M4 Pro | 12-core | 16-core | 24GB | 512GB | $1,399 |
| M4 Pro | 14-core | 20-core | 24GB | 512GB | $1,599 |

**Note**: No M4 Max or M4 Ultra in Mac Mini — those are Mac Studio/Mac Pro only.

---

## Mac Studio M4 Max (2025) — High-End Desktop

### Target Configuration: 128GB RAM / 4TB Storage

| Spec | Configuration |
|------|---------------|
| **Chip** | M4 Max (16-core CPU, 40-core GPU) |
| **RAM** | 128GB unified memory |
| **Storage** | 4TB SSD |
| **Price** | **~$4,899** |

### Pricing Breakdown

| Component | Price |
|-----------|-------|
| Base (M4 Max 128GB/1TB) | $3,699 |
| +3TB storage upgrade | +$1,200 |
| **Total** | **~$4,899** |

### Display Support

- **Max monitors**: 5 simultaneous
- 4× 6K @ 60Hz via Thunderbolt + 1× 4K @ 144Hz via HDMI
- Or: 2× 6K + 1× 8K @ 60Hz

### Ports
- 6× Thunderbolt 5 (front: 2, back: 4)
- 2× USB-A
- HDMI 2.1
- 10Gb Ethernet
- SD card slot

### References
- [Mac Studio Tech Specs - Apple](https://support.apple.com/en-us/122211)
- [Mac Studio Pricing - AppleInsider](https://prices.appleinsider.com/mac-studio-2025)

---

## MacBook Pro M4 Max (2024) — Portable Option

### Target Configuration: 128GB RAM / 4TB Storage

Both 14" and 16" support this config with M4 Max (16-core CPU, 40-core GPU).

| Model | Base (M4 Max) | 128GB/4TB Config | Display |
|-------|---------------|------------------|---------|
| **14-inch** | $3,199 | **~$5,899** | 14.2" Liquid Retina XDR |
| **16-inch** | $3,499 | **~$6,299** | 16.2" Liquid Retina XDR |

### Upgrade Costs (approximate)
- 128GB RAM: +$1,300
- 4TB SSD: +$1,400

### External Display Support (M4 Max)

- **Max external monitors**: 4 simultaneous
- 3× 6K @ 60Hz via Thunderbolt + 1× 4K @ 144Hz via HDMI
- Or: 2× 6K + 1× 8K @ 60Hz via HDMI

### Ports
- 3× Thunderbolt 5
- HDMI 2.1
- SDXC card slot
- MagSafe 3
- 3.5mm headphone jack

### References
- [MacBook Pro 14" M4 Max - Apple](https://www.apple.com/shop/buy-mac/macbook-pro/14-inch-m4-max)
- [MacBook Pro 16" M4 Max - Apple](https://www.apple.com/shop/buy-mac/macbook-pro/16-inch-m4-max)
- [MacBook Pro Tech Specs - Apple](https://www.apple.com/macbook-pro/specs/)

---

## Comparison: 128GB / 4TB / 3+ Monitors

| Device | Price | Max Displays | Portable | Form Factor |
|--------|-------|--------------|----------|-------------|
| Mac Studio M4 Max | ~$4,899 | 5 | No | Desktop |
| MacBook Pro 14" | ~$5,899 | 4 (+internal) | Yes | Laptop |
| MacBook Pro 16" | ~$6,299 | 4 (+internal) | Yes | Laptop |

**Best value for desk use**: Mac Studio (~$1,000-1,400 cheaper than laptops)

---

## Comparison: 128GB / 8TB / 3+ Monitors

Upgrading from 4TB to 8TB adds **+$1,200** (~$300/TB).

| Device | 4TB Price | 8TB Price | Difference |
|--------|-----------|-----------|------------|
| Mac Studio M4 Max | ~$4,899 | **~$6,099** | +$1,200 |
| MacBook Pro 14" | ~$5,899 | **~$7,099** | +$1,200 |
| MacBook Pro 16" | ~$6,299 | **~$7,499** | +$1,200 |

### Storage Upgrade Tiers (Apple Pricing)

| Upgrade | Cost | Per TB |
|---------|------|--------|
| 512GB → 1TB | +$200 | $200/TB |
| 1TB → 2TB | +$400 | $400/TB |
| 2TB → 4TB | +$600 | $300/TB |
| 4TB → 8TB | +$1,200 | $300/TB |

**Note**: Apple's SSD pricing is heavily marked up. Third-party 8TB NVMe drives cost ~$900-1,200 standalone. Mac Studio SSD is technically user-replaceable (with effort); MacBook Pro is soldered.

---

## Thermal Performance: LLMs, VLMs, Stable Diffusion

Running AI workloads (LLM inference, image generation) creates **sustained high load** — very different from typical burst workloads. This pushes Apple Silicon to thermal limits.

### Temperature Ranges (Under AI Load)

| Workload | Typical Temp | Max Safe |
|----------|--------------|----------|
| LLM inference (7B-13B) | 90-102°C | ~105-107°C |
| Stable Diffusion | Up to 100°C | ~105-107°C |
| LLM + SD + other apps | 90°C GPU | — |

Apple Silicon throttles around **95-100°C** to protect itself.

### Device Comparison

| Device | Thermal Headroom | Fan Noise | Throttling | Verdict |
|--------|------------------|-----------|------------|---------|
| **Mac Studio** | Excellent | Near-silent (~1,700 RPM) | Rare | Best for sustained AI |
| **MacBook Pro 16"** | Good | Moderate under load | Minimal | Good portable option |
| **MacBook Pro 14"** | Limited | Loud under load (2× speed of 16") | 10-20% performance loss | Compromised for heavy AI |

### MacBook Pro 14" vs 16" for AI Workloads

| Aspect | 14-inch | 16-inch |
|--------|---------|---------|
| **Cooling capacity** | Smaller fans, less heatsink | Larger fans, better dissipation |
| **Sustained performance** | Throttles sooner | Maintains clocks longer |
| **Fan noise (heavy load)** | ~68 dB (loud) | Quieter, fans spin slower |
| **Surface temperature** | Hotter chassis | Spreads heat better |
| **Burst duration** | ~10-15 min before throttle | Longer sustained performance |

### Mac Studio Advantage

- **Near-silent operation** — fans barely audible even at 100% load
- **No throttling** — larger thermal envelope handles sustained AI workloads
- **Desktop cooling** — not constrained by laptop form factor
- Users report "not hearing fans" during Stable Diffusion + LLM workloads

### Real-World Example

> Running llama.cpp with 13B Q4_K_M model on M2 Pro MacBook Pro:
> - Fan noise spiked to **68 dB** within 90 seconds
> - CPU die peaked at **102°C**, GPU at **97°C**
> - SSD controller hit **89°C**
> - Thermal throttling triggered across all components

### Recommendations

| Use Case | Recommended Device |
|----------|-------------------|
| **24/7 LLM server / heavy SD** | Mac Studio |
| **Portable + occasional AI** | MacBook Pro 16" |
| **Portability priority** | MacBook Pro 14" (expect throttling) |

### Mitigation Tips (Laptops)

- Use a **laptop cooling pad** with fans
- Run in **clamshell mode** with external display (better airflow)
- Limit concurrent AI workloads
- Use **lower quantization** (Q4 instead of Q8) to reduce compute
- Monitor with `sudo powermetrics` or third-party tools

### References
- [14" M4 Max Thermal Throttling - MacRumors Forums](https://forums.macrumors.com/threads/14-m4-max-unbinned-thermal-throttling-share-your-benchmarks.2444147/)
- [Mac Studio M4 Max Overheating Guide](https://zeerawireless.com/blogs/news/m4-max-mac-studio-overheating-here-s-how-to-keep-it-cool-efficiently)
- [M4 Max Studio 128GB LLM Testing - MacRumors](https://forums.macrumors.com/threads/m4-max-studio-128gb-llm-testing.2453816/)
- [Mac Studio vs MacBook Pro M4 Max - MacRumors](https://forums.macrumors.com/threads/anyone-deciding-between-m4-max-macbook-pro-and-m4-max-mac-studio.2453247/)

---

## Budget Alternative: Intel N100 Mini PCs

For basic experimentation only. ~$150-200 for complete systems.

### LLM Performance

| Model Size | Speed | Usability |
|------------|-------|-----------|
| 1.5B params | ~5 tokens/sec | Usable for simple tasks |
| 7B+ params | Very slow | Not practical |

### Viable Models on N100
- Qwen 3 (1.7B)
- Gemma 3 (4B)
- DeepSeek-Coder (1.3B)
- Phi-3 Mini

**Verdict**: Fine for Home Assistant automation or experimenting. Not for daily use.

---

## References

### Mac Mini
- [Mac mini Tech Specs - Apple](https://www.apple.com/mac-mini/specs/)
- [Mac mini Display Support - Apple](https://support.apple.com/en-us/102194)

### Mac Studio
- [Mac Studio Tech Specs - Apple](https://support.apple.com/en-us/122211)
- [Mac Studio Pricing - AppleInsider](https://prices.appleinsider.com/mac-studio-2025)

### MacBook Pro
- [MacBook Pro Tech Specs - Apple](https://www.apple.com/macbook-pro/specs/)
- [MacBook Pro 14" M4 Max - Apple](https://www.apple.com/shop/buy-mac/macbook-pro/14-inch-m4-max)
- [MacBook Pro 16" M4 Max - Apple](https://www.apple.com/shop/buy-mac/macbook-pro/16-inch-m4-max)
- [MacBook Pro 16" Pricing - AppleInsider](https://prices.appleinsider.com/macbook-pro-16-inch-m4)
