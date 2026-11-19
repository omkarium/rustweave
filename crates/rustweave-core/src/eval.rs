// SPDX-License-Identifier: MIT OR Apache-2.0
//! Expression evaluator and runtime operators.

use std::rc::Rc;

use crate::ast::{contains_placeholder, BinOp, Expr, InterpSegment, Param, UnOp};
use crate::error::DwError;
use crate::number::Number;
use crate::runtime::{Closure, Env, RValue};
use crate::stdlib;
use crate::value::Value;

/// Evaluates an expression in an environment.
pub fn eval(expr: &Expr, env: &Env) -> Result<RValue, DwError> {
    match expr {
        Expr::Null => Ok(RValue::Val(Value::Null)),
        Expr::Bool(value) => Ok(RValue::Val(Value::Bool(*value))),
        Expr::Number(number) => Ok(RValue::Val(Value::Number(number.clone()))),
        Expr::String(text) => Ok(RValue::Val(Value::String(text.clone()))),
        Expr::Literal(text) => crate::temporal::parse_literal(text)
            .map(RValue::Val)
            .ok_or_else(|| DwError::new(format!("invalid literal '|{text}|'"))),
        Expr::Regex(pattern) => Ok(RValue::Regex(pattern.clone())),
        Expr::Interp(parts) => {
            let mut text = String::new();
            for part in parts {
                match part {
                    InterpSegment::Text(segment) => text.push_str(segment),
                    InterpSegment::Expr(expr) => {
                        let value = as_value(eval(expr, env)?)?;
                        text.push_str(&value.key_string());
                    }
                }
            }
            Ok(RValue::Val(Value::String(text)))
        }
        Expr::Var(name) => {
            if let Some(value) = env.lookup(name) {
                return Ok(value);
            }
            if let Some(value) = stdlib::constant(name) {
                return Ok(RValue::Val(value));
            }
            if let Some(value) = crate::types::builtin(name) {
                return Ok(RValue::Val(Value::Type(Box::new(value))));
            }
            if let Some(builtin) = stdlib::builtin_name(name) {
                return Ok(RValue::Builtin(builtin));
            }
            Err(DwError::new(format!("unresolved name '{name}'")))
        }
        Expr::Array(items) => {
            let mut values = Vec::with_capacity(items.len());
            for item in items {
                values.push(as_value(eval(item, env)?)?);
            }
            Ok(RValue::Val(Value::Array(values)))
        }
        Expr::ArrayCons(head, tail) => {
            let head = as_value(eval(head, env)?)?;
            let tail = as_value(eval(tail, env)?)?;
            match tail {
                Value::Array(mut items) => {
                    items.insert(0, head);
                    Ok(RValue::Val(Value::Array(items)))
                }
                other => Err(DwError::new(format!(
                    "array cons requires an array, found {}",
                    other.type_name()
                ))),
            }
        }
        Expr::Object(entries) => {
            let mut values: Vec<(String, Value)> = Vec::new();
            for (key_expr, value_expr) in entries {
                let key = as_value(eval(key_expr, env)?)?.key_string();
                let value = as_value(eval(value_expr, env)?)?;
                values.push((key, value));
            }
            Ok(RValue::Val(Value::Object(values)))
        }
        Expr::Unary(op, inner) => {
            let value = as_value(eval(inner, env)?)?;
            match op {
                UnOp::Negate => match value {
                    Value::Number(number) => Ok(RValue::Val(Value::Number(number.neg()))),
                    other => Err(DwError::new(format!("cannot negate {}", other.type_name()))),
                },
                UnOp::Not => match value {
                    Value::Bool(value) => Ok(RValue::Val(Value::Bool(!value))),
                    other => Err(DwError::new(format!(
                        "cannot apply 'not' to {}",
                        other.type_name()
                    ))),
                },
            }
        }
        Expr::Binary(op, left, right) => eval_binary(*op, left, right, env),
        Expr::If(condition, then_branch, else_branch) => {
            let condition = as_value(eval(condition, env)?)?;
            if condition.truthy() {
                eval(then_branch, env)
            } else {
                eval(else_branch, env)
            }
        }
        Expr::Lambda(params, body) => Ok(RValue::Func(Rc::new(Closure {
            params: params.clone(),
            body: (**body).clone(),
            env: env.clone(),
        }))),
        Expr::Select(target, field) => {
            let target = as_value(eval(target, env)?)?;
            select_field(&target, field).map(RValue::Val)
        }
        Expr::SafeSelect(target, field) => {
            let target = as_value(eval(target, env)?)?;
            let exists = match &target {
                Value::Object(_) => target.get(field).is_some(),
                _ => false,
            };
            Ok(RValue::Val(Value::Bool(exists)))
        }
        Expr::Index(target, index) => {
            let target = as_value(eval(target, env)?)?;
            let index = as_value(eval(index, env)?)?;
            eval_index(&target, &index).map(RValue::Val)
        }
        Expr::SafeIndex(target, index) => {
            let target = as_value(eval(target, env)?)?;
            let index = as_value(eval(index, env)?)?;
            Ok(RValue::Val(Value::Bool(index_exists(&target, &index))))
        }
        Expr::Call(callee, args) => eval_call(callee, args, env),
        Expr::Infix(name, left, right) => {
            let call = Expr::Call(
                Box::new(Expr::Var(name.clone())),
                vec![(**left).clone(), (**right).clone()],
            );
            eval(&call, env)
        }
        Expr::As(inner, type_name, options) => {
            let value = as_value(eval(inner, env)?)?;
            let options = match options {
                Some(options) => Some(as_value(eval(options, env)?)?),
                None => None,
            };
            cast(value, type_name, options.as_ref()).map(RValue::Val)
        }
        Expr::Is(inner, type_name) => {
            let value = as_value(eval(inner, env)?)?;
            Ok(RValue::Val(Value::Bool(is_type(&value, type_name))))
        }
        Expr::Default(left, right) => {
            let value = as_value(eval(left, env)?)?;
            if value.is_null() {
                eval(right, env)
            } else {
                Ok(RValue::Val(value))
            }
        }
        Expr::Match(subject, arms) => eval_match(subject, arms, env),
        Expr::Do(declarations, body) => {
            let mut local = env.clone();
            for (name, value) in declarations {
                let value = eval(value, &local)?;
                local = local.extend(name.clone(), value);
            }
            eval(body, &local)
        }
    }
}

