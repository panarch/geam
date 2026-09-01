# Develop Geam

This section is for contributors changing Geam itself. User workflows and
public execution contracts live in the top-level guides and
[technical reference](../reference/README.md); these documents define how the
repository verifies, reviews, synchronizes, and publishes those contracts.

## Start Here

- [Testing](testing.md) maps workspace, acceptance, example, package, and
  coverage checks to their responsibilities.
- [Test development](test-development.md) explains how to choose a test owner,
  design fixtures, investigate coverage, and keep scenarios local.
- [Review policy](review-policy.md) records structural, ownership, error,
  embedding, provider, and coverage rules that are not fully expressed by the
  compiler or test suite.

Run the default workspace suite with the pinned Rust, Gleam, and Erlang/OTP
toolchains before submitting a repository-wide change:

```sh
cargo test --locked
```

The testing guide lists narrower checks and the cases exercised separately in
GitHub Actions.

## Upstream And Releases

- [Upstream Gleam](upstream-gleam.md) records the exact compiler mirror,
  official package baselines, compatibility evidence, and manual sync policy.
- [Publishing](publishing.md) documents release preparation, artifact ownership,
  recovery operations, authentication, and GitHub Release creation.
- [Writing release notes](release-notes.md) defines the user-facing release-note
  format and attribution policy.

These workflows are intentionally explicit. Do not infer an upstream upgrade
from a Geam version bump or bypass reviewed release inputs with a local publish.
