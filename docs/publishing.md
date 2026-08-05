# Publishing

Geam is published as one crate. The manual `Geam: Publish crate` workflow uses
the same `cargo publish --locked` path for every release; it does not create a
Git tag or GitHub release.

## Authentication

The workflow runs in the repository's `crates-io` environment and uses this
crates.io Trusted Publisher identity:

- repository owner: `panarch`
- repository: `geam`
- workflow: `publish.yml`
- environment: `crates-io`

The environment restricts deployment branches to `main`. The workflow requests
a short-lived crates.io token through OIDC, so no long-lived registry token is
stored in GitHub.

## Release

1. Merge the version and dependency update into `main` and wait for Checks.
2. Run `Geam: Publish crate` from `main` and leave `dry_run` enabled. This
   verifies the package and Trusted Publisher authentication without uploading
   it.
3. Run the workflow again with `dry_run` disabled. It repeats the package
   verification and then runs `cargo publish --locked`.

The published version comes directly from the `geam` package in `Cargo.toml`.
Concurrent publish runs are serialized, and only `main` can publish.