/// Selects a field from an object, maps over an array, or propagates null.
fn select_field(target: &Value, field: &str) -> Result<Value, DwError> {
    match target {
        Value::Object(_) => Ok(target.get(field).cloned().unwrap_or(Value::Null)),
        Value::Date(_)
        | Value::DateTime(_)
        | Value::LocalDateTime(_)
        | Value::LocalTime(_)
        | Value::Time(_)
        | Value::Period(_) => target
            .temporal_field(field)
            .ok_or_else(|| DwError::new(format!("unknown date field '{field}'"))),
        Value::Array(items) => Ok(Value::Array(
            items
                .iter()
                .map(|item| match item {
                    Value::Object(_) => item.get(field).cloned().unwrap_or(Value::Null),
                    _ => Value::Null,
                })
                .collect(),
        )),
        Value::Null => Ok(Value::Null),
        other => Err(DwError::new(format!(
            "cannot select '{}' from {}",
            field,
            other.type_name()
        ))),
    }
}

/// Applies an index, including object keys, negative positions, and ranges.
fn eval_index(target: &Value, index: &Value) -> Result<Value, DwError> {
    match (target, index) {
        (Value::Object(_), Value::String(key)) => {
            Ok(target.get(key).cloned().unwrap_or(Value::Null))
        }
        (Value::Object(_), other) => Ok(target
            .get(&other.key_string())
            .cloned()
            .unwrap_or(Value::Null)),
        (_, Value::Array(indices)) => {
            if indices.is_empty() {
                return Ok(Value::Array(Vec::new()));
            }
            let length = target
                .len()
                .ok_or_else(|| DwError::new(format!("cannot index {}", target.type_name())))?
                as i64;
            let start = indices[0]
                .as_number()
                .and_then(|number| number.to_i64())
                .ok_or_else(|| DwError::new("array index must be an integer"))?;
            let end = indices[indices.len() - 1]
                .as_number()
                .and_then(|number| number.to_i64())
                .ok_or_else(|| DwError::new("array index must be an integer"))?;
            let start = if start < 0 { length + start } else { start };
            let end = if end < 0 { length + end } else { end };
            if start < 0 || end < 0 || start >= length || end >= length {
                return Ok(Value::Null);
            }
            let mut out = Vec::new();
            if start <= end {
                for position in start..=end {
                    if let Some(value) = target.get_index(position) {
                        out.push(value);
                    }
                }
            } else {
                let mut position = start;
                while position >= end {
                    if let Some(value) = target.get_index(position) {
                        out.push(value);
                    }
                    position -= 1;
                }
            }
            if matches!(target, Value::String(_)) {
                Ok(Value::String(out.iter().map(Value::key_string).collect()))
            } else {
                Ok(Value::Array(out))
            }
        }
        (_, Value::Number(number)) => {
            let index = number
                .to_i64()
                .ok_or_else(|| DwError::new("array index must be an integer"))?;
            target
                .get_index(index)
                .ok_or_else(|| DwError::new(format!("cannot index {}", target.type_name())))
        }
        (_, other) => Err(DwError::new(format!(
            "cannot index {} with {}",
            target.type_name(),
            other.type_name()
        ))),
    }
}

/// Reports whether an index is present for safe indexing.
fn index_exists(target: &Value, index: &Value) -> bool {
    match (target, index) {
        (Value::Object(_), Value::String(key)) => target.get(key).is_some(),
        (Value::Object(_), other) => target.get(&other.key_string()).is_some(),
        (Value::Array(items), Value::Number(number)) => number
            .to_i64()
            .map(|index| {
                let index = if index < 0 {
                    items.len() as i64 + index
                } else {
                    index
                };
                index >= 0 && (index as usize) < items.len()
            })
            .unwrap_or(false),
        (Value::String(text), Value::Number(number)) => number
            .to_i64()
            .map(|index| {
                let len = text.chars().count() as i64;
                let index = if index < 0 { len + index } else { index };
                index >= 0 && index < len
            })
            .unwrap_or(false),
        _ => false,
    }
}

/// Evaluates the first matching arm of a `match` expression.
fn eval_match(subject: &Expr, arms: &[(Expr, Expr)], env: &Env) -> Result<RValue, DwError> {
    let value = as_value(eval(subject, env)?)?;
    for (pattern, body) in arms {
        if let Some(bindings) = match_pattern(pattern, &value) {
            let mut arm_env = env.clone();
            for (name, value) in bindings {
                arm_env = arm_env.extend(name, RValue::Val(value));
            }
            return eval(body, &arm_env);
        }
    }
    Err(DwError::new("no case matched"))
}

/// Matches one pattern against a value and returns any variable bindings.
fn match_pattern(pattern: &Expr, value: &Value) -> Option<Vec<(String, Value)>> {
    match pattern {
        Expr::Var(name) if name == "_" => Some(Vec::new()),
        Expr::Var(name) => Some(vec![(name.clone(), value.clone())]),
        Expr::Null => matches!(value, Value::Null).then(Vec::new),
        Expr::Bool(expected) => match value {
            Value::Bool(actual) if actual == expected => Some(Vec::new()),
            _ => None,
        },
        Expr::Number(expected) => match value {
            Value::Number(actual) if actual == expected => Some(Vec::new()),
            _ => None,
        },
        Expr::String(expected) => match value {
            Value::String(actual) if actual == expected => Some(Vec::new()),
            _ => None,
        },
        Expr::Array(items) => match value {
            Value::Array(values) if values.len() == items.len() => {
                let mut bindings = Vec::new();
                for (item, value) in items.iter().zip(values.iter()) {
                    bindings.extend(match_pattern(item, value)?);
                }
                Some(bindings)
            }
            _ => None,
        },
        Expr::ArrayCons(head, tail) => match value {
            Value::Array(values) if !values.is_empty() => {
                let mut bindings = match_pattern(head, &values[0])?;
                bindings.extend(match_pattern(tail, &Value::Array(values[1..].to_vec()))?);
                Some(bindings)
            }
            _ => None,
        },
        _ => None,
    }
}

