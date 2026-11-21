// SPDX-License-Identifier: MIT OR Apache-2.0
//! Tokenizer for scripts.

use crate::error::DwError;

/// One part of a string literal that may contain interpolation.
#[derive(Clone, Debug, PartialEq)]
pub enum InterpPart {
    /// Literal text.
    Text(String),
    /// Source text inside `$(...)`.
    Expr(String),
}

/// A lexical token.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(String),
    Str(String),
    Literal(String),
    Interp(Vec<InterpPart>),
    Regex(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Dot,
    Semi,
    Question,
    Op(String),
    Arrow,
    DoSeparator,
    Eof,
}

const REGEX_FUNCTIONS: &[&str] = &[
    "splitBy",
    "find",
    "match",
    "matches",
    "scan",
    "replace",
    "contains",
    "countMatches",
];

/// Tokenizes a script, returning an EOF token at the end.
pub fn lex(input: &str) -> Result<Vec<Token>, DwError> {
    let chars: Vec<char> = input.chars().collect();
    let mut pos = 0usize;
    let mut tokens = Vec::new();
    while pos < chars.len() {
        let c = chars[pos];
        match c {
            ' ' | '\t' | '\r' | '\n' => pos += 1,
            '/' if chars.get(pos + 1) == Some(&'/') => {
                pos += 2;
                while pos < chars.len() && chars[pos] != '\n' {
                    pos += 1;
                }
            }
            '/' if chars.get(pos + 1) == Some(&'*') => {
                pos += 2;
                let mut closed = false;
                while pos + 1 < chars.len() {
                    if chars[pos] == '*' && chars[pos + 1] == '/' {
                        pos += 2;
                        closed = true;
                        break;
                    }
                    pos += 1;
                }
                if !closed {
                    return Err(DwError::new("unterminated comment"));
                }
            }
            '/' if regex_allowed(&tokens) => {
                let (pattern, next) = lex_regex(&chars, pos)?;
                pos = next;
                tokens.push(Token::Regex(pattern));
            }
            '"' | '\'' => {
                let (parts, next) = lex_string(&chars, pos, c)?;
                pos = next;
                if parts.iter().any(|part| matches!(part, InterpPart::Expr(_))) {
                    tokens.push(Token::Interp(parts));
                } else {
                    let text = parts
                        .into_iter()
                        .map(|part| match part {
                            InterpPart::Text(text) => text,
                            InterpPart::Expr(_) => String::new(),
                        })
                        .collect();
                    tokens.push(Token::Str(text));
                }
            }
            '(' => {
                tokens.push(Token::LParen);
                pos += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                pos += 1;
            }
            '[' => {
                tokens.push(Token::LBracket);
                pos += 1;
            }
            ']' => {
                tokens.push(Token::RBracket);
                pos += 1;
            }
            '{' => {
                tokens.push(Token::LBrace);
                pos += 1;
            }
            '}' => {
                tokens.push(Token::RBrace);
                pos += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                pos += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                pos += 1;
            }
            ';' => {
                tokens.push(Token::Semi);
                pos += 1;
            }
            '.' => {
                tokens.push(Token::Dot);
                pos += 1;
            }
            '?' => {
                tokens.push(Token::Question);
                pos += 1;
            }
            '|' => {
                pos += 1;
                let start = pos;
                while pos < chars.len() && chars[pos] != '|' {
                    pos += 1;
                }
                if pos >= chars.len() {
                    return Err(DwError::new("unterminated literal"));
                }
                let text: String = chars[start..pos].iter().collect();
                pos += 1;
                tokens.push(Token::Literal(text));
            }
            '$' => {
                let mut name = String::new();
                while pos < chars.len() && chars[pos] == '$' {
                    name.push('$');
                    pos += 1;
                }
                tokens.push(Token::Ident(name));
            }
            '-' if chars.get(pos + 1) == Some(&'>') => {
                tokens.push(Token::Arrow);
                pos += 2;
            }
            '-' if chars.get(pos + 1) == Some(&'-') && chars.get(pos + 2) == Some(&'-') => {
                tokens.push(Token::DoSeparator);
                pos += 3;
            }
            '-' if chars.get(pos + 1) == Some(&'-') => {
                tokens.push(Token::Op("--".to_string()));
                pos += 2;
            }
            '+' => {
                if chars.get(pos + 1) == Some(&'+') {
                    tokens.push(Token::Op("++".to_string()));
                    pos += 2;
                } else {
                    tokens.push(Token::Op("+".to_string()));
                    pos += 1;
                }
            }
            '-' => {
                tokens.push(Token::Op("-".to_string()));
                pos += 1;
            }
            '*' => {
                tokens.push(Token::Op("*".to_string()));
                pos += 1;
            }
            '/' => {
                tokens.push(Token::Op("/".to_string()));
                pos += 1;
            }
            '%' => {
                tokens.push(Token::Op("%".to_string()));
                pos += 1;
            }
            '~' => {
                tokens.push(Token::Op("~".to_string()));
                pos += 1;
            }
            '^' => {
                tokens.push(Token::Op("^".to_string()));
                pos += 1;
            }
            '=' => {
                if chars.get(pos + 1) == Some(&'=') {
                    tokens.push(Token::Op("==".to_string()));
                    pos += 2;
                } else {
                    tokens.push(Token::Op("=".to_string()));
                    pos += 1;
                }
            }
            '!' => {
                if chars.get(pos + 1) == Some(&'=') {
                    tokens.push(Token::Op("!=".to_string()));
                    pos += 2;
                } else {
                    tokens.push(Token::Op("!".to_string()));
                    pos += 1;
                }
            }
            '<' => {
                if chars.get(pos + 1) == Some(&'=') {
                    tokens.push(Token::Op("<=".to_string()));
                    pos += 2;
                } else if chars.get(pos + 1) == Some(&'<') {
                    tokens.push(Token::Op("<<".to_string()));
                    pos += 2;
                } else {
                    tokens.push(Token::Op("<".to_string()));
                    pos += 1;
                }
            }
            '>' => {
                if chars.get(pos + 1) == Some(&'=') {
                    tokens.push(Token::Op(">=".to_string()));
                    pos += 2;
                } else {
                    tokens.push(Token::Op(">".to_string()));
                    pos += 1;
                }
            }
            c if c.is_ascii_digit() => {
                let start = pos;
                while pos < chars.len() && chars[pos].is_ascii_digit() {
                    pos += 1;
                }
                if pos < chars.len() && chars[pos] == '.' {
                    let next_is_digit = chars
                        .get(pos + 1)
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false);
                    if next_is_digit {
                        pos += 1;
                        while pos < chars.len() && chars[pos].is_ascii_digit() {
                            pos += 1;
                        }
                    }
                }
                if pos < chars.len() && (chars[pos] == 'e' || chars[pos] == 'E') {
                    let mut probe = pos + 1;
                    if probe < chars.len() && (chars[probe] == '+' || chars[probe] == '-') {
                        probe += 1;
                    }
                    if probe < chars.len() && chars[probe].is_ascii_digit() {
                        pos = probe;
                        while pos < chars.len() && chars[pos].is_ascii_digit() {
                            pos += 1;
                        }
                    }
                }
                let text: String = chars[start..pos].iter().collect();
                tokens.push(Token::Number(text));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = pos;
                while pos < chars.len()
                    && (chars[pos].is_alphanumeric() || chars[pos] == '_' || chars[pos] == '$')
                {
                    pos += 1;
                }
                let text: String = chars[start..pos].iter().collect();
                tokens.push(Token::Ident(text));
            }
            other => {
                return Err(DwError::new(format!("unexpected character '{other}'")));
            }
        }
    }
    tokens.push(Token::Eof);
    Ok(tokens)
}

