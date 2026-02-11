# stable-diffusion.cpp CLI Reference

Binary: `/home/chi/Projects/stable-diffusion.cpp/build/bin/sd-cli`
Version: `master-504-636d3cb`

## Modes

`-M, --mode` — one of: `img_gen` (default), `vid_gen`, `upscale`, `convert`

## CLI Options

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `-o, --output` | string | `./output.png` | Output path (supports printf `%d` for sequences) |
| `-M, --mode` | enum | `img_gen` | Run mode: img_gen, vid_gen, upscale, convert |
| `-v, --verbose` | bool | false | Print extra info |
| `--color` | bool | false | Color logging tags by level |
| `--preview` | enum | `none` | Preview method: none, proj, tae, vae |
| `--preview-path` | string | `./preview.png` | Path to write preview image |
| `--preview-interval` | int | 1 | Steps between preview updates |
| `--canny` | bool | false | Apply canny edge detection preprocessor |

## Context Options (Model & Backend)

### Model Paths

| Flag | Description |
|------|-------------|
| `-m, --model` | Path to full model (single-file checkpoint) |
| `--clip_l` | CLIP-L text encoder |
| `--clip_g` | CLIP-G text encoder |
| `--t5xxl` | T5-XXL text encoder |
| `--llm` | LLM text encoder (qwenvl2.5, mistral-small3.2, ...) |
| `--diffusion-model` | Standalone diffusion model |
| `--vae` | Standalone VAE model |
| `--taesd` / `--tae` | Tiny AutoEncoder (fast decoding, lower quality) |
| `--control-net` | ControlNet model |
| `--upscale-model` | ESRGAN upscaler model |
| `--photo-maker` | PhotoMaker model |
| `--lora-model-dir` | LoRA model directory (default: `.`) |
| `--embd-dir` | Embeddings directory |

### Computation

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `-t, --threads` | int | -1 (auto) | Thread count |
| `--type` | enum | (from file) | Weight type: f32, f16, q4_0, q4_1, q5_0, q5_1, q8_0, q2_K, q3_K, q4_K |
| `--rng` | enum | cuda | RNG: std_default, cuda, cpu |
| `--prediction` | enum | (auto) | Prediction type: eps, v, edm_v, sd3_flow, flux_flow, flux2_flow |
| `--lora-apply-mode` | enum | auto | LoRA application: auto, immediately, at_runtime |

### Memory Optimization

| Flag | Description |
|------|-------------|
| `--offload-to-cpu` | Offload weights to RAM, load to VRAM on demand |
| `--mmap` | Memory-map model files |
| `--vae-on-cpu` | Keep VAE on CPU |
| `--clip-on-cpu` | Keep CLIP on CPU |
| `--fa` | Use Flash Attention (global) |
| `--diffusion-fa` | Flash Attention for diffusion model only |
| `--vae-tiling` | Process VAE in tiles to reduce memory |
| `--vae-tile-size` | Tile size format `[X]x[Y]` (default: 32x32) |

## Generation Options

### Core Parameters

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `-p, --prompt` | string | | The prompt to render |
| `-n, --negative-prompt` | string | `""` | Negative prompt |
| `-H, --height` | int | 512 | Image height in pixels |
| `-W, --width` | int | 512 | Image width in pixels |
| `--steps` | int | 20 | Number of sample steps |
| `--cfg-scale` | float | 7.0 | Classifier-free guidance scale |
| `--guidance` | float | 3.5 | Distilled guidance (Flux/SD3) |
| `-s, --seed` | int | 42 | RNG seed (< 0 for random) |
| `-b, --batch-count` | int | 1 | Number of images to generate |
| `--strength` | float | 0.75 | Noising/unnoising strength (img2img) |
| `--clip-skip` | int | -1 (auto) | CLIP layers to skip |