/// Evaluates a call, wrapping implicit-lambda arguments for built-in functions.
fn eval_call(callee: &Expr, args: &[Expr], env: &Env) -> Result<RValue, DwError> {
    let callee = eval(callee, env)?;
    match callee {
        RValue::Builtin(name) => {
            let positions = stdlib::lambda_positions(name);
            let mut values = Vec::with_capacity(args.len());
            for (index, arg) in args.iter().enumerate() {
                let expr = if positions.contains(&index)
                    && !matches!(arg, Expr::Lambda(..))
                    && contains_placeholder(arg)
                {
                    Expr::Lambda(
                        vec![
                            Param {
                                name: "$".to_string(),
                                default: None,
                                type_name: None,
                            },
                            Param {
                                name: "$$".to_string(),
                                default: None,
                                type_name: None,
                            },
                        ],
                        Box::new(arg.clone()),
                    )
                } else {
                    arg.clone()
                };
                values.push(eval(&expr, env)?);
            }
            let value = stdlib::call(name, &values)?;
            Ok(RValue::Val(value))
        }
        RValue::Func(closure) => {
            let values = eval_args(args, env)?;
            call_closure(&closure, &values)
        }
        RValue::Overloads(overloads) => {
            let values = eval_args(args, env)?;
            let closure = select_overload(&overloads, &values)
                .ok_or_else(|| DwError::new("no matching function overload"))?;
            call_closure(closure, &values)
        }
        RValue::Regex(_) | RValue::Val(_) => Err(DwError::new("value is not callable")),
    }
}

/// Evaluates every argument expression in order.
fn eval_args(args: &[Expr], env: &Env) -> Result<Vec<RValue>, DwError> {
    let mut values = Vec::with_capacity(args.len());
    for arg in args {
        values.push(eval(arg, env)?);
    }
    Ok(values)
}

/// Calls a closure, applying defaults for omitted arguments.
fn call_closure(closure: &Closure, args: &[RValue]) -> Result<RValue, DwError> {
    let mut env = closure.env.clone();
    for (index, param) in closure.params.iter().enumerate() {
        let value = match args.get(index) {
            Some(value) => value.clone(),
            None => match &param.default {
                Some(default) => eval(default, &env)?,
                None => RValue::Val(Value::Null),
            },
        };
        env = env.extend(param.name.clone(), value);
    }
    eval(&closure.body, &env)
}

/// Calls a runtime function value with already evaluated arguments.
pub fn call_value(callee: &RValue, args: &[RValue]) -> Result<RValue, DwError> {
    match callee {
        RValue::Builtin(name) => {
            let value = stdlib::call(name, args)?;
            Ok(RValue::Val(value))
        }
        RValue::Func(closure) => call_closure(closure, args),
        RValue::Overloads(overloads) => {
            let closure = select_overload(overloads, args)
                .ok_or_else(|| DwError::new("no matching function overload"))?;
            call_closure(closure, args)
        }
        RValue::Regex(_) | RValue::Val(_) => Err(DwError::new("value is not callable")),
    }
}

/// Chooses the best overload for the supplied arguments.
fn select_overload<'a>(overloads: &'a [Rc<Closure>], args: &[RValue]) -> Option<&'a Rc<Closure>> {
    let mut best: Option<(&Rc<Closure>, i32)> = None;
    for closure in overloads {
        let required = closure
            .params
            .iter()
            .filter(|param| param.default.is_none())
            .count();
        if args.len() < required || args.len() > closure.params.len() {
            continue;
        }
        let mut score = 0i32;
        let mut compatible = true;
        for (arg, param) in args.iter().zip(closure.params.iter()) {
            match &param.type_name {
                Some(type_name) => {
                    if type_matches(arg, type_name) {
                        score += 2;
                    } else {
                        compatible = false;
                        break;
                    }
                }
                None => score += 1,
            }
        }
        if compatible
            && best
                .map(|(_, best_score)| score > best_score)
                .unwrap_or(true)
        {
            best = Some((closure, score));
        }
    }
    best.map(|(closure, _)| closure)
}

/// Reports whether a runtime value satisfies a declared parameter type.
fn type_matches(value: &RValue, type_name: &str) -> bool {
    match type_name {
        "Any" => true,
        "Function" => matches!(
            value,
            RValue::Func(_) | RValue::Overloads(_) | RValue::Builtin(_)
        ),
        "Regex" => matches!(value, RValue::Regex(_)),
        _ => match value {
            RValue::Val(value) => is_type(value, type_name),
            _ => false,
        },
    }
}

/// Calls a function and requires a plain value result.
pub fn invoke(callee: &RValue, args: &[RValue]) -> Result<Value, DwError> {
    match call_value(callee, args)? {
        RValue::Val(value) => Ok(value),
        _ => Err(DwError::new("function returned a function")),
    }
}

/// Returns the default value for a closure parameter, when present.
pub fn default_value(callee: &RValue, index: usize) -> Result<Option<Value>, DwError> {
    if let RValue::Func(closure) = callee {
        if let Some(param) = closure.params.get(index) {
            if let Some(default) = &param.default {
                return Ok(Some(as_value(eval(default, &closure.env)?)?));
            }
        }
    }
    Ok(None)
}

/// Converts a runtime value to a plain value, rejecting functions and regexes.
pub fn as_value(value: RValue) -> Result<Value, DwError> {
    match value {
        RValue::Val(value) => Ok(value),
        RValue::Regex(_) => Err(DwError::new("expected a value, found a regular expression")),
        _ => Err(DwError::new("expected a value, found a function")),
    }
}

