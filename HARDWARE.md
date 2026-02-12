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

- [Mac mini Tech Specs - Apple](https://www.apple.com/mac-mini/specs/)
- [Mac mini Display Support - Apple](https://support.apple.com/en-us/102194)
- [M4 Pro Mac Mini Display Support - MacRumors](https://www.macrumors.com/2024/10/29/m4-pro-mac-mini-display-support/)
