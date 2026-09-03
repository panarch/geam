# Publishing

Each Geam release publishes nine artifacts at one version: seven workspace
crates, the `geam-example-text-pattern` reference provider on crates.io, and the
`example_text_pattern` package on Hex. The workspace crates are `geam-core`,
`geam-macros`, `geam-stdlib`, `geam-json`, `geam-time`, `geam-cli`, and the root
`geam` facade. The root owns the installable `geam` binary.

`Cargo.toml` owns the release version; this guide does not repeat the current
version. The reference packages are public distribution fixtures rather than
independent products, so preparation copies that version into both package
manifests and the provider's exact Geam and Gleam requirements.

## Prepare And Review

1. Run **Geam: Prepare release** from `main` and choose `patch`, `minor`, or
   `major`. The default is `patch`; cargo-release computes the next version.
2. The workflow updates workspace, fixture, provider-example, and embedding
   example requirements; aligns both published reference packages to the
   release version; reconciles tracked Cargo and Gleam manifests; and opens a
   draft PR on `release/<version>`. It refuses an existing release branch or
   tag, or an open PR from that branch, and never force-pushes.
3. Add `docs/releases/<version>.md` to the PR following the
   [release note guide](release-notes.md), and add the new version to the release
   index. Write and review user-facing changes manually. Preparation does not
   invent release notes.
4. Approve workflow runs requested for the generated PR and wait for all checks.
   The workflow uses `GITHUB_TOKEN`, not an App or personal token. The repository
   must allow Actions to create PRs. If PR creation fails after pushing, open the
   PR from the retained branch instead of overwriting it with another prepare.
   To discard an unsuccessful preparation, close its PR and delete its remote
   release branch. A later dispatch may then prepare the same version again;
   the closed PR remains as the history of the abandoned attempt.
5. Review the manifest/lock changes and notes, mark the PR ready, squash-merge,
   then wait for the `Workspace`, `Coverage`, and `Acceptance` push runs at the
   merged commit.