/// Returns whether `/` starts a regex literal after a regex-aware function name.
fn regex_allowed(tokens: &[Token]) -> bool {
    match tokens.last() {
        Some(Token::Ident(name)) => REGEX_FUNCTIONS.contains(&name.as_str()),
        _ => false,
    }
}

/// Scans a `/.../` regex literal and returns its pattern and end position.
fn lex_regex(chars: &[char], start: usize) -> Result<(String, usize), DwError> {
    let mut pos = start + 1;
    let mut pattern = String::new();
    while pos < chars.len() {
        let c = chars[pos];
        pos += 1;
        if c == '/' {
            return Ok((pattern, pos));
        }
        if c == '\\' {
            let next = *chars
                .get(pos)
                .ok_or_else(|| DwError::new("unterminated regular expression"))?;
            pos += 1;
            if next == '/' {
                pattern.push('/');
            } else {
                pattern.push('\\');
                pattern.push(next);
            }
        } else {
            pattern.push(c);
        }
    }
    Err(DwError::new("unterminated regular expression"))
}

/// Scans a quoted string into literal and interpolation parts.
fn lex_string(
    chars: &[char],
    start: usize,
    quote: char,
) -> Result<(Vec<InterpPart>, usize), DwError> {
    let mut pos = start + 1;
    let mut text = String::new();
    let mut parts = Vec::new();
    while pos < chars.len() {
        let c = chars[pos];
        pos += 1;
        if c == quote {
            parts.push(InterpPart::Text(text));
            return Ok((parts, pos));
        }
        if c == '\\' {
            decode_escape(chars, &mut pos, &mut text)?;
            continue;
        }
        if c == '$' && chars.get(pos) == Some(&'(') {
            pos += 1;
            if !text.is_empty() {
                parts.push(InterpPart::Text(std::mem::take(&mut text)));
            }
            let (expr, next) = scan_interpolation(chars, pos)?;
            pos = next;
            parts.push(InterpPart::Expr(expr));
            continue;
        }
        text.push(c);
    }
    Err(DwError::new("unterminated string"))
}

