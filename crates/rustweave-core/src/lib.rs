//! A DataWeave-compatible expression evaluator.
//!
//! The crate parses a script, evaluates it with optional JSON inputs, and
//! pretty-prints the result as JSON. It supports exact decimal arithmetic,
//! arrays, objects, lambdas, `do` blocks, `match` expressions, and a
//! JSON-oriented standard library.
//!
//! # Example
//!
//! ```
//! let output = rustweave_core::run("1 + 1", &[]).unwrap();
//! assert_eq!(output, "2");
//! ```
//!
//! Inputs are named JSON values available as variables in the script:
//!
//! ```
//! let output = rustweave_core::run(
//!     "payload.value * 2",
//!     &[("payload", r#"{"value": 21}"#)],
//! )
//! .unwrap();
//! assert_eq!(output, "42");
//! ```

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