/// Evaluates a binary operator, with short-circuiting for `and` and `or`.
fn eval_binary(op: BinOp, left: &Expr, right: &Expr, env: &Env) -> Result<RValue, DwError> {
    if op == BinOp::And {
        let left = as_value(eval(left, env)?)?;
        if !left.truthy() {
            return Ok(RValue::Val(Value::Bool(false)));
        }
        let right = as_value(eval(right, env)?)?;
        return Ok(RValue::Val(Value::Bool(right.truthy())));
    }
    if op == BinOp::Or {
        let left = as_value(eval(left, env)?)?;
        if left.truthy() {
            return Ok(RValue::Val(Value::Bool(true)));
        }
        let right = as_value(eval(right, env)?)?;
        return Ok(RValue::Val(Value::Bool(right.truthy())));
    }

    let left = as_value(eval(left, env)?)?;
    let right = as_value(eval(right, env)?)?;
    match op {
        BinOp::Add => add(left, right),
        BinOp::Subtract => subtract(left, right),
        BinOp::Multiply => numeric_binary(&left, &right, |a, b| a.mul(b)),
        BinOp::Divide => match (numeric_operand(&left), numeric_operand(&right)) {
            (Some(a), Some(b)) => a
                .div(&b)
                .map(|n| RValue::Val(Value::Number(n)))
                .ok_or_else(|| DwError::new("division by zero")),
            _ => Err(DwError::new("division requires numbers")),
        },
        BinOp::Modulo => match (numeric_operand(&left), numeric_operand(&right)) {
            (Some(a), Some(b)) => match a.rem(&b) {
                Some(number) => Ok(RValue::Val(Value::Number(number))),
                None => Err(DwError::new("division by zero")),
            },
            _ => Err(DwError::new("modulo requires numbers")),
        },
        BinOp::Concat => concat(left, right),
        BinOp::Append => match left {
            Value::Array(mut items) => {
                items.push(right);
                Ok(RValue::Val(Value::Array(items)))
            }
            other => Err(DwError::new(format!(
                "cannot append to {}",
                other.type_name()
            ))),
        },
        BinOp::Difference => difference(left, right),
        BinOp::Equal => Ok(RValue::Val(Value::Bool(
            left.unwrap_formatted() == right.unwrap_formatted(),
        ))),
        BinOp::NotEqual => Ok(RValue::Val(Value::Bool(
            left.unwrap_formatted() != right.unwrap_formatted(),
        ))),
        BinOp::Less | BinOp::LessEqual | BinOp::Greater | BinOp::GreaterEqual => {
            let ordering = compare_values(&left, &right).ok_or_else(|| {
                DwError::new(format!(
                    "cannot compare {} with {}",
                    left.type_name(),
                    right.type_name()
                ))
            })?;
            let result = match op {
                BinOp::Less => ordering.is_lt(),
                BinOp::LessEqual => ordering.is_le(),
                BinOp::Greater => ordering.is_gt(),
                BinOp::GreaterEqual => ordering.is_ge(),
                _ => unreachable!(),
            };
            Ok(RValue::Val(Value::Bool(result)))
        }
        BinOp::And | BinOp::Or => unreachable!(),
    }
}

fn period_default_number(period: &crate::temporal::Period) -> Number {
    if period.date_based {
        Number::from_i64(period.years * 12 + period.months)
    } else {
        period.time_seconds.clone()
    }
}

fn period_plus(left: &crate::temporal::Period, right: &crate::temporal::Period) -> Number {
    period_default_number(left).add(&period_default_number(right))
}

fn local_time_seconds(time: &crate::temporal::LocalTime) -> i64 {
    time.hour as i64 * 3600 + time.minute as i64 * 60 + time.second as i64
}

fn time_seconds(time: &crate::temporal::Time) -> Number {
    let base = local_time_seconds(&time.time) - time.zone.seconds() as i64;
    let mut value = Number::from_i64(base);
    if time.time.nano != 0 {
        if let Some(fraction) =
            Number::from_i64(time.time.nano as i64).div(&Number::from_i64(1_000_000_000))
        {
            value = value.add(&fraction);
        }
    }
    value
}

fn add_period(
    value: &Value,
    period: &crate::temporal::Period,
    subtract: bool,
) -> Result<Option<Value>, DwError> {
    let result = match value {
        Value::Date(date) => {
            let date = if subtract {
                crate::temporal::subtract_period_from_date(date, period)?
            } else {
                crate::temporal::add_period_to_date(date, period)?
            };
            Value::Date(date)
        }
        Value::LocalTime(time) => {
            let time = if subtract {
                let mut negated = period.clone();
                negated.years = -negated.years;
                negated.months = -negated.months;
                negated.days = -negated.days;
                negated.time_seconds = negated.time_seconds.neg();
                crate::temporal::add_period_to_local_time(time, &negated)?
            } else {
                crate::temporal::add_period_to_local_time(time, period)?
            };
            Value::LocalTime(time)
        }
        Value::Time(time) => {
            let time = if subtract {
                let mut negated = period.clone();
                negated.years = -negated.years;
                negated.months = -negated.months;
                negated.days = -negated.days;
                negated.time_seconds = negated.time_seconds.neg();
                crate::temporal::add_period_to_time(time, &negated)?
            } else {
                crate::temporal::add_period_to_time(time, period)?
            };
            Value::Time(time)
        }
        Value::LocalDateTime(value) => {
            let value = if subtract {
                crate::temporal::subtract_period_from_local_date_time(value, period)?
            } else {
                crate::temporal::add_period_to_local_date_time(value, period)?
            };
            Value::LocalDateTime(value)
        }
        Value::DateTime(value) => {
            let value = if subtract {
                crate::temporal::subtract_period_from_date_time(value, period)?
            } else {
                crate::temporal::add_period_to_date_time(value, period)?
            };
            Value::DateTime(value)
        }
        _ => return Ok(None),
    };
    Ok(Some(result))
}

/// Implements `+`, including array append and numeric-string coercion.
fn add(left: Value, right: Value) -> Result<RValue, DwError> {
    // Arithmetic consumes the ISO value and produces a fresh unformatted result.
    let left = left.unwrap_formatted().clone();
    let right = right.unwrap_formatted().clone();
    if let Value::Array(mut items) = left {
        items.push(right);
        return Ok(RValue::Val(Value::Array(items)));
    }
    if let (Value::Period(a), Value::Period(b)) = (&left, &right) {
        return Ok(RValue::Val(Value::Number(period_plus(a, b))));
    }
    if let Value::Period(period) = &right {
        if let Some(value) = add_period(&left, period, false)? {
            return Ok(RValue::Val(value));
        }
    }
    if let Value::Period(period) = &left {
        if let Some(value) = add_period(&right, period, false)? {
            return Ok(RValue::Val(value));
        }
    }
    if matches!(right, Value::Array(_)) {
        return Err(DwError::new("cannot add an array to a scalar"));
    }
    if let (Some(a), Some(b)) = (numeric_operand(&left), numeric_operand(&right)) {
        return Ok(RValue::Val(Value::Number(a.add(&b))));
    }
    Err(DwError::new(format!(
        "cannot add {} and {}",
        left.type_name(),
        right.type_name()
    )))
}

