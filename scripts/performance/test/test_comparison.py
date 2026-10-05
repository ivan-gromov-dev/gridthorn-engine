"""Validate warning policy and reject incomplete performance measurements."""

import contextlib
import io
import sys
import unittest
from pathlib import Path
import json
import tempfile
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from compare import annotate, compare, main, parse_samples, resolve_probe, summarize


class ComparisonTests(unittest.TestCase):
    """Protect the measurement boundary and advisory regression decision."""

    def setUp(self):
        self.workload = {"prefix": "probe", "keys": ["kind"], "index": "batch",
                         "units": "calls", "cases": {"warm": 2}}
        self.output = ("running 1 test\n"
                       "test domain::probe ... probe,kind,batch,calls,elapsed_ns\n"
                       "probe,warm,0,2,999999\n"
                       "probe,warm,1,2,999999\n"
                       "probe,warm,2,2,20000\n"
                       "probe,warm,3,2,40000\n")

    def test_warmup_normalization_and_percentile(self):
        self.assertEqual(summarize(parse_samples(self.output, self.workload)),
                         {"warm": {"samples": 2, "median_ns": 15000, "p95_ns": 20000}})

    def test_rejects_missing_changed_duplicate_or_invalid_samples(self):
        for output in (self.output.rsplit("probe,warm,3", 1)[0],
                       self.output.replace("warm,3", "cold,3"),
                       self.output + "probe,warm,3,2,40000\n",
                       self.output.replace("40000", "nan"),
                       self.output.replace("3,2,40000", "3,0,40000"),
                       self.output.replace("3,2,40000", "3,2,-1"),
                       self.output.replace("probe,kind,batch,calls,elapsed_ns\n", "")):
            with self.subTest(output=output), self.assertRaises(ValueError):
                parse_samples(output, self.workload)

    def test_sustained_regression_warns_without_failing(self):
        base = [{"warm": {"median_ns": 10000, "p95_ns": 20000}}] * 2
        head = [{"warm": {"median_ns": 15000, "p95_ns": 30000}}] * 2
        self.assertTrue(all(row["warning"] for row in compare(base, head, .2, 1000)))

    def test_one_noisy_pair_and_small_absolute_changes_do_not_warn(self):
        base = [{"warm": {"median_ns": 10000, "p95_ns": 20000}}] * 2
        head = [{"warm": {"median_ns": 15000, "p95_ns": 30000}}, base[0]]
        self.assertFalse(any(row["warning"] for row in compare(base, head, .2, 1000)))
        tiny = [{"warm": {"median_ns": 100, "p95_ns": 100}}] * 2
        changed = [{"warm": {"median_ns": 200, "p95_ns": 200}}] * 2
        self.assertFalse(any(row["warning"] for row in compare(tiny, changed, .2, 1000)))

    def test_mismatched_pairs_are_errors(self):
        run = {"warm": {"median_ns": 1, "p95_ns": 1}}
        with self.assertRaises(ValueError):
            compare([run, run], [run, {}], .2, 1000)

    def test_annotations_escape_workflow_commands(self):
        stream = io.StringIO()
        with contextlib.redirect_stdout(stream):
            annotate("warning", "20%\n::error::unexpected\r")
        self.assertEqual(stream.getvalue(), "::warning::20%25%0A::error::unexpected%0D\n")

    def test_old_revision_may_lack_new_probe(self):
        self.assertIsNone(resolve_probe(["domain::older_probe"], "probe", "base"))
        self.assertEqual(resolve_probe(["domain::probe"], "probe", "head"), "domain::probe")

    def test_missing_current_and_ambiguous_probes_are_errors(self):
        with self.assertRaisesRegex(ValueError, "head: missing probe"):
            resolve_probe([], "probe", "head")
        for label in ("base", "head"):
            with self.subTest(label=label), self.assertRaisesRegex(ValueError, "ambiguous probe"):
                resolve_probe(["first::probe", "second::probe"], "probe", label)

    def run_cli(self, has_shared=False, invalid_current=False, build_target=False):
        """Exercise orchestration with an old revision that lacks one workload."""
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            workload = dict(self.workload, test="probe", package="fixture")
            workloads = [workload]
            if has_shared:
                workloads.append(dict(workload, test="shared"))
            config = output / "workloads.json"
            config.write_text(json.dumps(workloads), encoding="utf-8")
            arguments = ["compare.py", "--baseline", directory, "--output", directory]
            if build_target:
                arguments.extend(["--build-target", str(output / "cache")])
            probes = {"probe": ("head", "probe"), "shared": ("head", "shared")}
            previous = {"shared": ("base", "shared")} if has_shared else {}
            sample = self.output + "test result: ok. 1 passed; 0 failed\n"
            if invalid_current:
                sample = sample.replace("probe,warm,3,2,40000\n", "")
            stream = io.StringIO()
            with patch("sys.argv", arguments), patch("compare.CONFIG", config), \
                    patch("compare.build", side_effect=[previous, probes]) as builds, \
                    patch("compare.command", return_value=sample), \
                    patch.dict("os.environ", {"RUSTUP_TOOLCHAIN": "fixture", "GITHUB_STEP_SUMMARY": ""}), \
                    contextlib.redirect_stdout(stream):
                if invalid_current:
                    with self.assertRaises(SystemExit) as error:
                        main()
                    self.assertEqual(error.exception.code, 1)
                else:
                    main()
            cache = output / "cache" if build_target else output
            for label, call in zip(("base", "head"), builds.call_args_list):
                self.assertEqual(Path(call.args[4]["CARGO_TARGET_DIR"]), cache / f"target-{label}")
            report = json.loads((output / "comparison.json").read_text(encoding="utf-8"))
            return report, stream.getvalue()

    def test_first_run_records_current_baseline_without_false_comparison(self):
        report, annotations = self.run_cli()
        self.assertEqual(report["status"], "baseline_only")
        self.assertEqual(report["results"], [])
        self.assertEqual(len(report["uncompared"][0]["head_runs"]), 2)
        self.assertIn("::warning::No previous measurement", annotations)

    def test_partial_baseline_compares_shared_workloads(self):
        report, _ = self.run_cli(has_shared=True)
        self.assertEqual(report["status"], "partial")
        self.assertEqual(report["results"][0]["workload"]["test"], "shared")
        self.assertEqual(report["uncompared"][0]["workload"]["test"], "probe")

    def test_custom_cache_keeps_revision_builds_isolated(self):
        self.run_cli(build_target=True)

    def test_baseline_initialization_still_rejects_incomplete_current_data(self):
        report, annotations = self.run_cli(invalid_current=True)
        self.assertEqual(report["status"], "incomplete")
        self.assertIn("::error::Performance comparison incomplete", annotations)


if __name__ == "__main__":
    unittest.main()
