# External OTP service consumer

This fixture connects unmodified Gleam OTP source to an independent Rust provider
using Geam's public process service, native value, and typed callable contracts.
It is a verification consumer, not a published OTP implementation.

The [project](project) locks `gleam_otp` 1.3.0, `gleam_erlang` 1.3.0 and
`gleam_stdlib` 1.0.3. Tests acquire those versions with Gleam and check all 30 OTP
source-file hashes against [upstream.sha256](upstream.sha256). Downloaded package
source and build outputs are not tracked. The [native inventory](native-inventory.json)
records all 22 Erlang declarations; [CONTRACTS.md](CONTRACTS.md) maps each one to
its implementation and maintained tests.

The [provider](provider) is a separate Cargo workspace with its own lock. Its
profile-dependent component retains real typed child-start callbacks after their
creating calls return. Actors, supervisors and provider requests share the
producer's process identities and mailboxes. There is no second process engine.

The fixture exercises actor initialization, failure, timeout, system requests,
static `OneForOne` supervision with `Never` auto shutdown, and `SimpleOneForOne`
factory supervision. Restart budgets, child exit/restart, graceful and timed
shutdown, and domain cancellation are tested. It does not implement every OTP
strategy, auto-shutdown policy, debug facility, or BEAM service protocol. Those
are work for an external OTP package; this fixture establishes the required
public Geam boundaries and the concrete combinations listed in CONTRACTS.

From the repository root, with Rust and Gleam available:

```sh
cargo test --manifest-path tests/fixtures/otp_service/provider/Cargo.toml --locked
cargo fmt --manifest-path tests/fixtures/otp_service/provider/Cargo.toml --check
cargo clippy --manifest-path tests/fixtures/otp_service/provider/Cargo.toml --all-targets --locked -- -D warnings
cargo test -p geam --test provider_examples otp_service --locked
cargo test -p geam --test prepared_embedding process_consumers --locked
```

Owner unit tests stay beside their implementation in `provider/src/`; their
private test support only supplies execution and storage plumbing. The
[`original_otp` integration target](provider/tests/original_otp.rs) consumes the
provider's public component and runs the pinned source, including cancellation
and retained-callback lifetime. Both targets supply a manually advanced host
clock and run under the provider test command above. The root acceptance
tests use the normal CLI to build generated embedding and standalone consumers,
then execute relocated prepared binaries after removing their original source
and build directories. Both run the same provider and application entrypoint.

The [embedding application](embedding) runs dynamic and prepared modes by
default; `--prepared` runs only compiled execution data. Regenerate its managed
files with the checkout CLI's `geam embedding sync` from that directory. Generation
does not run application main or initialize an execution service.
