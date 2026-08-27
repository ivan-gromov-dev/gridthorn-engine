# CI Instructions

- CI must call repository-owned verification scripts instead of duplicating
  their logic in workflow YAML.
- Keep a dedicated MSRV job in addition to stable formatting, lint, and tests.
- Run stable verification on Windows, Linux, and macOS. Keep an aggregate
  `CI Success` job that succeeds only when every required job succeeds.
- Pin actions to an explicit major version and grant the minimum permissions.
- Do not add a CI-only exception for a failure reproducible by local scripts.
