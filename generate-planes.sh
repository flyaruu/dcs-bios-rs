#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

cd "$ROOT_DIR"

if [[ "$#" -eq 0 ]]; then
    cargo run -p dcs-bios-codegen -- \
        --input-dir "$ROOT_DIR/model-definitions" \
        --output-dir "$ROOT_DIR/crates/generated/src"
else
    cargo run -p dcs-bios-codegen -- "$@"
fi

cargo fmt --all
