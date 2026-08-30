Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $repositoryRoot
try {
    $metadataJson = & cargo metadata --format-version 1 --no-deps
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed with exit code $LASTEXITCODE"
    }

    $metadata = $metadataJson | ConvertFrom-Json
    $workspaceIds = @{}
    foreach ($id in $metadata.workspace_members) {
        $workspaceIds[$id] = $true
    }
    $workspaceNames = @(
        $metadata.packages |
            Where-Object { $workspaceIds.ContainsKey($_.id) } |
            ForEach-Object { $_.name }
    )

    $allowedDependencies = @{
        gridthorn = @("gridthorn_app", "gridthorn_world")
        gridthorn_app = @("gridthorn_render", "gridthorn_world")
        gridthorn_cli = @()
        gridthorn_render = @()
        gridthorn_simulation = @()
        gridthorn_world = @()
    }

    foreach ($package in $metadata.packages) {
        if (-not $workspaceIds.ContainsKey($package.id)) {
            continue
        }
        if (-not $allowedDependencies.ContainsKey($package.name)) {
            throw "Workspace crate '$($package.name)' has no dependency-boundary rule"
        }

        $actual = @(
            $package.dependencies |
                Where-Object { $_.name -in $workspaceNames } |
                ForEach-Object { $_.name }
        )
        $forbidden = @($actual | Where-Object { $_ -notin $allowedDependencies[$package.name] })
        if ($forbidden.Count -gt 0) {
            throw "Workspace crate '$($package.name)' has forbidden project dependencies: $($forbidden -join ', ')"
        }
    }

    Write-Host "Dependency boundaries are valid."
}
finally {
    Pop-Location
}
