// SPDX-License-Identifier: MIT OR Apache-2.0
//! An independent Rust implementation of a DataWeave-compatible expression language.
//!
//! `rustweave_core` parses and evaluates DataWeave expressions and returns JSON. It is the engine
//! behind the `rustweave` command-line crate. The evaluator is eager and supports exact decimal
//! arithmetic, JSON inputs and outputs, arrays, objects, lambdas, `do` blocks, `match`
//! expressions, regex literals, ranges, and a JSON-oriented standard library.
//!
//! # Quick start
//!
//! [`run`] is the one-shot API. Inputs are named JSON texts and the result is pretty-printed JSON:
//!
//! ```
//! let output = rustweave_core::run(
//!     "payload.orders map { id: $.id, total: $.total }",
//!     &[("payload", r#"{"orders":[{"id":1,"total":120.50},{"id":2,"total":80}]}"#)],
//! )?;
//! assert!(output.contains("\"total\": 120.5"));
//! # Ok::<(), rustweave_core::DwError>(())
//! ```
//!
//! Arithmetic is exact decimal arithmetic, and division keeps 34 significant digits:
//!
//! ```
//! let output = rustweave_core::run("[0.1 + 0.2, 1 / 3]", &[])?;
//! assert!(output.starts_with("[\n  0.3,\n  0.3333"));
//! # Ok::<(), rustweave_core::DwError>(())
//! ```
//!
//! # Collections and the standard library
//!
//! The language supports `map`, `filter`, `reduce`, `groupBy`, `orderBy`, joins, object
//! transforms, string and regex helpers, numeric helpers, and the `dw::Core`, `dw::core::Arrays`,
//! `dw::core::Strings`, `dw::core::Objects`, and `dw::core::Numbers` modules.
//!
//! ```
//! let output = rustweave_core::run(
//!     r#"%dw 2.0
//! import * from dw::core::Arrays
//! output application/json
//! ---
//! {
//!   total: payload.orders sumBy $.total,
//!   paid: payload.orders filter $.paid map $.customer,
//!   byRegion: payload.orders groupBy $.region
//! }"#,
//!     &[("payload", r#"{"orders":[
//!         {"customer":"ana","region":"north","total":120.5,"paid":true},
//!         {"customer":"ben","region":"south","total":80,"paid":false}
//!     ]}"#)],
//! )?;
//! println!("{output}");
//! # Ok::<(), rustweave_core::DwError>(())
//! ```
//!
//! # Expressions, `do`, and `match`
//!
//! The engine supports `if`/`else`, `default`, `as`, `is`, lambdas, `$`/`$$`, safe selectors,
//! ranges, string interpolation, array cons, dynamic keys, trailing commas, `do` blocks, and
//! `match`/`case` with literal, empty-array, `[head ~ tail]`, and `else` arms.
//!
//! ```
//! let output = rustweave_core::run(
//!     r#"do {
//!          var status = "paid"
//!          ---
//!          { label: status match { case "paid" -> "Paid" else -> "Unknown" } }
//!        }"#,
//!     &[],
//! )?;
//! assert_eq!(output, "{\n  \"label\": \"Paid\"\n}");
//! # Ok::<(), rustweave_core::DwError>(())
//! ```
//!
//! # Reusable interpreter and values
//!
//! [`Interp`] keeps variables between evaluations, and [`Value`], [`Number`], and [`json`] are
//! available for embedding:
//!
//! ```
//! use rustweave_core::{Interp, Number, Value};
//!
//! let interp = Interp::new().with_variable("x", Value::Number(Number::from_i64(41)));
//! assert_eq!(interp.run("x + 1", &[])?, "42");
//! # Ok::<(), rustweave_core::DwError>(())
//! ```
//!
//! ```
//! let value = rustweave_core::json::parse(r#"{"a":[1,2]}"#)?;
//! assert_eq!(
//!     rustweave_core::json::write(&value),
//!     "{\n  \"a\": [\n    1,\n    2\n  ]\n}"
//! );
//! # Ok::<(), rustweave_core::DwError>(())
//! ```
//!
//! Errors are returned as [`DwError`], not panics:
//!
//! ```
//! assert!(rustweave_core::run("notAFunction(1)", &[]).is_err());
//! ```
//!
//! # What is supported
//!
//! - DataWeave expressions with `%dw 2.0` headers, imports, `var`, `fun`, overloads and defaults.
//! - JSON input and pretty-printed JSON output.
//! - Exact decimal numbers, arrays, objects, lambdas, `do`, `match`, regex literals and ranges.
//! - The `dw::Core`, `dw::core::Arrays`, `dw::core::Strings`, `dw::core::Objects` and
//!   `dw::core::Numbers` function surface.
//! - A reusable [`Interp`] and direct [`Value`]/[`json`] access.
//!
//! # What is not supported yet
//!
//! - XML, CSV, YAML, dates/times, binary values, user-defined types, metadata, `update`, and
//!   `using`.
//! - `read`, `write`, `readUrl`, `dw::Runtime` (`try`/`orElse`/`fail`), and Mule/Anypoint
//!   connectors.
//! - Lazy or streaming evaluation. This engine evaluates eagerly.
//! - Object spread syntax.
//! - The exact text of error messages is not part of the public contract, and regex behaviour
//!   follows Rust's `regex` crate rather than `java.util.regex`.
//!
//! # Command line
//!
//! The [`rustweave`](https://crates.io/crates/rustweave) crate provides the `rustweave` binary:
//!
//! ```text
//! rustweave [run] [-i name=file.json]... (-f script.dwl | "<script>")
//! ```
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

