// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the `dw::extra::Common` extension module.

#[path = "extra_common/arrays.rs"]
mod arrays;
#[path = "extra_common/collections.rs"]
mod collections;
#[path = "extra_common/functional.rs"]
mod functional;
#[path = "extra_common/null.rs"]
mod null;
#[path = "extra_common/numbers.rs"]
mod numbers;
#[path = "extra_common/objects.rs"]
mod objects;
#[path = "extra_common/strings.rs"]
mod strings;

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(script: &str) -> Result<String, rustweave_core::DwError> {
    rustweave_core::run(script, &[])
}
