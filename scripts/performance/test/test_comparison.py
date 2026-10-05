"""Validate warning policy and reject incomplete performance measurements."""

import contextlib
import io
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from compare import annotate, compare, parse_samples, summarize


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


if __name__ == "__main__":
    unittest.main()
