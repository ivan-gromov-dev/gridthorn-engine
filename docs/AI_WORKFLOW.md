# Repository AI workflow

Repository instructions own coding and verification policy. Skills define
repeatable implementation, review, diagnosis, and release workflows. This file
routes context and describes optional long-task state and measurements.

## Select context

Start with the requested scope, Git status, and applicable instruction chains.
For implementation, read ROADMAP's Immediate target and affected milestone,
ARCHITECTURE's Dependency and ownership rules, and PUBLIC_API_POLICY for public
behavior. These are routing anchors, not substitutes for additional applicable
invariants: follow references and search for the lifecycle, determinism, or
serialization rules relevant to the change. Historical milestone evidence and
proposed subsystems need not be loaded for an unrelated local fix.

Search with `rg` before reading large files and batch independent searches and
reads. Use Cargo metadata and lockfile diffs instead of reading the whole
lockfile unless dependency investigation needs it.

| Area | Owning crate(s) | Contract to read when relevant | Example or check |
| --- | --- | --- | --- |
| SDK exports and compatibility | `gridthorn` | [Public API policy](PUBLIC_API_POLICY.md) | Facade domain tests |
| ECS, schedules, reflection | `gridthorn_world` | [Architecture](ARCHITECTURE.md), [Reflection](REFLECTION.md) | World/schedule/reflection tests |
| Runtime and states | `gridthorn_app`, `gridthorn` | [Runtime APIs](RUNTIME_APIS.md), [Architecture](ARCHITECTURE.md) | App runtime/state tests |
| Keyboard, pointer, text input | `gridthorn_input`, `gridthorn_app` | [Input](INPUT.md) | Input and window domain tests |
| Font assets and multilingual UI | `gridthorn_assets`, `gridthorn_render`, `gridthorn_app` | [Text](TEXT.md) | Sibling `multilingual-text` example |
| UI composition and controls | `gridthorn_app`, `gridthorn_render`, `gridthorn` | [UI](UI.md) | Sibling `composed-controls` headless/native smoke |
| Locale selection and translated messages | `gridthorn_localization`, `gridthorn` | [Localization](LOCALIZATION.md) | Sibling `localization` example |
| Assets and reload | `gridthorn_assets` | [Assets](ASSETS.md) | Sibling `asset-reload --smoke` workflow |
| GPU and presentation | `gridthorn_render`, `gridthorn_app` | [Architecture](ARCHITECTURE.md), [Runtime APIs](RUNTIME_APIS.md) | Surface/presentation tests; affected window example |
| Audio | `gridthorn_audio`, runtime facade | [Runtime APIs](RUNTIME_APIS.md) | Audio command/output tests; affected audio example |
| Collision | `gridthorn_collision`, runtime facade | [Architecture](ARCHITECTURE.md) | Collision domain tests; sibling `collision-basics` |
| Grids, placement, navigation | `gridthorn_grid` | [Grids](GRIDS.md) | Sibling `grid-placement`, `pathfinding` examples |
| Fixed time and determinism | `gridthorn_simulation`, runtime facade | [Simulation](SIMULATION.md) | Sibling `simulation-clock`, `headless-simulation` |
| Scenarios and snapshots | `gridthorn_app`, `gridthorn_simulation` | [Scenarios](SCENARIOS.md) | Sibling `scenarios-snapshots` |
| Scene persistence | `gridthorn_scene`, `gridthorn_world` | [Scenes](SCENES.md) | Persistence roundtrip/migration tests |
| World saving | `gridthorn_app` | [World saves](WORLD_SAVES.md) | Sibling `world-saving` |
| CLI and templates | `gridthorn_cli` | [Development workflow](DEVELOPMENT_WORKFLOW.md), [CLI simulation](CLI_SIMULATION.md) | Command tests and generated-project workflow |
| Crates and dependencies | Workspace manifests | [Dependency boundaries](DEPENDENCY_BOUNDARIES.md) | Platform boundary checker |
| Automation and CI | `scripts`, `.github` | [Contributing](../CONTRIBUTING.md) | Full platform verify |

Examples live in `../gridthorn-examples`, with their own instructions and
manifest. Resolve exact package names from that manifest rather than inferring
them from directory names. Use `cargo test -p <crate> <domain-filter> --locked`
for focused engine checks and
`cargo test --manifest-path ../gridthorn-examples/Cargo.toml -p <package> --locked` for affected example
packages. A successful compilation cannot prove native device/window behavior;
exercise the documented smoke or interactive path when it is affected.

## Long-task checkpoint

For tasks spanning multiple sessions or substantial phases, maintain a small
checkpoint in `docs/work-in-progress/<task-name>.md`. Ordinary local fixes do
not need one. Write it after a material decision or phase, not after each tool.

Record the goal and limits, starting revision for each repository, decisions
and evidence, changed files, exact checks and results, unresolved issues, and
next action. Distinguish planned work from completed work. On resume, verify
Git status and revisions against the checkpoint before relying on its results.
Preserve unrelated changes. Remove the temporary checkpoint at completion after
moving durable decisions and deferrals into their owning documentation or ADR.

## Verification and output

Use full verify for code, configuration, scripts, and release candidates.
Markdown-only prose/instruction changes can use the explicit docs-only mode;
it checks tracked diff whitespace and rejects non-Markdown staged, unstaged,
and untracked paths. It does not review links, facts, skill behavior, or code
snippets: validate those according to the affected artifact. This mode inspects
changes against HEAD, not a committed branch range; review the intended range
separately when changes are already committed. CI continues full verification
on all supported platforms.

Keep complete failing diagnostics available while returning compact successful
results. For verbose commands, retain a log outside the reviewed change or in
ignored build output and include its path and exit status. Do not rerun an
unchanged failing input without new evidence or repeat a passed suite unless
later changes invalidate it. Keep Cargo commands sharing build output sequential.

## Measure improvements

Before claiming token or latency savings, compare representative tasks such as
a local bug fix, a cross-crate feature, a CLI change, a review, and a release.
Use isolated equivalent starting states and record model/reasoning settings,
cache/build state, scope, and correctness outcomes. Compare elapsed time, token
usage when exposed by the host, tool calls, repeated file reads, verification
time, and correction cycles. Mark unavailable usage data as unknown; characters
or instruction size are proxies, not measured token savings.

For build changes, compare the existing `check -> clippy -> test` sequence with
`clippy -> test` on equivalent warm and cold build states. Check target/feature
coverage and failure diagnostics before removing an entrypoint. Keep explicit
MSRV coverage and the full verification gate. Independent subagents are useful
only when authorized and their distinct work outweighs duplicated context and
coordination. Assign bounded context, a concrete result, and disjoint edit
ownership; measure total usage across agents as well as wall time.

On 2026-10-03, one warm-cache Windows run measured workspace check at about
0.6 seconds and Clippy at 0.8 seconds, both successful. The separate check is
retained: this sample establishes neither a cold-build benefit nor equivalent
failure diagnostics. End-to-end token and workflow savings remain unmeasured.
