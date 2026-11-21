// SPDX-License-Identifier: MIT OR Apache-2.0
//! Abstract syntax tree for parsed scripts.

use crate::number::Number;

/// A function or lambda parameter, with optional default and type annotation.
#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub default: Option<Expr>,
    pub type_name: Option<String>,
}

/// One piece of an interpolated string.
#[derive(Clone, Debug)]
pub enum InterpSegment {
    /// Literal text.
    Text(String),
    /// An expression embedded with `$(...)`.
    Expr(Expr),
}

/// A parsed expression.
#[derive(Clone, Debug)]
pub enum Expr {
    /// `null`.
    Null,
    /// A boolean literal.
    Bool(bool),
    /// A numeric literal.
    Number(Number),
    /// A string literal.
    String(String),
    /// A `|...|` temporal or binary literal.
    Literal(String),
    /// A regular-expression literal.
    Regex(String),
    /// A string containing embedded expressions.
    Interp(Vec<InterpSegment>),
    /// A variable reference.
    Var(String),
    /// An array literal.
    Array(Vec<Expr>),
    /// An object literal with key/value expressions.
    Object(Vec<(Expr, Expr)>),
    /// A unary operation.
    Unary(UnOp, Box<Expr>),
    /// A binary operation.
    Binary(BinOp, Box<Expr>, Box<Expr>),
    /// A function call.
    Call(Box<Expr>, Vec<Expr>),
    /// An `if`/`else` expression.
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    /// A lambda expression.
    Lambda(Vec<Param>, Box<Expr>),
    /// Field selection with `.name`.
    Select(Box<Expr>, String),
    /// Safe field selection with `.name?`.
    SafeSelect(Box<Expr>, String),
    /// Multi-value selection with `.*name`.
    MultiSelect(Box<Expr>, String),
    /// Descendant selection with `..name`.
    DescendantSelect(Box<Expr>, String),
    /// Key-value selection with `.&name`.
    KeyValueSelect(Box<Expr>, String),
    /// XML attribute selection with `.@name`, `.@"name"`, or `.@`.
    AttrSelect(Box<Expr>, Option<String>),
    /// Safe XML attribute selection with `.@name?`.
    SafeAttr(Box<Expr>, Option<String>),
    /// Indexing with `[index]`.
    Index(Box<Expr>, Box<Expr>),
    /// Safe indexing with `[index]?`.
    SafeIndex(Box<Expr>, Box<Expr>),
    /// Filter selection with `[?(predicate)]`.
    Filter(Box<Expr>, Box<Expr>),
    /// Postfix non-null assertion with `!`.
    Assert(Box<Expr>),
    /// A function call written in infix form.
    Infix(String, Box<Expr>, Box<Expr>),
    /// A type cast with `as` and optional options.
    As(Box<Expr>, String, Option<Box<Expr>>),
    /// A type test with `is`.
    Is(Box<Expr>, String),
    /// A `default` fallback.
    Default(Box<Expr>, Box<Expr>),
    /// Array construction with `[head ~ tail]`.
    ArrayCons(Box<Expr>, Box<Expr>),
    /// A `match` expression with pattern/body arms.
    Match(Box<Expr>, Vec<(MatchPattern, Expr)>),
    /// A `do` block with local declarations and a body.
    Do(Vec<(String, Expr)>, Box<Expr>),
    /// A `using` block with local declarations and a body.
    Using(Vec<(String, Expr)>, Box<Expr>),
    /// Object construction with one head entry and an object tail.
    ObjectSpread(Box<Expr>, Box<Expr>, Box<Expr>),
    /// Metadata lookup with `.^key`.
    MetaKey(Box<Expr>, String),
    /// The whole metadata object with `.^`.
    MetaAll(Box<Expr>),
}

/// A pattern in a `match` expression.
#[derive(Clone, Debug)]
pub enum MatchPattern {
    /// The `else` arm, which always matches.
    Else,
    /// A bare name that binds the matched value.
    Bind(String),
    /// A literal value.
    Literal(Expr),
    /// `case [name is] Type`.
    Type {
        binding: Option<String>,
        type_name: String,
    },
    /// `case name: literal`.
    NamedLiteral { binding: String, literal: Expr },
    /// `case name if (condition)`.
    Guard { binding: String, condition: Expr },
    /// `case [name] matches /pattern/`.
    Regex {
        binding: Option<String>,
        pattern: String,
    },
    /// An exact array pattern.
    Array(Vec<MatchPattern>),
    /// An array pattern with a head and a tail.
    ArrayCons {
        head: Box<MatchPattern>,
        tail: Box<MatchPattern>,
    },
    /// `{ keyVar: valueVar ~ tailVar }`.
    ObjectCons {
        key: String,
        value: String,
        tail: String,
    },
    /// `{}`.
    EmptyObject,
}

