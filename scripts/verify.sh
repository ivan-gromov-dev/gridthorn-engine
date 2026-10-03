#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repository_root=$(CDPATH= cd -- "$script_dir/.." && pwd)

cd "$repository_root"

case "${1-}" in
    --docs-only)
        if [ "$#" -ne 1 ]; then
            printf '%s\n' 'Usage: verify.sh [--docs-only]' >&2
            exit 2
        fi
        staged_paths=$(git -c core.quotepath=false diff --name-only --no-renames --cached --)
        unstaged_paths=$(git -c core.quotepath=false diff --name-only --no-renames --)
        untracked_paths=$(git -c core.quotepath=false ls-files --others --exclude-standard)
        printf '%s\n' "$staged_paths" "$unstaged_paths" "$untracked_paths" |
            while IFS= read -r path; do
                case "$path" in
                    ''|*.md) ;;
                    *) printf 'Documentation-only verification rejects: %s\n' "$path" >&2; exit 1 ;;
                esac
            done
        git diff --check --cached --
        git diff --check --
        printf '%s\n' 'Documentation-only verification passed; executable snippets require their affected checks.'
        exit 0
        ;;
    '')
        if [ "$#" -ne 0 ]; then
            printf '%s\n' 'Usage: verify.sh [--docs-only]' >&2
            exit 2
        fi
        ;;
    *) printf '%s\n' 'Usage: verify.sh [--docs-only]' >&2; exit 2 ;;
esac

printf '%s\n' '==> Formatting'
cargo fmt --all -- --check

printf '%s\n' '==> Workspace check'
cargo check --workspace --all-targets --locked

printf '%s\n' '==> Clippy'
cargo clippy --workspace --all-targets --locked -- -D warnings

printf '%s\n' '==> Tests'
cargo test --workspace --locked

sh "$script_dir/check-dependency-boundaries.sh"
git diff --check HEAD --

