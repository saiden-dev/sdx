# Plan: simple-diffusion

Rust CLI/server wrapping `sd-cli` from stable-diffusion.cpp. Single binary with two modes:
- `simple-diffusion generate` — CLI txt2img calling sd-cli as subprocess
- `simple-diffusion serve` — OpenAI-compatible HTTP API that calls sd-cli per request

Key feature: model selection via config/flag, auto-resolves to correct sd-cli arguments.

## Phase 1: Core CLI — txt2img

### Description
Build the foundation: argument parsing, model config, sd-cli process spawning, and basic txt2img generation from the command line.

### Steps

#### Step 1.1: Create project structure and dependencies
- **Objective**: Set up Cargo.toml with required dependencies
- **Files**: `Cargo.toml`
- **Dependencies**: None
- **Implementation**:
  - Add `clap` (derive) for CLI parsing
  - Add `serde`, `serde_json` for config deserialization
  - Add `toml` for config file parsing
  - Add `thiserror` for error types

#### Step 1.2: Define model configuration
- **Objective**: Define a TOML config format for named model profiles
- **Files**: `src/config.rs`
- **Dependencies**: None
- **Implementation**:
  - Define `ModelConfig` struct: name, model path(s) (single-file or split clip_l/clip_g/t5xxl/diffusion-model/vae), default generation params (steps, cfg_scale, guidance, sampler, scheduler, width, height)
  - Define `AppConfig` struct: sd_cli_path, models (HashMap<String, ModelConfig>), defaults
  - Load from `~/.config/simple-diffusion/config.toml`
  - Support both single-file models (`model = "path"`) and component models (clip_l, t5xxl, vae, diffusion_model as separate fields)

#### Step 1.3: Implement sd-cli command builder
- **Objective**: Build sd-cli argument vectors from typed Rust structs
- **Files**: `src/sd_cli.rs`
- **Dependencies**: Step 1.2
- **Implementation**:
  - Define `GenerateArgs` struct with all essential txt2img fields: prompt, negative_prompt, width, height, steps, cfg_scale, guidance, seed, sampling_method, scheduler, output, batch_count
  - Implement `to_args(&self) -> Vec<String>` that produces the sd-cli argument list
  - Merge model config defaults with per-request overrides
  - Implement `run()` that spawns sd-cli as `std::process::Command`, streams stderr for progress, waits for completion

#### Step 1.4: Implement CLI subcommand `generate`
- **Objective**: Wire up clap CLI with `generate` subcommand
- **Files**: `src/main.rs`, `src/cli.rs`
- **Dependencies**: Steps 1.2, 1.3
- **Implementation**:
  - Define clap `Cli` struct with subcommands: `Generate`, `Serve`
  - `Generate` subcommand: `--model` (name from config), `-p/--prompt`, `-n/--negative-prompt`, `-W/--width`, `-H/--height`, `--steps`, `--cfg-scale`, `--guidance`, `-s/--seed`, `--sampler`, `--scheduler`, `-o/--output`, `-b/--batch-count`
  - Load config, resolve model by name, build GenerateArgs, call sd_cli::run()
  - Print output path on success, stderr on failure

#### Step 1.5: Error handling and validation
- **Objective**: Proper error types and user-facing messages
- **Files**: `src/error.rs`
- **Dependencies**: Step 1.3
- **Implementation**:
  - Define error enum: ConfigNotFound, ModelNotFound, SdCliNotFound, SdCliFailed(exit_code, stderr), InvalidConfig
  - Validate sd-cli binary exists before spawning
  - Validate model files exist before spawning

## Phase 2: HTTP API Server

### Description
Add an OpenAI-compatible HTTP server mode that accepts image generation requests and calls sd-cli per request, returning base64-encoded images.

### Steps

#### Step 2.1: Add server dependencies
- **Objective**: Add HTTP framework to Cargo.toml
- **Files**: `Cargo.toml`
- **Dependencies**: Phase 1 complete
- **Implementation**:
  - Add `axum` for HTTP server
  - Add `tokio` (full) for async runtime
  - Add `base64` for image encoding
  - Add `tower-http` for CORS

#### Step 2.2: Define API types
- **Objective**: Define OpenAI-compatible request/response structs
- **Files**: `src/api.rs`
- **Dependencies**: Step 2.1
- **Implementation**:
  - `ImageGenerationRequest`: model (optional, selects config profile), prompt, n (batch), size (WxH string), negative_prompt (extension), steps, cfg_scale, guidance, seed, sampler, scheduler
  - `ImageGenerationResponse`: created (timestamp), data (vec of `ImageData { b64_json }`)
  - `ImageEditRequest`: multipart form with image, mask, prompt (for img2img — future)
  - `ModelsResponse` for `GET /v1/models` listing configured model profiles
  - Error response type with OpenAI-compatible error format

#### Step 2.3: Implement server and routes
- **Objective**: HTTP server with OpenAI endpoints
- **Files**: `src/server.rs`
- **Dependencies**: Steps 2.2, 1.3
- **Implementation**:
  - `GET /v1/models` — return list of configured model profiles
  - `POST /v1/images/generations` — parse request, resolve model from config, build GenerateArgs with temp output file, spawn sd-cli, read output PNG, base64-encode, return response, cleanup temp file
  - Request queue with mutex (one generation at a time, sd-cli is GPU-bound)
  - CORS middleware for browser clients

#### Step 2.4: Implement CLI subcommand `serve`
- **Objective**: Wire up `serve` subcommand
- **Files**: `src/main.rs`, `src/cli.rs`
- **Dependencies**: Step 2.3
- **Implementation**:
  - `Serve` subcommand: `--host` (default 127.0.0.1), `--port` (default 8080)
  - Load config, start axum server, log listening address

## Phase 3: Polish

### Steps

#### Step 3.1: Example config and documentation
- **Objective**: Ship a working example config
- **Files**: `config.example.toml`
- **Dependencies**: Phase 2 complete
- **Implementation**:
  - Example config with SD1.5, SDXL, and Flux model profiles
  - Comments explaining each field
  - Update CLAUDE.md with new architecture info

#### Step 3.2: Model listing command
- **Objective**: Add `simple-diffusion models` subcommand
- **Files**: `src/cli.rs`, `src/main.rs`
- **Dependencies**: Step 1.2
- **Implementation**:
  - List configured model profiles with their paths and default params
  - Show which model files exist/missing
