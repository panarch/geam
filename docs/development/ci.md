# CI Coverage Diagnostics

The Coverage workflow can run one existing coverage closure manually and export
detailed reports from its test collection. Pull request and main-branch runs
still execute every coverage job with the same owner tests and 100% line and
region requirements.

## Select A Coverage Closure

In GitHub Actions, open **Coverage → Run workflow**, select the branch, choose
a `target`, and enable `diagnostics` when detailed reports are needed.

| Target | Coverage closure |
| --- | --- |
| `core-macros` | Core and macros, collected together and reported separately |
| `built-ins` | Erlang, built-in aggregation, stdlib, JSON, and time owners |
| `cli-binary` | CLI owners and root library/binary tests |
| `benchmark` | Benchmark tooling and support in their independent workspace |
| `application-arguments` | Application argument provider fixture |
| `process_service` | Process service provider consumer |
| `original_otp` | Original OTP provider consumer |
| `charlist_service` | Charlist service provider consumer |
| `dict_service` | Dict service provider consumer |
| `bytes_tree_service` | BytesTree service provider consumer |
| `selective_receive_service` | Selective receive provider consumer |
| `all` | Every existing coverage closure |

For example, run core/macros coverage on a work branch with detailed reports:

```sh
gh workflow run coverage.yml --ref WORK_BRANCH -f target=core-macros -f diagnostics=true
```

The workflow must first exist on the default branch to accept manual triggers.
Use `--ref` to select a branch containing this workflow configuration. Manual
execution triggers Coverage only; it does not trigger Workspace or Acceptance.
See [GitHub's manual workflow guide](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/manually-run-a-workflow).

The diagnostic commands live in `coverage.yml`. Each export step declares its
working directory and `COVERAGE_PACKAGES`; the shared `coverage_diagnostics`
YAML anchor contains only the commands. `default` retains a service consumer's
existing default report scope. Execution conditions and artifact uploads remain
visible in each job.

## Read The Diagnostic Artifact

With `diagnostics=true`, each selected job uploads a
`coverage-TARGET-CHECKOUT_SHA` artifact retained for seven days. It contains:

- `toolchain.txt`: the actual checkout commit, workspace, OS, Rust/Cargo, and
  cargo-llvm-cov versions.
- `reports.tsv`: the exit status of each diagnostic export.
- `PACKAGE/summary.json`: per-file and total coverage counts.
- `PACKAGE/text/`: source reports with function instantiations and execution
  counts. `text.log` records the verbose report command and selected objects.
- `PACKAGE/missing-lines.txt`: the uncovered-line report. A service consumer
  uses `default/` for its existing default report scope.

Diagnostics reuse the successful test collection. They do not build or run
tests again, add another owner's profiles, filter source files, or relax the
coverage threshold. Strict reports still fail the job when coverage is below
100%; later package reports and diagnostic exports run even after that failure.
If an export fails, its log and the other exports are still uploaded, and the
job remains failed.

If test collection fails or the job is cancelled, these exports are skipped:
an incomplete collection is not a coverage result. Read the test log first.
Detailed exports are disabled by default and are available only on manual runs.

When summary counts show a gap but the merged source report does not, inspect
the function instantiations in `text/`. Compare reports from the same owner
collection and build conditions; a local 100% result does not establish a
different platform's result. The usual owner-scope rules remain in
[Testing](testing.md), and [test development](test-development.md) describes how
to construct a meaningful test for a diagnosed gap.