/// Implements `-`, including array removal and numeric-string coercion.
fn subtract(left: Value, right: Value) -> Result<RValue, DwError> {
    // Arithmetic consumes the ISO value and produces a fresh unformatted result.
    let left = left.unwrap_formatted().clone();
    let right = right.unwrap_formatted().clone();
    if let Value::Array(items) = left {
        let filtered = items
            .into_iter()
            .filter(|item| *item != right)
            .collect::<Vec<_>>();
        return Ok(RValue::Val(Value::Array(filtered)));
    }
    match (&left, &right) {
        (Value::Date(a), Value::Date(b)) => {
            let days = crate::temporal::date_difference(b, a)?;
            return Ok(RValue::Val(Value::Period(
                crate::temporal::duration_period(Number::from_i64(days * 86_400)),
            )));
        }
        (Value::DateTime(a), Value::DateTime(b)) => {
            let seconds = crate::temporal::date_time_difference(b, a)?;
            return Ok(RValue::Val(Value::Period(
                crate::temporal::duration_period(seconds),
            )));
        }
        (Value::LocalDateTime(a), Value::LocalDateTime(b)) => {
            let seconds = crate::temporal::local_date_time_difference(b, a)?;
            return Ok(RValue::Val(Value::Period(
                crate::temporal::duration_period(seconds),
            )));
        }
        (Value::Time(a), Value::Time(b)) => {
            let left = time_seconds(a);
            let right = time_seconds(b);
            return Ok(RValue::Val(Value::Period(
                crate::temporal::duration_period(left.sub(&right)),
            )));
        }
        (Value::LocalTime(a), Value::LocalTime(b)) => {
            let left = Number::from_i64(local_time_seconds(a));
            let right = Number::from_i64(local_time_seconds(b));
            return Ok(RValue::Val(Value::Period(
                crate::temporal::duration_period(left.sub(&right)),
            )));
        }
        (Value::Period(a), Value::Period(b)) => {
            return Ok(RValue::Val(Value::Number(
                period_default_number(a).sub(&period_default_number(b)),
            )));
        }
        (_, Value::Period(period)) => {
            if let Some(value) = add_period(&left, period, true)? {
                return Ok(RValue::Val(value));
            }
        }
        _ => {}
    }
    match (numeric_operand(&left), numeric_operand(&right)) {
        (Some(a), Some(b)) => Ok(RValue::Val(Value::Number(a.sub(&b)))),
        _ => Err(DwError::new(format!(
            "cannot subtract {} from {}",
            right.type_name(),
            left.type_name()
        ))),
    }
}

/// Applies a numeric operation to two values that can be coerced to numbers.
fn numeric_binary<F>(left: &Value, right: &Value, f: F) -> Result<RValue, DwError>
where
    F: FnOnce(&Number, &Number) -> Number,
{
    match (numeric_operand(left), numeric_operand(right)) {
        (Some(a), Some(b)) => Ok(RValue::Val(Value::Number(f(&a, &b)))),
        _ => Err(DwError::new("arithmetic requires numbers")),
    }
}

/// Returns a number for numeric values and numeric strings.
fn numeric_operand(value: &Value) -> Option<Number> {
    match value {
        Value::Number(number) => Some(number.clone()),
        Value::String(text) => Number::parse(text),
        _ => None,
    }
}

/// Implements `--` for arrays and objects.
fn difference(left: Value, right: Value) -> Result<RValue, DwError> {
    match left {
        Value::Array(items) => {
            let removed = match &right {
                Value::Array(values) => values.clone(),
                other => vec![other.clone()],
            };
            Ok(RValue::Val(Value::Array(
                items
                    .into_iter()
                    .filter(|item| !removed.iter().any(|value| value == item))
                    .collect(),
            )))
        }
        Value::Object(entries) => {
            let keys = match &right {
                Value::Array(values) => values.iter().map(Value::key_string).collect::<Vec<_>>(),
                other => vec![other.key_string()],
            };
            Ok(RValue::Val(Value::Object(
                entries
                    .into_iter()
                    .filter(|(key, _)| !keys.iter().any(|removed| removed == key))
                    .collect(),
            )))
        }
        other => Err(DwError::new(format!(
            "cannot subtract from {}",
            other.type_name()
        ))),
    }
}

/// Concatenates two values using `++` semantics.
pub fn concat_values(left: Value, right: Value) -> Result<Value, DwError> {
    match concat(left, right)? {
        RValue::Val(value) => Ok(value),
        _ => Err(DwError::new("concatenation produced a function")),
    }
}

/// Removes values using `--` semantics.
pub fn difference_values(left: Value, right: Value) -> Result<Value, DwError> {
    match difference(left, right)? {
        RValue::Val(value) => Ok(value),
        _ => Err(DwError::new("difference produced a function")),
    }
}

/// Implements `++` for arrays, objects, and scalars.
fn concat(left: Value, right: Value) -> Result<RValue, DwError> {
    let left = left.unwrap_formatted().clone();
    let right = right.unwrap_formatted().clone();
    if matches!(left, Value::Null) || matches!(right, Value::Null) {
        return Err(DwError::new("cannot concatenate null"));
    }
    match (&left, &right) {
        (Value::Date(date), Value::LocalTime(time)) => {
            return Ok(RValue::Val(Value::LocalDateTime(
                crate::temporal::LocalDateTime {
                    date: *date,
                    time: *time,
                },
            )));
        }
        (Value::LocalTime(time), Value::Date(date)) => {
            return Ok(RValue::Val(Value::LocalDateTime(
                crate::temporal::LocalDateTime {
                    date: *date,
                    time: *time,
                },
            )));
        }
        (Value::Date(date), Value::Time(time)) => {
            return Ok(RValue::Val(Value::DateTime(crate::temporal::DateTime {
                date: *date,
                time: time.time,
                zone: time.zone.clone(),
            })));
        }
        (Value::Time(time), Value::Date(date)) => {
            return Ok(RValue::Val(Value::DateTime(crate::temporal::DateTime {
                date: *date,
                time: time.time,
                zone: time.zone.clone(),
            })));
        }
        _ => {}
    }
    match (left, right) {
        (Value::Array(mut a), Value::Array(b)) => {
            a.extend(b);
            Ok(RValue::Val(Value::Array(a)))
        }
        (Value::Object(mut a), Value::Object(b)) => {
            a.extend(b);
            Ok(RValue::Val(Value::Object(a)))
        }
        (Value::String(a), b) if is_scalar(&b) => {
            Ok(RValue::Val(Value::String(a + &b.key_string())))
        }
        (a, Value::String(b)) if is_scalar(&a) => {
            Ok(RValue::Val(Value::String(a.key_string() + &b)))
        }
        (Value::Number(a), Value::Number(b)) => {
            Ok(RValue::Val(Value::String(format!("{}{}", a, b))))
        }
        (Value::Number(a), Value::Bool(b)) => Ok(RValue::Val(Value::String(format!("{}{}", a, b)))),
        (Value::Bool(a), Value::Number(b)) => Ok(RValue::Val(Value::String(format!("{}{}", a, b)))),
        (Value::Bool(a), Value::Bool(b)) => Ok(RValue::Val(Value::String(format!("{}{}", a, b)))),
        (a, b) => Err(DwError::new(format!(
            "cannot concatenate {} and {}",
            a.type_name(),
            b.type_name()
        ))),
    }
}

