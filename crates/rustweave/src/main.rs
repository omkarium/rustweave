// SPDX-License-Identifier: MIT OR Apache-2.0
//! Command-line interface for the `rustweave` expression engine.
//!
//! `rustweave` reads a DataWeave-compatible expression or script, evaluates it with optional JSON
//! inputs, and prints pretty-printed JSON. It is a thin wrapper around the
//! [`rustweave_core`](https://crates.io/crates/rustweave-core) crate; the language and
//! standard-library surface is documented there.
//!
//! # Install
//!
//! ```text
//! cargo install rustweave
//! ```
//!
//! # Usage
//!
//! ```text
//! rustweave [run] [-i name=file.json]... (-f script.dwl | "<script>")
//! ```
//!
//! - A leading `run` is accepted and ignored.
//! - `-s` is accepted and ignored.
//! - `-i name=path` reads JSON from a file and binds it to `name`.
//! - `-f path` reads the script from a file.
//! - Any other argument is used as the script text.
//!
//! # Examples
//!
//! Run an expression directly:
//!
//! ```console
//! $ rustweave '[1, 2, 3] map $ * 2'
//! [
//!   2,
//!   4,
//!   6
//! ]
//! ```
//!
//! Bind a JSON file:
//!
//! ```console
//! $ rustweave -i feed=readings.json \
//!     'feed.devices map { code: $.code, reading: $.reading }'
//! ```
//!
//! Run a script file with a header:
//!
//! ```text
//! %dw 2.0
//! output application/json
//! ---
//! feed.devices
//!   filter $.reading > 20
//!   map { code: $.code, zone: $.zone }
//! ```
//!
//! ```console
//! $ rustweave -i feed=readings.json -f transform.dwl
//! ```
//!
//! `rustweave run ...` is equivalent when your tooling always emits a leading `run`.
//!
//! # Exit status
//!
//! - `0` on success; the JSON result is printed to stdout.
//! - `1` on an input, parse, or evaluation error; `[ERROR] message` is printed to stderr.
//!
//! # Scope
//!
//! The CLI targets JSON output. The parser, evaluator, exact decimal arithmetic, standard library,
//! and supported/unsupported feature list are provided by `rustweave-core`; see that crate's
//! documentation. XML, YAML, CSV, Multipart, Crypto, data formats, timers, and `dw::Runtime` are
//! not supported yet.
//!
//! # Trademarks
//!
//! DataWeave, MuleSoft, Anypoint, and Salesforce are trademarks or registered trademarks of
//! Salesforce, Inc. or its affiliates. `rustweave` is an independent project and is not
//! affiliated with, sponsored by, endorsed by, or approved by Salesforce or MuleSoft. The names
//! are used only to describe compatibility. See the repository `TRADEMARKS.md`.
//!
//! # License
//!
//! Licensed under either of the Apache License, Version 2.0 or the MIT license, at your option.

#![deny(dead_code)]

use std::env;
use std::fs;
use std::process::ExitCode;

/// Runs the CLI and prints either the result or an error message.
fn main() -> ExitCode {
    match run() {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("[ERROR] {message}");
            ExitCode::FAILURE
        }
    }
}

/// Parses command-line arguments, reads files, and evaluates the script.
fn run() -> Result<String, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut inputs: Vec<(String, String)> = Vec::new();
    let mut script: Option<String> = None;
    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];
        if index == 0 && arg == "run" {
            index += 1;
            continue;
        }
        if arg == "-s" {
            index += 1;
            continue;
        }
        if arg == "-i" {
            let spec = args
                .get(index + 1)
                .ok_or_else(|| "missing value for -i".to_string())?;
            let (name, path) = spec
                .split_once('=')
                .ok_or_else(|| "expected -i name=path".to_string())?;
            let text = fs::read_to_string(path)
                .map_err(|error| format!("cannot read input '{path}': {error}"))?;
            inputs.push((name.to_string(), text));
            index += 2;
            continue;
        }
        if arg == "-f" {
            let path = args
                .get(index + 1)
                .ok_or_else(|| "missing value for -f".to_string())?;
            let text = fs::read_to_string(path)
                .map_err(|error| format!("cannot read script '{path}': {error}"))?;
            script = Some(text);
            index += 2;
            continue;
        }
        script = Some(arg.clone());
        index += 1;
    }

    let script = script.ok_or_else(|| "missing script".to_string())?;
    let borrowed: Vec<(&str, &str)> = inputs
        .iter()
        .map(|(name, text)| (name.as_str(), text.as_str()))
        .collect();
    rustweave_core::run(&script, &borrowed).map_err(|error| error.to_string())
}
