install:
    #!/usr/bin/env bash
    set -euo pipefail
    # Bump pre-release version
    current=$(grep '^version' Cargo.toml | sed 's/.*pre\.\([0-9]*\)".*/\1/')
    next=$((current + 1))
    sed -i "s/pre\.$current/pre.$next/" Cargo.toml
    echo "Bumped to pre.$next"
    # Clean old installs
    rm -f ~/.cargo/bin/sdx
    sudo rm -f /usr/local/bin/sdx
    # Build and install
    cargo build --release
    sudo cp target/release/sdx /usr/local/bin/sdx
    sdx --version
