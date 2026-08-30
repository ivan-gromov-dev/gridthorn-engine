#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repository_root=$(CDPATH= cd -- "$script_dir/.." && pwd)

cd "$repository_root"

python3 - <<'PY'
import json
import subprocess

metadata_process = subprocess.run(
    ["cargo", "metadata", "--format-version", "1", "--no-deps"],
    check=True,
    capture_output=True,
    text=True,
)
metadata = json.loads(metadata_process.stdout)
workspace_ids = set(metadata["workspace_members"])
workspace_packages = [
    package for package in metadata["packages"] if package["id"] in workspace_ids
]
workspace_names = {package["name"] for package in workspace_packages}

allowed_dependencies = {
    "gridthorn": {"gridthorn_app", "gridthorn_input", "gridthorn_simulation", "gridthorn_world"},
    "gridthorn_app": {"gridthorn_input", "gridthorn_render", "gridthorn_simulation", "gridthorn_world"},
    "gridthorn_cli": set(),
    "gridthorn_input": set(),
    "gridthorn_render": set(),
    "gridthorn_simulation": set(),
    "gridthorn_world": set(),
}

for package in workspace_packages:
    package_name = package["name"]
    if package_name not in allowed_dependencies:
        raise SystemExit(
            f"Workspace crate '{package_name}' has no dependency-boundary rule"
        )

    actual_dependencies = {
        dependency["name"]
        for dependency in package["dependencies"]
        if dependency["name"] in workspace_names
    }
    forbidden = sorted(actual_dependencies - allowed_dependencies[package_name])
    if forbidden:
        raise SystemExit(
            f"Workspace crate '{package_name}' has forbidden project dependencies: "
            + ", ".join(forbidden)
        )

print("Dependency boundaries are valid.")
PY
