[CmdletBinding()]
param(
    [Parameter(Mandatory)] [string] $Baseline,
    [string] $Python = 'python',
    [string] $Output = (Join-Path $PSScriptRoot '../target/performance'),
    [string] $BuildTarget = ''
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$arguments = @((Join-Path $PSScriptRoot 'performance/compare.py'), '--baseline', $Baseline, '--output', $Output)
if ($BuildTarget) { $arguments += @('--build-target', $BuildTarget) }
& $Python @arguments
if ($LASTEXITCODE -ne 0) { throw "Performance comparison failed with exit code $LASTEXITCODE" }