/// Unary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    /// Arithmetic negation.
    Negate,
    /// Logical negation.
    Not,
}

/// Binary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Concat,
    Append,
    Prepend,
    Difference,
    Equal,
    CoercingEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

/// Returns whether an expression contains an implicit `$` or `$$` parameter.
pub fn contains_placeholder(expr: &Expr) -> bool {
    match expr {
        Expr::Var(name) => name == "$" || name == "$$",
        Expr::Array(items) => items.iter().any(contains_placeholder),
        Expr::Object(entries) => entries
            .iter()
            .any(|(key, value)| contains_placeholder(key) || contains_placeholder(value)),
        Expr::Unary(_, inner) => contains_placeholder(inner),
        Expr::Binary(_, left, right) => contains_placeholder(left) || contains_placeholder(right),
        Expr::Call(callee, args) => {
            contains_placeholder(callee) || args.iter().any(contains_placeholder)
        }
        Expr::If(condition, then_branch, else_branch) => {
            contains_placeholder(condition)
                || contains_placeholder(then_branch)
                || contains_placeholder(else_branch)
        }
        Expr::Lambda(_, body) => contains_placeholder(body),
        Expr::Select(target, _)
        | Expr::SafeSelect(target, _)
        | Expr::MultiSelect(target, _)
        | Expr::DescendantSelect(target, _)
        | Expr::KeyValueSelect(target, _)
        | Expr::AttrSelect(target, _)
        | Expr::SafeAttr(target, _)
        | Expr::Assert(target) => contains_placeholder(target),
        Expr::Index(target, index)
        | Expr::SafeIndex(target, index)
        | Expr::Filter(target, index) => {
            contains_placeholder(target) || contains_placeholder(index)
        }
        Expr::Infix(_, left, right) => contains_placeholder(left) || contains_placeholder(right),
        Expr::As(inner, _, options) => {
            contains_placeholder(inner)
                || options
                    .as_ref()
                    .map(|options| contains_placeholder(options))
                    .unwrap_or(false)
        }
        Expr::Is(inner, _) => contains_placeholder(inner),
        Expr::Default(left, right) => contains_placeholder(left) || contains_placeholder(right),
        Expr::ArrayCons(head, tail) => contains_placeholder(head) || contains_placeholder(tail),
        Expr::Match(subject, arms) => {
            contains_placeholder(subject)
                || arms.iter().any(|(pattern, body)| {
                    pattern_contains_placeholder(pattern) || contains_placeholder(body)
                })
        }
        Expr::Do(declarations, body) => {
            declarations
                .iter()
                .any(|(_, value)| contains_placeholder(value))
                || contains_placeholder(body)
        }
        Expr::Using(declarations, body) => {
            declarations
                .iter()
                .any(|(_, value)| contains_placeholder(value))
                || contains_placeholder(body)
        }
        Expr::ObjectSpread(key, value, tail) => {
            contains_placeholder(key) || contains_placeholder(value) || contains_placeholder(tail)
        }
        Expr::MetaKey(target, _) | Expr::MetaAll(target) => contains_placeholder(target),
        Expr::Interp(parts) => parts.iter().any(|part| match part {
            InterpSegment::Text(_) => false,
            InterpSegment::Expr(expr) => contains_placeholder(expr),
        }),
        Expr::Null
        | Expr::Bool(_)
        | Expr::Number(_)
        | Expr::String(_)
        | Expr::Literal(_)
        | Expr::Regex(_) => false,
    }
}

/// Returns whether a match pattern contains an implicit `$` or `$$` parameter.
fn pattern_contains_placeholder(pattern: &MatchPattern) -> bool {
    match pattern {
        MatchPattern::Else | MatchPattern::Bind(_) | MatchPattern::EmptyObject => false,
        MatchPattern::Literal(value) => contains_placeholder(value),
        MatchPattern::Type { .. } => false,
        MatchPattern::NamedLiteral { literal, .. } => contains_placeholder(literal),
        MatchPattern::Guard { condition, .. } => contains_placeholder(condition),
        MatchPattern::Regex { .. } => false,
        MatchPattern::Array(items) => items.iter().any(pattern_contains_placeholder),
        MatchPattern::ArrayCons { head, tail } => {
            pattern_contains_placeholder(head) || pattern_contains_placeholder(tail)
        }
        MatchPattern::ObjectCons { .. } => false,
    }
}