### Sampling

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--sampling-method` | enum | euler_a | Sampler: euler, euler_a, heun, dpm2, dpm++2s_a, dpm++2m, dpm++2mv2, ipndm, ipndm_v, lcm, ddim_trailing, tcd, res_multistep, res_2s |
| `--scheduler` | enum | discrete | Scheduler: discrete, karras, exponential, ays, gits, smoothstep, sgm_uniform, simple, kl_optimal, lcm, bong_tangent |
| `--sigmas` | string | | Custom sigma values (comma-separated) |

### Input Images

| Flag | Description |
|------|-------------|
| `-i, --init-img` | Init image for img2img |
| `--mask` | Mask image for inpainting |
| `--control-image` | Control image for ControlNet |
| `-r, --ref-image` | Reference image for Flux Kontext (repeatable) |

### ControlNet & Guidance

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--control-strength` | float | 0.9 | ControlNet strength |
| `--slg-scale` | float | 0 (disabled) | Skip Layer Guidance scale (DiT models, 2.5 for SD3.5 medium) |
| `--skip-layer-start` | float | 0.01 | SLG enabling point |
| `--skip-layer-end` | float | 0.2 | SLG disabling point |
| `--skip-layers` | array | [7,8,9] | Layers to skip for SLG |

### Caching (Speed Optimization)

| Flag | Type | Description |
|------|------|-------------|
| `--cache-mode` | enum | easycache (DiT), ucache (UNET), dbcache/taylorseer/cache-dit (DiT block-level) |
| `--cache-option` | string | Named params: threshold=, start=, end=, decay=, relative=, reset= |
| `--cache-preset` | enum | cache-dit preset: slow/s, medium/m, fast/f, ultra/u |

## Server Mode

Binary: `/home/chi/Projects/stable-diffusion.cpp/build/bin/sd-server`

Shares all Context and Generation options with `sd-cli`, plus:

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `-l, --listen-ip` | string | 127.0.0.1 | Server bind address |
| `--listen-port` | int | 1234 | Server port |
| `--serve-html-path` | string | | HTML file to serve at root |

### API Endpoints

**OpenAI-Compatible:**
- `GET /v1/models` — list models (returns `sd-cpp-local`)
- `POST /v1/images/generations` — txt2img
- `POST /v1/images/edits` — img2img/inpaint (multipart form)

**AUTOMATIC1111-Compatible:**
- `POST /sdapi/v1/txt2img` — txt2img
- `POST /sdapi/v1/img2img` — img2img
- `GET /sdapi/v1/samplers` — list samplers
- `GET /sdapi/v1/schedulers` — list schedulers
- `GET /sdapi/v1/sd-models` — list models
- `GET /sdapi/v1/loras` — list LoRAs
- `GET /sdapi/v1/options` — server options

### OpenAI Request Format (`POST /v1/images/generations`)

```json
{
  "prompt": "string",
  "n": 1,
  "size": "512x512",
  "output_format": "png",
  "output_compression": 100
}
```

Response:
```json
{
  "created": 1234567890,
  "data": [{ "b64_json": "..." }],
  "output_format": "png"
}
```

### AUTOMATIC1111 Request Format (`POST /sdapi/v1/txt2img`)

```json
{
  "prompt": "string",
  "negative_prompt": "",
  "width": 512,
  "height": 512,
  "steps": 20,
  "cfg_scale": 7.0,
  "seed": -1,
  "batch_size": 1,
  "clip_skip": -1,
  "sampler_name": "euler_a",
  "scheduler": "discrete",
  "lora": [{ "path": "name.safetensors", "multiplier": 1.0 }]
}
```

Response:
```json
{
  "images": ["base64_image_1", ...],
  "parameters": {},
  "info": ""
}
```

## Supported Model Formats

- `.ckpt`, `.pth` — PyTorch checkpoints
- `.safetensors` — SafeTensors
- `.gguf` — GGUF quantized

## Example Commands

```bash
# Basic txt2img
sd-cli -m model.safetensors -p "a cat" -o output.png

# With specific dimensions and steps
sd-cli -m model.safetensors -p "a cat" -H 768 -W 768 --steps 30 -o output.png

# Flux model with separate components
sd-cli --diffusion-model flux-dev-q8_0.gguf --clip_l clip_l.safetensors --t5xxl t5xxl_fp16.safetensors --vae ae.safetensors -p "a cat" -o output.png

# img2img
sd-cli -m model.safetensors -p "oil painting" -i input.png --strength 0.6 -o output.png

# With LoRA (embedded in prompt)
sd-cli -m model.safetensors -p "a cat <lora:detail:0.8>" --lora-model-dir ./loras -o output.png

# Low VRAM
sd-cli -m model.safetensors -p "a cat" --offload-to-cpu --vae-tiling --fa -o output.png

# Random seed
sd-cli -m model.safetensors -p "a cat" -s -1 -o output.png
```