Preparation delegates workspace versions and exact internal requirements to
[cargo-release](https://github.com/crate-ci/cargo-release/blob/master/docs/reference.md).
[cargo-edit](https://github.com/killercup/cargo-edit) updates exact Geam
requirements in standalone fixtures, example providers, and independently
locked embedding examples, and sets the reference provider package version. The
workflow sets the matching Hex package and provider metadata to the same
version, then asks Gleam to update the local package entry in the reference
project, the complete Rust embedding application, and the staged external
provider example. Each tracked Cargo lock is reconciled by `cargo update
--workspace` from its own directory, preserving its local Cargo configuration
and checkout patches before the new crates exist on crates.io.

The [prepare workflow](../../.github/workflows/prepare-release.yml) pins the release
tool versions and owns this sequence; there is no separate release script.
To preview only the workspace version change, run
`cargo release version patch --workspace` without `--execute`.
To inspect the complete preparation locally,
run the workflow's bump, requirement-update, and lock-refresh steps in a
disposable checkout, stopping before its commit/push/PR step.

## Publish

Run **Geam: Publish release** from `main` and select an `operation`:

| Operation | `crates` | Work performed |
| --- | --- | --- |
| `Publish release` | Empty | Publish all seven workspace crates, call the reference-example workflow, then create the GitHub Release. |
| `Retry workspace crates` | Remaining workspace crate names, space-separated | Publish those crates, call the reference-example workflow, then create the GitHub Release. |
| `Create GitHub Release` | Empty | Verify the workspace and reference example, then create only the GitHub Release. |

Every operation requires the full release `commit` SHA. There is no fallback to
the latest main commit. For a new release, select `Publish release`, enter the
reviewed merge SHA, leave `crates` empty, and enable **dry-run** first.

The first `Validate inputs` step rejects a non-main workflow ref, a malformed
SHA, an unknown operation, or an invalid operation/crate combination before
checkout. It reports the reason without authentication, uploads, or Release
creation; correct the inputs and dispatch again.

Before workspace authentication or uploads, the workflow checks:

- checkout at the exact release SHA, on main's history;
- matching workspace versions;
- a matching, nonempty release note file without placeholder markers;
- successful push runs of all three verification workflows at that SHA;
- an existing release tag, if any, pointing to that same SHA.

After the workspace succeeds, the workflow calls
[Geam: Publish reference example](../../.github/workflows/publish-reference-example.yml)
synchronously. That workflow independently validates the provider and Hex
package versions, publishes or dry-runs the two reference artifacts, and checks
the public execution path. The GitHub Release job starts only after both owners
have completed successfully.

Dry-run invokes native `cargo publish --locked ... --dry-run` for the workspace,
`gleam export hex-tarball` for the Hex package, and a provider dry-run patched to
the reviewed checkout because the new registry version does not exist yet. No
dry-run creates tags, uploads, or GitHub Releases. A successful dry-run does not
prove actual OIDC or Hex credentials, nor upload availability.

For a new release, run `Publish release` again with **dry-run** disabled and the
**same full commit SHA**. The workspace job publishes its seven crates through
Trusted Publishing. The reference workflow publishes the provider against the
released workspace, waits until crates.io serves it, then runs
`gleam publish --yes` with the Hex API key stored in the release environment.
For pre-1.0 versions, it supplies Gleam's required textual acknowledgement on
standard input and rejects Gleam's successful `Not publishing.` no-op result.

After uploading, `cargo info <crate>@<version> --registry crates-io` checks every
Rust package from outside the checkout, so local packages and patches cannot
satisfy the check. A clean Gleam project then installs the exact same-version Hex
package, discovers and approves the same-version provider, and runs with the
released Geam. Only then does `gh release create` publish the reviewed note file
verbatim, creating `v<version>` at the selected SHA. It does not move existing
tags or overwrite existing releases.

## Reference Example

The reference workflow is also directly dispatchable from `main`. Its operation
is independent of the workspace retry input:

| Operation | Work performed |
| --- | --- |
| `Publish reference example` | Publish and verify the Rust provider, publish the Hex package, then verify the complete public path. |
| `Publish Hex package` | Publish or dry-run only `example_text_pattern`. |
| `Publish Rust provider` | Publish or dry-run only `geam-example-text-pattern`; an actual publish waits for crates.io availability. |
| `Verify published example` | Perform no publication; install and execute the exact released Geam, Hex package, and provider combination. |

All operations require the original full release `commit` SHA. The workflow
accepts only a commit on main's history with successful `Workspace`, `Coverage`,
and `Acceptance` push runs, and requires the workspace, provider, provider
metadata, and Hex package to use the same version. **dry-run** applies to the
three publishing operations; verification is always read-only.

## Retry

Use the original full release SHA for every recovery operation. Do not select a
newer main commit for the same version.

- If workspace uploads failed, run **Geam: Publish release**, select
  `Retry workspace crates`, and enter only the remaining names in `crates`, for
  example `geam-cli geam`. Each name becomes a Cargo `--package` selection. Once
  all workspace crates are available, the normal reference workflow follows.
- If reference publication stopped after the workspace completed, inspect which
  component versions are public, then run **Geam: Publish reference example**
  once for each missing component. Publish the Rust provider before the Hex
  package. Use `Verify published example` after component recovery when the
  registry state needs an explicit check. The workflow does not automatically
  skip published versions; verification fails if any component is still absent.
- An upload error can occur after crates.io received the package. Check the
  upload log and exact published versions before choosing a component retry.
- Once all packages are available, run **Geam: Publish release**, select
  `Create GitHub Release`, and leave `crates` empty. This skips authentication
  and publication, checks all exact registry versions, then creates the GitHub
  Release. Its dry-run performs the checks without creating a tag or release.
- If GitHub already created the release before reporting a failure, inspect the
  existing release; this workflow does not overwrite it.
- A failure before any upload can use **Re-run failed jobs**, retaining the
  original SHA and inputs. A partial upload needs a new dispatch with the retry
  selection above.

Registry and GitHub network or permission failures stop the run. There is no
custom registry client, missing-package inference, or automatic recovery loop.

Publication attempts are serialized. There is no upload retry loop, personal
token fallback, or publication from a non-main workflow ref.

## Authentication

The seven workspace crates use this Trusted Publisher configuration:

- repository owner: `panarch`
- repository: `geam`
- workflow: `publish.yml`
- environment: `crates-io`

The reference provider needs two configurations with the same repository and
environment: `publish.yml` authorizes the reusable workflow when the main
release calls it, while `publish-reference-example.yml` authorizes direct
component recovery. crates.io identifies the workflow that entered the run, so
these two explicit entry points cannot share one workflow-file configuration.

Keep those workflow filenames and the environment stable. The environment
allows main only; crates.io requires Trusted Publishing for new versions.
`id-token: write` permits short-lived crates.io authentication, `actions: read`
checks CI, and the final release job alone receives `contents: write`. No
long-lived registry token or local-publish fallback is part of the regular
release path.

The same `crates-io` environment stores `HEXPM_API_KEY` for the Hex publication.
The reference workflow resolves that environment secret in both automatic and
direct runs. It does not use the key for crates.io and does not provide a local
or personal-token fallback for either registry.
