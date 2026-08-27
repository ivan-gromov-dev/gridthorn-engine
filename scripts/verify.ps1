Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory)]
        [string] $Description,
        [Parameter(Mandatory)]
        [scriptblock] $Command
    )

    Write-Host "==> $Description"
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Description failed with exit code $LASTEXITCODE"
    }
}

$repositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $repositoryRoot
try {
    Invoke-CheckedCommand "Formatting" { cargo fmt --all -- --check }
    Invoke-CheckedCommand "Workspace check" { cargo check --workspace --all-targets --locked }
    Invoke-CheckedCommand "Clippy" { cargo clippy --workspace --all-targets --locked -- -D warnings }
    Invoke-CheckedCommand "Tests" { cargo test --workspace --locked }
    & (Join-Path $PSScriptRoot "check-dependency-boundaries.ps1")
}
finally {
    Pop-Location
}

