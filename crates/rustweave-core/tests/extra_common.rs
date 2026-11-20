// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for the `dw::extra::Common` extension module.

mod arrays;
mod numbers;
mod objects;

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

fn run_result(script: &str) -> Result<String, rustweave_core::DwError> {
    rustweave_core::run(script, &[])
}
