use std::env;
use std::fs;
use std::path::PathBuf;

const DEFAULT_SD_CLI_SRC: &str = "/home/chi/Projects/stable-diffusion.cpp/build/bin/sd-cli";

fn main() {
    let sd_cli_src = env::var("SD_CLI_BIN").unwrap_or_else(|_| DEFAULT_SD_CLI_SRC.into());

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let dest = out_dir.join("sd-cli");

    println!("cargo:rerun-if-env-changed=SD_CLI_BIN");
    println!("cargo:rerun-if-changed={sd_cli_src}");

    fs::copy(&sd_cli_src, &dest)
        .unwrap_or_else(|e| panic!("failed to copy sd-cli from {sd_cli_src}: {e}"));
}
