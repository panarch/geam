# Publishing

Geam publishes seven lockstep crates from one workspace:

```text
geam-core + geam-macros
  -> geam-stdlib

geam-core + geam-macros + geam-stdlib
  -> geam-json
  -> geam-time

geam-core + geam-stdlib + geam-json + geam-time
  -> geam-cli

geam-core + geam-macros + built-ins + geam-cli
  -> geam
```

The root `geam` crate remains the public facade and owns the installable
`geam` binary. The other crates are internal ownership boundaries rather than
separately versioned products. Every release updates all seven package versions
and their exact internal dependency requirements together.

The manual `Geam: Publish crates` workflow uses one
`cargo publish --workspace --locked` command. Cargo derives dependency order
from the workspace graph, verifies each package against the packages assembled
before it, and polls the registry index after each upload. The workflow does not
create a Git tag or GitHub release, and it does not duplicate Cargo's ordering
or polling with a release script.

## Authentication

The workflow runs in the repository's `crates-io` environment. Each of the seven
crates uses this crates.io Trusted Publisher identity:

- repository owner: `panarch`
- repository: `geam`
- workflow: `publish.yml`
- environment: `crates-io`

The environment restricts deployment branches to `main`. An actual publish run
requests a short-lived crates.io token through OIDC, so no long-lived registry
token is stored in GitHub. A dry run does not authenticate because it only
checks the workspace and assembles package archives.

## Regular Release

1. Create a release branch and update `[workspace.package].version` plus every
   exact internal requirement in `[workspace.dependencies]` to the same version.
   Update the exact `geam` requirements in the standalone fixture providers and
   example providers, then reconcile the root and independently locked fixture
   and example `Cargo.lock` files without upgrading unrelated dependencies.
   Provider package versions and Gleam package versions remain independent.
2. Run the full workspace checks and
   `cargo package --workspace --locked --no-verify` locally.
3. Merge the release branch into `main` and wait for Checks.
4. Run `Geam: Publish crates` from `main` with `dry_run` enabled. This runs
   `cargo publish --workspace --locked --dry-run`, which assembles and verifies
   all seven packages without authentication or upload.
5. Run the workflow again with `dry_run` disabled. It authenticates once and
   Cargo publishes in dependency order:

```text
geam-core
geam-macros
geam-stdlib
geam-json
geam-time
geam-cli
geam
```

Concurrent publish runs are serialized, and only `main` can publish.

## First Workspace Release

The next workspace release is prepared at version `0.2.0`, including the
provider authoring API. All seven packages and their exact internal
requirements move together.

The six newly named internal crates (`geam-core`, `geam-stdlib`, `geam-json`,
`geam-time`, `geam-cli`, and `geam-macros`) do not yet exist on crates.io, so
they cannot have Trusted Publisher records before their first upload. Bootstrap
the first workspace release once from a locally authenticated checkout with the
same workspace command:

```sh
cargo publish --workspace --locked
```

After all seven crates exist, register `publish.yml` and the `crates-io`
environment as the Trusted Publisher for each package. Subsequent releases use
only the regular workflow above. The local credential is a one-time bootstrap
mechanism, not a second maintained release path.
