// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runtime values and lexical environments.

use std::rc::Rc;

use crate::ast::{Expr, Param};
use crate::value::Value;

/// A value that can appear while evaluating an expression.
#[derive(Clone)]
pub enum RValue {
    /// A plain data value.
    Val(Value),
    /// A regular-expression literal, before it is used as a value.
    Regex(String),
    /// A user-defined function.
    Func(Rc<Closure>),
    /// A set of overloaded user-defined functions.
    Overloads(Vec<Rc<Closure>>),
    /// A name resolved to a standard-library function.
    Builtin(&'static str),
}

/// A user-defined function with its captured environment.
pub struct Closure {
    pub params: Vec<Param>,
    pub body: Expr,
    pub env: Env,
}

impl From<Value> for RValue {
    /// Wraps a data value as [`RValue::Val`].
    fn from(value: Value) -> Self {
        RValue::Val(value)
    }
}

/// An immutable chain of lexical bindings.
#[derive(Clone, Default)]
pub struct Env(Option<Rc<Frame>>);

/// One binding in an environment chain.
struct Frame {
    name: String,
    value: RValue,
    parent: Env,
}

impl Env {
    /// Returns a new environment with `name` bound to `value`.
    pub fn extend(&self, name: impl Into<String>, value: RValue) -> Env {
        Env(Some(Rc::new(Frame {
            name: name.into(),
            value,
            parent: self.clone(),
        })))
    }

    /// Looks up a binding, searching inner frames first.
    pub fn lookup(&self, name: &str) -> Option<RValue> {
        let mut cursor = self;
        while let Some(frame) = &cursor.0 {
            if frame.name == name {
                return Some(frame.value.clone());
            }
            cursor = &frame.parent;
        }
        None
    }
}