pub mod json;

mod ast;
mod error;
mod eval;
mod lexer;
mod number;
mod parser;
mod runtime;
mod stdlib;
#[allow(dead_code)]
mod temporal;
pub mod value;

use std::rc::Rc;

pub use error::DwError;
pub use number::Number;
pub use value::Value;

use parser::HeaderItem;
use runtime::{Closure, Env, RValue};

/// A reusable interpreter that keeps variables between evaluations.
///
/// Use [`Interp::with_variable`] to seed values, then call [`Interp::eval`]
/// for a [`Value`] or [`Interp::run`] for JSON text.
#[derive(Clone, Default)]
pub struct Interp {
    globals: Env,
}

impl Interp {
    /// Creates an interpreter with no predefined variables.
    pub fn new() -> Self {
        Interp::default()
    }

    /// Returns a new interpreter with `value` bound to `name`.
    pub fn with_variable(mut self, name: &str, value: Value) -> Self {
        self.globals = self.globals.extend(name, RValue::Val(value));
        self
    }

    /// Parses and evaluates `script`, returning the resulting [`Value`].
    ///
    /// Each input is a name and a JSON text; the name becomes a variable in
    /// the script.
    ///
    /// # Errors
    ///
    /// Returns [`DwError`] for invalid JSON, syntax errors, or evaluation
    /// errors such as unresolved names or bad operand types.
    pub fn eval(&self, script: &str, inputs: &[(&str, &str)]) -> Result<Value, DwError> {
        let parsed = parser::parse_script(script)?;
        let mut env = self.globals.clone();
        for module in &parsed.imports {
            for (local, builtin) in stdlib::module_aliases(module) {
                env = env.extend(*local, RValue::Builtin(builtin));
            }
        }
        for (name, text) in inputs {
            let value = json::parse(text)
                .map_err(|error| DwError::new(format!("input '{name}': {error}")))?;
            env = env.extend(*name, RValue::Val(value));
        }
        for item in &parsed.items {
            match item {
                HeaderItem::Var(name, expr) => {
                    let value = eval::eval(expr, &env)?;
                    env = env.extend(name.clone(), value);
                }
                HeaderItem::Fun(def) => {
                    let closure = Rc::new(Closure {
                        params: def.params.clone(),
                        body: def.body.clone(),
                        env: env.clone(),
                    });
                    let value = match env.lookup(&def.name) {
                        Some(RValue::Func(existing)) => RValue::Overloads(vec![existing, closure]),
                        Some(RValue::Overloads(mut overloads)) => {
                            overloads.push(closure);
                            RValue::Overloads(overloads)
                        }
                        _ => RValue::Func(closure),
                    };
                    env = env.extend(def.name.clone(), value);
                }
            }
        }
        let value = eval::eval(&parsed.body, &env)?;
        eval::as_value(value)
    }

    /// Evaluates `script` and returns pretty-printed JSON text.
    ///
    /// # Errors
    ///
    /// Returns [`DwError`] under the same conditions as [`Interp::eval`].
    pub fn run(&self, script: &str, inputs: &[(&str, &str)]) -> Result<String, DwError> {
        self.eval(script, inputs).map(|value| json::write(&value))
    }
}

/// Evaluates a script with optional JSON inputs and returns pretty-printed JSON.
///
/// This is a convenience wrapper around [`Interp::run`].
///
/// # Errors
///
/// Returns [`DwError`] for invalid JSON, syntax errors, or evaluation errors.
pub fn run(script: &str, inputs: &[(&str, &str)]) -> Result<String, DwError> {
    Interp::new().run(script, inputs)
}
