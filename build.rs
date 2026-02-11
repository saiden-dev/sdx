use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

const DOWNLOAD_URL: &str =
    "https://github.com/saiden-dev/sdx/releases/download/sd-cli-v0.1.0/sd-cli";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let dest = out_dir.join("sd-cli");

    println!("cargo:rerun-if-changed=build.rs");

    if dest.exists() {
        return;
    }

    eprintln!("Downloading sd-cli from GitHub release...");

    let response = ureq::get(DOWNLOAD_URL)
        .call()
        .unwrap_or_else(|e| panic!("failed to download sd-cli: {e}"));

    let mut body = response.into_body().into_reader();
    let mut bytes = Vec::new();
    body.read_to_end(&mut bytes)
        .unwrap_or_else(|e| panic!("failed to read sd-cli response: {e}"));

    fs::write(&dest, &bytes)
        .unwrap_or_else(|e| panic!("failed to write sd-cli to {}: {e}", dest.display()));

    eprintln!("Downloaded sd-cli ({} bytes)", bytes.len());
}
