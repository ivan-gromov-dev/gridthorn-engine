[CmdletBinding()]
param(
    [switch] $DocsOnly
)

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
    if ($DocsOnly) {
        $changedPaths = @(& git -c core.quotepath=false diff --name-only --no-renames --cached --)
        if ($LASTEXITCODE -ne 0) {
            throw "Could not inspect staged changes"
        }
        $changedPaths += @(& git -c core.quotepath=false diff --name-only --no-renames --)
        if ($LASTEXITCODE -ne 0) {
            throw "Could not inspect unstaged changes"
        }
        $changedPaths += @(& git -c core.quotepath=false ls-files --others --exclude-standard)
        if ($LASTEXITCODE -ne 0) {
            throw "Could not inspect untracked changes"
        }
        $nonMarkdownPaths = @($changedPaths | Where-Object { $_ -cnotmatch '\.md$' })
        if ($nonMarkdownPaths.Count -gt 0) {
            throw "Documentation-only verification rejects: $($nonMarkdownPaths -join ', ')"
        }
        Invoke-CheckedCommand "Staged documentation whitespace" { git diff --check --cached -- }
        Invoke-CheckedCommand "Unstaged documentation whitespace" { git diff --check -- }
        Write-Host "Documentation-only verification passed; executable snippets require their affected checks."
        return
    }
    Invoke-CheckedCommand "Formatting" { cargo fmt -- --check }
    Invoke-CheckedCommand "Workspace check" { cargo check --workspace --all-targets --locked }
    Invoke-CheckedCommand "Clippy" { cargo clippy --workspace --all-targets --locked -- -D warnings }
    Invoke-CheckedCommand "Tests" { cargo test --workspace --locked }
    & (Join-Path $PSScriptRoot "check-dependency-boundaries.ps1")
    Invoke-CheckedCommand "Diff whitespace" { git diff --check HEAD -- }
}
finally {
    Pop-Location
}

