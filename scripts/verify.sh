#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repository_root=$(CDPATH= cd -- "$script_dir/.." && pwd)

cd "$repository_root"

printf '%s\n' '==> Formatting'
cargo fmt --all -- --check

printf '%s\n' '==> Workspace check'
cargo check --workspace --all-targets --locked

printf '%s\n' '==> Clippy'
cargo clippy --workspace --all-targets --locked -- -D warnings

printf '%s\n' '==> Tests'
cargo test --workspace --locked

sh "$script_dir/check-dependency-boundaries.sh"

