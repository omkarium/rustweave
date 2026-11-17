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

## Contribution boundaries

Do not submit work that puts the project or its users at risk:

- Do not copy code from another implementation, or reverse-engineer, decompile or disassemble a
  proprietary runtime, and do not include generated or decompiled source.
- Do not add third-party source code, vendored dependencies, binaries, jars, or research material.
  A small permissively licensed excerpt is acceptable only with attribution in
  `THIRD_PARTY_NOTICES.md` and maintainer approval.
- Do not use DataWeave, MuleSoft, Anypoint, Salesforce, or confusingly similar names as the crate,
  binary, repository, or command name. Descriptive references such as "DataWeave-compatible" are
  allowed; logos, icons, and other branding are not.
- Do not claim official affiliation, sponsorship, endorsement, certification, or support.
- Do not remove, weaken, or reword the license, trademark, or non-affiliation notices.
- Do not commit secrets, tokens, credentials, or private data.
- Do not change release settings, publish configuration, or release workflows; only the repository
  owner publishes.
- Do not modify tests to hide a failure. Fix the behaviour or discuss the spec change with the
  maintainer.
- Do not submit code you do not have the right to license under `MIT OR Apache-2.0`.
- Do not add a personal copyright header to every file. Keep the SPDX identifier and the project
  `NOTICE`.

## Submitting changes

- Keep changes focused and include tests.
- Update `README.md` when the public API, command line, or supported functions change.
- Make sure the checks above pass before requesting review.

## License

By contributing, you agree that your contribution is dual licensed under the MIT and Apache-2.0
licenses, as described in the project README.