/// Reports whether a value can take part in scalar concatenation.
fn is_scalar(value: &Value) -> bool {
    matches!(value, Value::String(_) | Value::Number(_) | Value::Bool(_))
}

/// Compares two values, coercing numeric strings to numbers.
fn compare_values(left: &Value, right: &Value) -> Option<std::cmp::Ordering> {
    let left = left.unwrap_formatted();
    let right = right.unwrap_formatted();
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Some(a.cmp(b)),
        (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
        (Value::Bool(a), Value::Bool(b)) => Some(a.cmp(b)),
        (Value::Date(a), Value::Date(b)) => a.partial_cmp(b),
        (Value::DateTime(a), Value::DateTime(b)) => a.partial_cmp(b),
        (Value::LocalDateTime(a), Value::LocalDateTime(b)) => a.partial_cmp(b),
        (Value::LocalTime(a), Value::LocalTime(b)) => a.partial_cmp(b),
        (Value::Time(a), Value::Time(b)) => a.partial_cmp(b),
        (Value::Number(a), Value::String(b)) => Number::parse(b).map(|b| a.cmp(&b)),
        (Value::String(a), Value::Number(b)) => Number::parse(a).map(|a| a.cmp(b)),
        _ => None,
    }
}

/// Tests a value against a DataWeave type name.
fn is_type(value: &Value, type_name: &str) -> bool {
    let value = value.unwrap_formatted();
    let base = type_name.split('{').next().unwrap_or(type_name).trim();
    match base {
        "Any" => true,
        "Number" => matches!(value, Value::Number(_)),
        "Integer" => value
            .as_number()
            .map(|number| number.is_integer())
            .unwrap_or(false),
        "Decimal" => value
            .as_number()
            .map(|number| !number.is_integer())
            .unwrap_or(false),
        "String" => matches!(value, Value::String(_)),
        "Boolean" => matches!(value, Value::Bool(_)),
        "Array" => matches!(value, Value::Array(_)),
        "Object" => matches!(value, Value::Object(_)),
        "Null" => matches!(value, Value::Null),
        "Date" => matches!(value, Value::Date(_)),
        "DateTime" => matches!(value, Value::DateTime(_)),
        "LocalDateTime" => matches!(value, Value::LocalDateTime(_)),
        "LocalTime" => matches!(value, Value::LocalTime(_)),
        "Time" => matches!(value, Value::Time(_)),
        "TimeZone" => matches!(value, Value::TimeZone(_)),
        "Period" => matches!(value, Value::Period(_)),
        "Binary" => matches!(value, Value::Binary(_)),
        "Regex" => matches!(value, Value::Regex(_)),
        "Uri" | "URI" => matches!(value, Value::Uri(_)),
        "Key" => matches!(value, Value::Key(_)),
        "Type" => matches!(value, Value::Type(_)),
        _ => false,
    }
}

fn option_text(options: Option<&Value>, key: &str) -> Option<String> {
    options
        .and_then(|value| value.get(key))
        .and_then(|value| match value {
            Value::String(text) => Some(text.clone()),
            _ => None,
        })
}

fn temporal_string(value: &Value, format: Option<&str>) -> String {
    // String casts deliberately ignore any format stored by an earlier parse.
    let value = value.unwrap_formatted();
    let pattern = format.unwrap_or("");
    match value {
        Value::Date(date) => {
            if pattern.is_empty() {
                crate::temporal::format_date(date)
            } else {
                crate::temporal::format_pattern(Some(date), None, None, pattern)
            }
        }
        Value::DateTime(date) => {
            if pattern.is_empty() {
                crate::temporal::format_date_time(date)
            } else {
                crate::temporal::format_pattern(
                    Some(&date.date),
                    Some(&date.time),
                    Some(&date.zone),
                    pattern,
                )
            }
        }
        Value::LocalDateTime(date) => {
            if pattern.is_empty() {
                crate::temporal::format_local_date_time(date)
            } else {
                crate::temporal::format_pattern(Some(&date.date), Some(&date.time), None, pattern)
            }
        }
        Value::LocalTime(time) => {
            if pattern.is_empty() {
                crate::temporal::format_local_time(time)
            } else {
                crate::temporal::format_pattern(None, Some(time), None, pattern)
            }
        }
        Value::Time(time) => {
            if pattern.is_empty() {
                crate::temporal::format_time(time)
            } else {
                crate::temporal::format_pattern(None, Some(&time.time), Some(&time.zone), pattern)
            }
        }
        Value::Period(period) => crate::temporal::format_period(period),
        Value::TimeZone(zone) => zone.label(),
        Value::Regex(pattern) => pattern.clone(),
        Value::Uri(text) | Value::Key(text) => text.clone(),
        other => crate::json::write(other),
    }
}

fn pattern_from(options: Option<&Value>) -> Option<String> {
    option_text(options, "format").or_else(|| option_text(options, "unit"))
}

fn formatted_display(input: &Value, result: &Value, format: Option<&str>) -> Option<String> {
    let pattern = format.filter(|pattern| !pattern.is_empty())?;
    match input {
        // Parsing a string keeps the original text exactly as supplied.
        Value::String(text) => Some(text.clone()),
        // Reformatting an existing temporal value applies the requested pattern.
        _ => Some(temporal_string(result, Some(pattern))),
    }
}

