"""Run existing release probes and compare paired revisions on one host."""

import argparse
import csv
import json
import math
import os
from pathlib import Path
import platform
import shutil
import statistics
import subprocess


ROOT = Path(__file__).resolve().parents[2]
CONFIG = Path(__file__).with_name("workloads.json")


def command(arguments, root, env, log):
    """Preserve command output without interpreting it as workflow commands."""
    result = subprocess.run(arguments, cwd=root, env=env, capture_output=True, text=True,
                            encoding="utf-8", errors="replace", timeout=1200)
    log.write_text(result.stdout + result.stderr, encoding="utf-8")
    if result.returncode:
        raise RuntimeError(f"Command failed ({result.returncode}); see {log}")
    return result.stdout


def parse_samples(output, workload):
    """Reject truncated, changed, or invalid CSV instead of reporting success."""
    header = None
    samples = {}
    seen = set()
    prefix = workload["prefix"] + ","
    for line in output.splitlines():
        if line.startswith("test ") and " ... " in line:
            line = line.split(" ... ", 1)[1]
        if not line.startswith(prefix):
            continue
        fields = next(csv.reader([line]))[1:]
        if "elapsed_ns" in fields:
            if header is not None and header != fields:
                raise ValueError("Conflicting CSV headers")
            header = fields
            continue
        if header is None or len(fields) != len(header):
            raise ValueError("Missing header or malformed sample")
        row = dict(zip(header, fields))
        case = "/".join(row[name] for name in workload["keys"])
        index = int(row[workload["index"]])
        identity = (case, index)
        if identity in seen:
            raise ValueError("Duplicate sample")
        seen.add(identity)
        divisor = float(row[workload["units"]]) if workload.get("units") else 1
        if not math.isfinite(divisor) or divisor <= 0:
            raise ValueError("Invalid work count")
        value = float(row["elapsed_ns"]) / divisor
        if not math.isfinite(value) or value < 0:
            raise ValueError("Invalid timing or work count")
        if index >= 2:
            samples.setdefault(case, []).append(value)
    counts = {key: len(values) for key, values in samples.items()}
    if counts != workload["cases"]:
        raise ValueError(f"Workload coverage changed: expected {workload['cases']}, got {counts}")
    return samples


def summarize(samples):
    """Report median and nearest-rank p95 of operations or batch means."""
    return {key: {"samples": len(values), "median_ns": statistics.median(values),
                  "p95_ns": sorted(values)[math.ceil(len(values) * .95) - 1]}
            for key, values in samples.items()}


def compare(base, head, threshold, floor_ns):
    """Warn only when both independent pairs exceed relative and absolute floors."""
    if len(base) != 2 or len(head) != 2 or any(set(run) != set(base[0]) for run in base + head):
        raise ValueError("Paired coverage mismatch")
    rows = []
    for case in sorted(base[0]):
        for metric in ("median_ns", "p95_ns"):
            previous = [run[case][metric] for run in base]
            current = [run[case][metric] for run in head]
            if any(value <= 0 or not math.isfinite(value) for value in previous + current):
                raise ValueError("Invalid comparison timing")
            ratios = [new / old for old, new in zip(previous, current)]
            warning = all(ratio > 1 + threshold and new - old > floor_ns
                          for ratio, old, new in zip(ratios, previous, current))
            rows.append({"case": case, "metric": metric, "base_ns": previous,
                         "head_ns": current, "ratios": ratios, "warning": warning})
    return rows


def annotate(kind, message):
    """Escape workflow annotation data, including percent and line breaks."""
    escaped = message.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
    print(f"::{kind}::{escaped}", flush=True)


def resolve_probe(names, test, label):
    """Allow a newly introduced workload to have no previous measurement."""
    matches = [name for name in names if name.endswith("::" + test)]
    if not matches and label == "base":
        return None
    if len(matches) != 1:
        raise ValueError(f"{label}: {'missing' if not matches else 'ambiguous'} probe {test}")
    return matches[0]


def build(root, label, workloads, output, env):
    """Compile library tests once, then resolve each exact ignored test name."""
    packages = sorted({item["package"] for item in workloads})
    args = ["cargo", "test", "--lib", "--release", "--locked", "--no-run", "--message-format=json"]
    for package in packages:
        args.extend(["-p", package])
    print(f"Building {label}: {', '.join(packages)}", flush=True)
    raw = command(args, root, env, output / f"{label}-build.log")
    executables = {}
    for line in raw.splitlines():
        artifact = json.loads(line)
        if artifact.get("reason") == "compiler-artifact" and artifact.get("executable"):
            executables[artifact["target"]["name"]] = artifact["executable"]
    probes = {}
    for package in packages:
        binary = Path(executables[package])
        executable = output / f"{label}-{package}{binary.suffix}"
        shutil.copy2(binary, executable)
        listing = command([executable, "--list", "--ignored"], root, env,
                          output / f"{label}-{package}-list.log")
        names = [line.removesuffix(": test") for line in listing.splitlines() if line.endswith(": test")]
        for item in (item for item in workloads if item["package"] == package):
            name = resolve_probe(names, item["test"], label)
            if name is not None:
                probes[item["test"]] = (executable, name)
    return probes


