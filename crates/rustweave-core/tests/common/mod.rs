// SPDX-License-Identifier: MIT OR Apache-2.0
//! Shared inputs and helpers for the behaviour tests.

pub const PAYLOAD: &str = r####"{
  "orders": [
    { "id": 1, "customer": "ana",  "region": "north", "total": 120.50, "paid": true },
    { "id": 2, "customer": "ben",  "region": "south", "total": 80,     "paid": false },
    { "id": 3, "customer": "ana",  "region": "north", "total": 15.25,  "paid": true },
    { "id": 4, "customer": "cara", "region": "east",  "total": 300.10, "paid": true }
  ],
  "empty": [],
  "nothing": null
}"####;

/// Runs a script with the shared payload and returns its JSON output.
pub fn run(script: &str) -> String {
    rustweave_core::run(script, &[("payload", PAYLOAD)]).unwrap_or_else(|error| panic!("{error}"))
}

/// Runs a script and returns the result or error.
pub fn run_result(script: &str) -> Result<String, rustweave_core::DwError> {
    rustweave_core::run(script, &[("payload", PAYLOAD)])
}