fn parse_date_value(value: &Value, format: Option<&str>) -> Result<Value, DwError> {
    let value = value.unwrap_formatted();
    let date = match value {
        Value::Date(date) => *date,
        Value::DateTime(date) => date.date,
        Value::LocalDateTime(date) => date.date,
        Value::String(text) => {
            let parsed = match format {
                Some(pattern) if !pattern.is_empty() => {
                    crate::temporal::parse_date_pattern(text, pattern)
                }
                _ => crate::temporal::parse_date(text)
                    .or_else(|| crate::temporal::parse_date_pattern(text, "uuuu/MM/dd")),
            };
            parsed.ok_or_else(|| DwError::new(format!("cannot convert '{text}' to Date")))?
        }
        other => {
            return Err(DwError::new(format!(
                "cannot convert {} to Date",
                other.type_name()
            )))
        }
    };
    let result = Value::Date(date);
    let display = formatted_display(value, &result, format);
    Ok(result.apply_format(display.as_deref()))
}

fn parse_date_time_value(value: &Value, format: Option<&str>) -> Result<Value, DwError> {
    let value = value.unwrap_formatted();
    let date_time = match value {
        Value::DateTime(date) => date.clone(),
        Value::LocalDateTime(date) => crate::temporal::DateTime {
            date: date.date,
            time: date.time,
            zone: crate::temporal::Zone::utc(),
        },
        Value::String(text) => {
            if let Some(pattern) = format.filter(|pattern| !pattern.is_empty()) {
                crate::temporal::parse_date_time_pattern(text, pattern)
                    .ok_or_else(|| DwError::new(format!("cannot convert '{text}' to DateTime")))?
            } else {
                match crate::temporal::parse_literal(text) {
                    Some(Value::DateTime(value)) => value,
                    _ => crate::temporal::parse_date_time_pattern(text, "uuuu-MM-dd'T'HH:mm:ssXXX")
                        .or_else(|| {
                            crate::temporal::parse_date_time_pattern(text, "uuuu-MM-dd HH:mm:ssXXX")
                        })
                        .ok_or_else(|| {
                            DwError::new(format!("cannot convert '{text}' to DateTime"))
                        })?,
                }
            }
        }
        other => {
            return Err(DwError::new(format!(
                "cannot convert {} to DateTime",
                other.type_name()
            )))
        }
    };
    let result = Value::DateTime(date_time);
    let display = formatted_display(value, &result, format);
    Ok(result.apply_format(display.as_deref()))
}

fn parse_local_date_time_value(value: &Value, format: Option<&str>) -> Result<Value, DwError> {
    let value = value.unwrap_formatted();
    let date_time = match value {
        Value::LocalDateTime(date) => date.clone(),
        Value::DateTime(date) => crate::temporal::LocalDateTime {
            date: date.date,
            time: date.time,
        },
        Value::String(text) => {
            if let Some(pattern) = format.filter(|pattern| !pattern.is_empty()) {
                crate::temporal::parse_local_date_time_pattern(text, pattern).ok_or_else(|| {
                    DwError::new(format!("cannot convert '{text}' to LocalDateTime"))
                })?
            } else {
                match crate::temporal::parse_literal(text) {
                    Some(Value::LocalDateTime(value)) => value,
                    _ => crate::temporal::parse_local_date_time_pattern(
                        text,
                        "uuuu-MM-dd'T'HH:mm:ss",
                    )
                    .or_else(|| {
                        crate::temporal::parse_local_date_time_pattern(text, "uuuu-MM-dd HH:mm:ss")
                    })
                    .ok_or_else(|| {
                        DwError::new(format!("cannot convert '{text}' to LocalDateTime"))
                    })?,
                }
            }
        }
        other => {
            return Err(DwError::new(format!(
                "cannot convert {} to LocalDateTime",
                other.type_name()
            )))
        }
    };
    let result = Value::LocalDateTime(date_time);
    let display = formatted_display(value, &result, format);
    Ok(result.apply_format(display.as_deref()))
}

fn parse_local_time_value(value: &Value, format: Option<&str>) -> Result<Value, DwError> {
    let value = value.unwrap_formatted();
    let time = match value {
        Value::LocalTime(time) => *time,
        Value::Time(time) => time.time,
        Value::String(text) => {
            if let Some(pattern) = format.filter(|pattern| !pattern.is_empty()) {
                crate::temporal::parse_local_time_pattern(text, pattern)
                    .ok_or_else(|| DwError::new(format!("cannot convert '{text}' to LocalTime")))?
            } else {
                match crate::temporal::parse_literal(text) {
                    Some(Value::LocalTime(value)) => value,
                    _ => crate::temporal::parse_local_time_pattern(text, "HH:mm:ss")
                        .or_else(|| crate::temporal::parse_local_time_pattern(text, "HH:mm"))
                        .ok_or_else(|| {
                            DwError::new(format!("cannot convert '{text}' to LocalTime"))
                        })?,
                }
            }
        }
        other => {
            return Err(DwError::new(format!(
                "cannot convert {} to LocalTime",
                other.type_name()
            )))
        }
    };
    let result = Value::LocalTime(time);
    let display = formatted_display(value, &result, format);
    Ok(result.apply_format(display.as_deref()))
}

fn parse_time_value(value: &Value, format: Option<&str>) -> Result<Value, DwError> {
    let value = value.unwrap_formatted();
    let time = match value {
        Value::Time(time) => time.clone(),
        Value::LocalTime(time) => crate::temporal::Time {
            time: *time,
            zone: crate::temporal::Zone::utc(),
        },
        Value::String(text) => {
            if let Some(pattern) = format.filter(|pattern| !pattern.is_empty()) {
                crate::temporal::parse_time_pattern(text, pattern)
                    .ok_or_else(|| DwError::new(format!("cannot convert '{text}' to Time")))?
            } else {
                match crate::temporal::parse_literal(text) {
                    Some(Value::Time(value)) => value,
                    _ => crate::temporal::parse_time_pattern(text, "HH:mm:ssXXX")
                        .or_else(|| crate::temporal::parse_time_pattern(text, "HH:mm:ss"))
                        .ok_or_else(|| DwError::new(format!("cannot convert '{text}' to Time")))?,
                }
            }
        }
        other => {
            return Err(DwError::new(format!(
                "cannot convert {} to Time",
                other.type_name()
            )))
        }
    };
    let result = Value::Time(time);
    let display = formatted_display(value, &result, format);
    Ok(result.apply_format(display.as_deref()))
}