def main():
    """Write raw logs, revision metadata and a complete comparison artifact."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--current", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, default=ROOT / "target/performance")
    parser.add_argument("--threshold", type=float, default=.20)
    parser.add_argument("--floor-ns", type=float, default=1000)
    parser.add_argument("--workload", action="append", help="Select probe by function name")
    parser.add_argument("--build-target", type=Path, help="Parent directory for separate base/head Cargo caches")
    args = parser.parse_args()
    if not math.isfinite(args.threshold) or args.threshold < 0 or not math.isfinite(args.floor_ns) or args.floor_ns < 0:
        parser.error("Threshold and floor must be finite and nonnegative")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    workloads = json.loads(CONFIG.read_text(encoding="utf-8"))
    if args.workload:
        workloads = [item for item in workloads if item["test"] in args.workload]
        if {item["test"] for item in workloads} != set(args.workload):
            parser.error("Unknown workload")
    env = {key: value for key, value in os.environ.items() if not key.startswith("GRIDTHORN_")}
    env["RUST_TEST_THREADS"] = "1"
    roots = {"base": args.baseline.resolve(), "head": args.current.resolve()}
    report = {"platform": platform.platform(), "threshold": args.threshold,
              "floor_ns": args.floor_ns, "order": ["base", "head", "head", "base"],
              "revisions": {}, "results": [], "uncompared": [], "status": "incomplete"}
    try:
        if "RUSTUP_TOOLCHAIN" not in env:
            active = command(["rustup", "show", "active-toolchain"], roots["head"], env,
                             output / "active-toolchain.log")
            env["RUSTUP_TOOLCHAIN"] = active.split()[0]
        for label, root in roots.items():
            report["revisions"][label] = command(["git", "rev-parse", "HEAD"], root, env,
                                                 output / f"{label}-revision.log").strip()
        report["toolchain"] = command(["rustc", "-vV"], roots["head"], env, output / "toolchain.log")
        probes = {}
        for label, root in roots.items():
            target = (args.build_target.resolve() if args.build_target else output) / f"target-{label}"
            build_env = dict(env, CARGO_TARGET_DIR=str(target))
            probes[label] = build(root, label, workloads, output, build_env)
        for item in workloads:
            print(f"Measuring {item['test']}", flush=True)
            runs = {"base": [], "head": []}
            for label in report["order"]:
                if label == "base" and item["test"] not in probes["base"]:
                    continue
                executable, name = probes[label][item["test"]]
                raw = command([executable, name, "--exact", "--ignored", "--nocapture", "--test-threads=1"],
                              roots[label], env, output / f"{item['test']}-{label}-{len(runs[label])}.log")
                if "1 passed; 0 failed" not in raw:
                    raise ValueError(f"{label}: exact probe did not run")
                runs[label].append(summarize(parse_samples(raw, item)))
            if not runs["base"]:
                report["uncompared"].append({"workload": item, "head_runs": runs["head"],
                                            "reason": "probe absent in baseline revision"})
                continue
            rows = compare(runs["base"], runs["head"], args.threshold, args.floor_ns)
            report["results"].append({"workload": item, "runs": runs, "comparison": rows})
            warnings = [row for row in rows if row["warning"]]
            if warnings:
                worst = max(warnings, key=lambda row: min(row["ratios"]))
                annotate("warning", f"{item['test']}: {len(warnings)} metrics slower in both pairs; "
                         f"largest sustained change {worst['case']}/{worst['metric']} "
                         f"({worst['ratios'][0]:.2f}x, {worst['ratios'][1]:.2f}x). See comparison.json.")
        if report["uncompared"]:
            report["status"] = "partial" if report["results"] else "baseline_only"
            names = ", ".join(item["workload"]["test"] for item in report["uncompared"])
            annotate("warning", f"No previous measurement for {len(report['uncompared'])} workloads: {names}. "
                     "Current measurements saved; these workloads were not compared. See comparison.json.")
        else:
            report["status"] = "complete"
    except (OSError, subprocess.SubprocessError, ValueError, KeyError, RuntimeError) as error:
        report["error"] = str(error)
        annotate("error", f"Performance comparison incomplete: {error}")
        raise SystemExit(1) from error
    finally:
        (output / "comparison.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        warnings = sum(row["warning"] for result in report["results"] for row in result["comparison"])
        summary = (f"Performance comparison: {report['status']}; {len(report['results'])}/{len(workloads)} compared; "
                   f"{len(report['uncompared'])} current-only; {warnings} regression warnings.\n")
        print(summary, flush=True)
        if os.environ.get("GITHUB_STEP_SUMMARY"):
            with open(os.environ["GITHUB_STEP_SUMMARY"], "a", encoding="utf-8") as stream:
                stream.write(summary)


if __name__ == "__main__":
    main()