/// Decodes one backslash escape into `out`.
fn decode_escape(chars: &[char], pos: &mut usize, out: &mut String) -> Result<(), DwError> {
    let escape = *chars
        .get(*pos)
        .ok_or_else(|| DwError::new("unterminated string escape"))?;
    *pos += 1;
    match escape {
        'n' => out.push('\n'),
        'r' => out.push('\r'),
        't' => out.push('\t'),
        'b' => out.push('\u{08}'),
        'f' => out.push('\u{0c}'),
        '\\' => out.push('\\'),
        '"' => out.push('"'),
        '\'' => out.push('\''),
        '/' => out.push('/'),
        'u' => {
            let mut value: u32 = 0;
            for _ in 0..4 {
                let digit = *chars
                    .get(*pos)
                    .ok_or_else(|| DwError::new("invalid unicode escape"))?;
                let digit = digit
                    .to_digit(16)
                    .ok_or_else(|| DwError::new("invalid unicode escape"))?;
                value = value * 16 + digit;
                *pos += 1;
            }
            if let Some(ch) = char::from_u32(value) {
                out.push(ch);
            } else {
                return Err(DwError::new("invalid unicode escape"));
            }
        }
        other => {
            out.push('\\');
            out.push(other);
        }
    }
    Ok(())
}

/// Extracts the balanced source text inside `$(...)`.
fn scan_interpolation(chars: &[char], start: usize) -> Result<(String, usize), DwError> {
    let mut pos = start;
    let mut depth = 1usize;
    let mut expr = String::new();
    while pos < chars.len() {
        let c = chars[pos];
        pos += 1;
        if c == '"' || c == '\'' {
            expr.push(c);
            while pos < chars.len() {
                let d = chars[pos];
                pos += 1;
                expr.push(d);
                if d == '\\' {
                    if let Some(escaped) = chars.get(pos) {
                        expr.push(*escaped);
                        pos += 1;
                    }
                } else if d == c {
                    break;
                }
            }
            continue;
        }
        if c == '(' {
            depth += 1;
            expr.push(c);
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                return Ok((expr, pos));
            }
            expr.push(c);
        } else {
            expr.push(c);
        }
    }
    Err(DwError::new("unterminated string interpolation"))
}
