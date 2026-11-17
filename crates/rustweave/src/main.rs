//! Command-line interface for evaluating scripts.

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
