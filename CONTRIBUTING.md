# Contributing

Thank you for your interest in improving `rustweave`.

## Development setup

Install a stable Rust toolchain with `cargo`. The workspace uses the 2021 edition and declares its
minimum supported Rust version in the root `Cargo.toml`.

The repository contains two crates:

- `crates/rustweave-core`: parser, evaluator, runtime values, and standard library.
- `crates/rustweave`: command-line executable.

## Building and testing

```sh
cargo build --workspace
cargo test --workspace
cargo run -p rustweave -- '1 + 1'
```

The core crate includes behaviour tests for the language surface and the standard library. Please
add or update tests when changing behaviour.

## Code quality

Before submitting a change, run:

```sh
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo package --workspace
```

Keep public items documented and avoid changing existing behaviour without a test that describes
the intended result. New standard-library functions should follow the existing naming, argument
order, and error handling conventions.

## Reporting issues

Include a minimal script that reproduces the problem, the input values, the actual output, and the
expected output. For command-line problems, include the exact arguments used.

## Submitting changes

- Keep changes focused and include tests.
- Update `README.md` when the public API, command line, or supported functions change.
- Make sure the checks above pass before requesting review.

## License

By contributing, you agree that your contribution is dual licensed under the MIT and Apache-2.0
licenses, as described in the project README.
