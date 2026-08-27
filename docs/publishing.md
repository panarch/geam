# Publishing

Geam publishes seven lockstep crates: `geam-core`, `geam-macros`,
`geam-stdlib`, `geam-json`, `geam-time`, `geam-cli`, and the root `geam` facade.
The root owns the installable `geam` binary. Cargo derives dependency order
from the workspace graph and polls the registry after uploads.

`Cargo.toml` owns the release version; this guide does not repeat the current
version. Provider package versions, Gleam versions, compiler dependencies, and
the Rust toolchain are independent of a Geam version bump.

## Prepare And Review

1. Run **Geam: Prepare release** from `main` and choose `patch`, `minor`, or
   `major`. The default is `patch`; cargo-release computes the next version.
2. The workflow updates workspace and provider requirements, reconciles tracked
   Cargo locks, and opens a draft PR on `release/<version>`. It refuses an
   existing release branch, PR, or tag and never force-pushes.
3. Add `docs/releases/<version>.md` to the PR following the
   [release note guide](releases/README.md). Write and review user-facing changes
   manually. Preparation does not invent release notes.
4. Approve workflow runs requested for the generated PR and wait for all checks.
   The workflow uses `GITHUB_TOKEN`, not an App or personal token. The repository
   must allow Actions to create PRs. If PR creation fails after pushing, open the
   PR from the retained branch instead of overwriting it with another prepare.
5. Review the manifest/lock changes and notes, mark the PR ready, squash-merge,
   then wait for the `Workspace`, `Coverage`, and `Acceptance` push runs at the
   merged commit.

Preparation delegates workspace versions and exact internal requirements to
[cargo-release](https://github.com/crate-ci/cargo-release/blob/master/docs/reference.md).
[cargo-edit](https://github.com/killercup/cargo-edit) updates only the independent
providers' exact `geam` requirements, with recursive dependency upgrades disabled.
Each tracked lock is reconciled by `cargo update --workspace` from its own
directory, preserving its local Cargo configuration. Provider package versions
and unrelated dependencies must remain unchanged in the reviewed PR.

The [prepare workflow](../.github/workflows/prepare-release.yml) pins the release
tool versions and owns this sequence; there is no separate release script.
To preview only the workspace version change, run
`cargo release version patch --workspace` without `--execute`.
To inspect the complete preparation locally,
run the workflow's bump, requirement-update, and lock-refresh steps in a
disposable checkout, stopping before its commit/push/PR step.

## Publish

Run **Geam: Publish crates** from `main` and select an `operation`:

| Operation | `packages` | Work performed |
| --- | --- | --- |
| `Publish workspace` | Empty | Publish all workspace crates, then create the GitHub Release. |
| `Retry packages` | Remaining crate names, space-separated | Publish those crates, then create the GitHub Release. |
| `Create GitHub Release` | Empty | Verify all crates are published, then create only the GitHub Release. |

Every operation requires the full release `commit` SHA. There is no fallback to
the latest main commit. For a new release, select `Publish workspace`, enter the
reviewed merge SHA, leave `packages` empty, and enable **dry-run** first.

The first `Validate inputs` step rejects a non-main workflow ref, a malformed
SHA, an unknown operation, or an invalid operation/package combination before
checkout. It reports the reason without authentication, uploads, or Release
creation; correct the inputs and dispatch again.

Before authentication or uploads, the workflow checks:

- checkout at the exact release SHA, on main's history;
- matching workspace versions;
- a matching, nonempty release note file without placeholder markers;
- successful push runs of all three verification workflows at that SHA;
- an existing release tag, if any, pointing to that same SHA.

Dry-run runs the same validation for every operation. Publishing operations
invoke native `cargo publish --locked ... --dry-run`; `Create GitHub Release`
only checks the exact registry versions. Neither creates tags, uploads, or
GitHub Releases. A successful dry-run does not prove actual OIDC permissions or
upload availability.

For a new release, run `Publish workspace` again with **dry-run** disabled and
the **same full commit SHA**. It authenticates through Trusted Publishing and
runs native `cargo publish --workspace --locked`. Cargo owns dependency
ordering, package verification, and index polling.

After uploading, `cargo info <crate>@<version> --registry crates-io` checks every
workspace package from outside the checkout, so local packages and patches
cannot satisfy the check. Only then does `gh release create` publish the reviewed
note file verbatim, creating `v<version>` at the selected SHA. It does not move
existing tags or overwrite existing releases.

## Retry

Dispatch Publish from main with `commit` set to the original full SHA, as for
the first attempt. Do not select a newer main commit for the same version.

- If some uploads failed, select `Retry packages` and enter only the remaining
  names in `packages`, separated by spaces, for example `geam-cli geam`.
  Cargo receives one `--package` per name.
  It does not automatically skip published versions. If other packages are still
  missing, the final registry check fails without creating a GitHub Release.
- An upload error can occur after crates.io received the package. Check the
  upload log and exact published versions before choosing the retry packages.
- If all crates were uploaded, select `Create GitHub Release` and leave
  `packages` empty. This skips authentication and Cargo publish, checks all
  exact registry versions, then creates the GitHub Release. Its dry-run performs
  the checks without creating a tag or release.
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

All seven crates have been bootstrapped and configured for Trusted Publishing:

- repository owner: `panarch`
- repository: `geam`
- workflow: `publish.yml`
- environment: `crates-io`

Keep the workflow filename and environment stable. The environment allows main
only; crates.io requires Trusted Publishing for new versions. `id-token: write`
permits short-lived crates.io authentication, `actions: read` checks CI, and
`contents: write` permits the tag and GitHub Release. No long-lived registry
token or local-publish fallback is part of the regular release path.