fn period_to_number(
    period: &crate::temporal::Period,
    unit: Option<&str>,
) -> Result<Number, DwError> {
    let unit = unit.unwrap_or(if period.date_based {
        "months"
    } else {
        "seconds"
    });
    let years = Number::from_i64(period.years);
    let months = Number::from_i64(period.months);
    let days = Number::from_i64(period.days);
    match unit {
        "years" => {
            let months_part = months
                .div(&Number::from_i64(12))
                .unwrap_or_else(Number::zero);
            Ok(years.add(&months_part))
        }
        "months" => Ok(years.mul(&Number::from_i64(12)).add(&months)),
        "days" => Ok(years
            .mul(&Number::from_i64(365))
            .add(&months.mul(&Number::from_i64(30)))
            .add(&days)),
        "nanos" => Ok(period.time_seconds.mul(&Number::from_i64(1_000_000_000))),
        "milliseconds" | "millis" => Ok(period.time_seconds.mul(&Number::from_i64(1_000))),
        "seconds" => Ok(period.time_seconds.clone()),
        "minutes" => Ok(period
            .time_seconds
            .div(&Number::from_i64(60))
            .unwrap_or_else(Number::zero)),
        "hours" => Ok(period
            .time_seconds
            .div(&Number::from_i64(3_600))
            .unwrap_or_else(Number::zero)),
        other => Err(DwError::new(format!("unsupported period unit '{other}'"))),
    }
}

/// Converts a value to a named type with optional cast options.
pub fn cast_value(
    value: Value,
    type_name: &str,
    options: Option<&Value>,
) -> Result<Value, DwError> {
    cast(value, type_name, options)
}

/// Converts a value with `as`, supporting scalar, collection, temporal, and binary types.
fn cast(value: Value, type_name: &str, options: Option<&Value>) -> Result<Value, DwError> {
    // Conversions inspect the ISO value; only the target temporal type keeps a new format.
    let value = value.unwrap_formatted().clone();
    let base = type_name.split('{').next().unwrap_or(type_name).trim();
    let format = pattern_from(options);
    match base {
        "Any" => Ok(value),
        "String" => Ok(Value::String(temporal_string(&value, format.as_deref()))),
        "Number" => match value {
            Value::Number(_) => Ok(value),
            Value::String(text) => Number::parse(&text)
                .map(Value::Number)
                .ok_or_else(|| DwError::new("cannot convert string to number")),
            Value::Bool(value) => Ok(Value::Number(Number::from_i64(if value { 1 } else { 0 }))),
            Value::DateTime(date) => {
                let millis = matches!(format.as_deref(), Some("milliseconds" | "millis"));
                crate::temporal::timestamp_seconds(&date).map(|seconds| {
                    Value::Number(Number::from_i64(if millis {
                        seconds * 1000
                    } else {
                        seconds
                    }))
                })
            }
            Value::Period(period) => {
                period_to_number(&period, format.as_deref()).map(Value::Number)
            }
            other => Err(DwError::new(format!(
                "cannot convert {} to number",
                other.type_name()
            ))),
        },
        "Boolean" => match value {
            Value::Bool(_) => Ok(value),
            Value::String(text) if text.eq_ignore_ascii_case("true") => Ok(Value::Bool(true)),
            Value::String(text) if text.eq_ignore_ascii_case("false") => Ok(Value::Bool(false)),
            other => Err(DwError::new(format!(
                "cannot convert {} to boolean",
                other.type_name()
            ))),
        },
        "Array" => match value {
            Value::Array(_) => Ok(value),
            Value::Null => Ok(Value::Array(Vec::new())),
            other => Ok(Value::Array(vec![other])),
        },
        "Object" => match value {
            Value::Object(_) => Ok(value),
            other => Err(DwError::new(format!(
                "cannot convert {} to object",
                other.type_name()
            ))),
        },
        "Null" => Ok(Value::Null),
        "Date" => parse_date_value(&value, format.as_deref()),
        "DateTime" => match value {
            Value::Number(number) => {
                let millis = matches!(format.as_deref(), Some("milliseconds" | "millis"));
                let value = number
                    .to_i64()
                    .ok_or_else(|| DwError::new("expected an integer timestamp"))?;
                crate::temporal::utc_date_time_from_timestamp(value, millis).map(Value::DateTime)
            }
            other => parse_date_time_value(&other, format.as_deref()),
        },
        "LocalDateTime" => parse_local_date_time_value(&value, format.as_deref()),
        "LocalTime" => parse_local_time_value(&value, format.as_deref()),
        "Time" => parse_time_value(&value, format.as_deref()),
        "TimeZone" => match value {
            Value::TimeZone(_) => Ok(value),
            Value::String(text) => crate::temporal::Zone::parse(&text)
                .map(Value::TimeZone)
                .ok_or_else(|| DwError::new(format!("cannot convert '{text}' to TimeZone"))),
            other => Err(DwError::new(format!(
                "cannot convert {} to TimeZone",
                other.type_name()
            ))),
        },
        "Period" => match value {
            Value::Period(_) => Ok(value),
            Value::String(text) => crate::temporal::parse_period(&text)
                .map(Value::Period)
                .ok_or_else(|| DwError::new(format!("cannot convert '{text}' to Period"))),
            other => Err(DwError::new(format!(
                "cannot convert {} to Period",
                other.type_name()
            ))),
        },
        "Binary" => match value {
            Value::Binary(_) => Ok(value),
            Value::String(text) => {
                let base = option_text(options, "base").unwrap_or_default();
                let bytes = match base.as_str() {
                    "16" | "hex" => crate::temporal::hex_decode(&text)?,
                    "64" | "base64" => crate::temporal::base64_decode(&text)?,
                    _ => text.into_bytes(),
                };
                Ok(Value::Binary(bytes))
            }
            other => Err(DwError::new(format!(
                "cannot convert {} to Binary",
                other.type_name()
            ))),
        },
        "Regex" => match value {
            Value::Regex(_) => Ok(value),
            Value::String(text) => Ok(Value::Regex(text)),
            other => Err(DwError::new(format!(
                "cannot convert {} to Regex",
                other.type_name()
            ))),
        },
        "Uri" | "URI" => match value {
            Value::Uri(_) => Ok(value),
            Value::String(text) => Ok(Value::Uri(text)),
            other => Ok(Value::Uri(other.key_string())),
        },
        "Key" => Ok(Value::Key(value.key_string())),
        other => Err(DwError::new(format!("unsupported type '{other}'"))),
    }
}
