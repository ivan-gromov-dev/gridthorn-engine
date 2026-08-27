# Repository Script Instructions

- Scripts must be non-interactive and deterministic. Provide matching
  PowerShell (`.ps1`) entrypoints for Windows and POSIX shell (`.sh`) entrypoints
  for macOS/Linux.
- Fail immediately when a delegated command fails and preserve its useful
  output.
- Resolve paths from `$PSScriptRoot`; do not depend on the caller's current
  directory.
- Never delete broad or unresolved paths. Validate exact generated targets
  before cleanup.
- Keep `verify.ps1` and `verify.sh` behaviorally aligned with each other and CI.
  Shared policy changes must update both platform entrypoints.
- Do not narrate ordinary commands with comments inside script blocks. Reserve
  an in-block comment for a non-obvious portability or safety constraint that
  cannot be expressed through a function or variable name.
