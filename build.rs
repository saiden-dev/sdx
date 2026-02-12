use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

const BASE_URL: &str = "https://github.com/saiden-dev/sdx/releases/download/sd-cli-v0.1.0";

fn binary_name() -> String {
    let os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set");

    let platform = match (os.as_str(), arch.as_str()) {
        ("linux", "x86_64") => "linux-x86_64",
        ("macos", "aarch64") => "macos-aarch64",
        _ => panic!("unsupported target: {os}-{arch}"),
    };

    format!("sd-cli-{platform}")
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let dest = out_dir.join("sd-cli");

    println!("cargo:rerun-if-changed=build.rs");

    if dest.exists() {
        return;
    }

    let name = binary_name();
    let url = format!("{BASE_URL}/{name}");

    eprintln!("Downloading {name} from GitHub release...");

    let response = ureq::get(&url)
        .call()
        .unwrap_or_else(|e| panic!("failed to download {name}: {e}"));

    let mut body = response.into_body().into_reader();
    let mut bytes = Vec::new();
    body.read_to_end(&mut bytes)
        .unwrap_or_else(|e| panic!("failed to read {name} response: {e}"));

    fs::write(&dest, &bytes)
        .unwrap_or_else(|e| panic!("failed to write sd-cli to {}: {e}", dest.display()));

    eprintln!("Downloaded {name} ({} bytes)", bytes.len());
}
