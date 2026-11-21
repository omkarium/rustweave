// SPDX-License-Identifier: MIT OR Apache-2.0
//! Recursive-descent parser for scripts and expressions.

use crate::ast::{BinOp, Expr, InterpSegment, Param, UnOp};
use crate::error::DwError;
use crate::lexer::{lex, InterpPart, Token};
use crate::number::Number;
use crate::value::Value;

/// A named function declared in a script header.
#[derive(Clone, Debug)]
pub struct FunDef {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Expr,
}

/// A declaration from a script header.
#[derive(Clone, Debug)]
pub enum HeaderItem {
    /// A global `var` binding.
    Var(String, Expr),
    /// A global `fun` definition.
    Fun(FunDef),
    /// A named type declaration with its source expression.
    Type(String, String),
}

/// A parsed script: header declarations followed by a body expression.
#[derive(Clone, Debug)]
pub struct Script {
    pub items: Vec<HeaderItem>,
    pub imports: Vec<String>,
    pub output: Option<String>,
    pub output_options: Option<Value>,
    pub body: Expr,
}

/// Finds the header separator, ignoring `---` used inside a body or `do` block.
fn find_header_separator(lines: &[&str]) -> Option<usize> {
    for (index, line) in lines.iter().enumerate() {
        if line.trim() != "---" {
            continue;
        }
        let is_header = lines[..index].iter().all(|line| {
            let line = line.trim();
            line.is_empty()
                || line.starts_with("%dw")
                || line.starts_with("output ")
                || line.starts_with("import ")
                || line.starts_with("var ")
                || line.starts_with("fun ")
                || line.starts_with("type ")
                || line.starts_with("ns ")
                || line.starts_with('@')
        });
        if is_header {
            return Some(index);
        }
    }
    None
}

/// Parses a complete script with an optional header.
pub fn parse_script(text: &str) -> Result<Script, DwError> {
    let lines: Vec<&str> = text.lines().collect();
    let (header_lines, body_text) = match find_header_separator(&lines) {
        Some(index) => (&lines[..index], lines[index + 1..].join("\n")),
        None => (&lines[..0], text.to_string()),
    };

    let mut items = Vec::new();
    let mut imports = Vec::new();
    let mut output = None;
    let mut output_options = None;
    for line in header_lines {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("output ") {
            let (mime, options) = parse_output_declaration(rest)?;
            output = Some(mime);
            output_options = options;
            continue;
        }
        if line.starts_with("import ") {
            if let Some((_, module)) = line.split_once(" from ") {
                imports.push(module.trim().to_string());
            }
            continue;
        }
        if line.is_empty()
            || line.starts_with("%dw")
            || line.starts_with("output ")
            || line.starts_with('@')
        {
            continue;
        }
        if let Some(rest) = line.strip_prefix("var ") {
            let (name, value) = split_assignment(rest)?;
            let name = name.split(':').next().unwrap_or(name).trim().to_string();
            if name.is_empty() {
                return Err(DwError::new("invalid var declaration"));
            }
            items.push(HeaderItem::Var(name, parse_expression(value.trim())?));
            continue;
        }
        if let Some(rest) = line.strip_prefix("fun ") {
            items.push(HeaderItem::Fun(parse_fun(rest.trim())?));
            continue;
        }
        if let Some(rest) = line.strip_prefix("type ") {
            let (name, value) = rest
                .split_once('=')
                .ok_or_else(|| DwError::new("invalid type declaration"))?;
            let name = name.trim().to_string();
            if name.is_empty() {
                return Err(DwError::new("invalid type declaration"));
            }
            items.push(HeaderItem::Type(name, value.trim().to_string()));
            continue;
        }
        if line.starts_with("ns ") {
            continue;
        }
        return Err(DwError::new(format!("unsupported header line: {line}")));
    }

    let body = parse_expression(&body_text)?;
    Ok(Script {
        items,
        imports,
        output,
        output_options,
        body,
    })
}

fn parse_output_declaration(text: &str) -> Result<(String, Option<Value>), DwError> {
    let mut chars = text.trim().char_indices().peekable();
    let mut mime = String::new();
    while let Some((_, character)) = chars.peek().copied() {
        if character.is_whitespace() || character == ',' {
            break;
        }
        mime.push(character);
        chars.next();
    }
    if mime.is_empty() {
        return Err(DwError::new("output declaration is missing a MIME type"));
    }

    let rest = chars
        .peek()
        .map(|(index, _)| &text[*index..])
        .unwrap_or("")
        .trim();
    if rest.is_empty() {
        return Ok((mime, None));
    }

    let mut entries = Vec::new();
    let mut cursor = 0usize;
    while cursor < rest.len() {
        while cursor < rest.len() {
            let character = rest[cursor..].chars().next().unwrap_or(' ');
            if character.is_whitespace() || character == ',' {
                cursor += character.len_utf8();
            } else {
                break;
            }
        }
        if cursor >= rest.len() {
            break;
        }
        let key_start = cursor;
        while cursor < rest.len() {
            let character = rest[cursor..].chars().next().unwrap_or(' ');
            if character == '=' || character.is_whitespace() || character == ',' {
                break;
            }
            cursor += character.len_utf8();
        }
        let key = rest[key_start..cursor].trim();
        if key.is_empty() {
            return Err(DwError::new("invalid output property"));
        }
        while cursor < rest.len() {
            let character = rest[cursor..].chars().next().unwrap_or(' ');
            if character.is_whitespace() {
                cursor += character.len_utf8();
            } else {
                break;
            }
        }
        if !rest[cursor..].starts_with('=') {
            return Err(DwError::new(format!(
                "output property '{key}' is missing a value"
            )));
        }
        cursor += 1;
        while cursor < rest.len() {
            let character = rest[cursor..].chars().next().unwrap_or(' ');
            if character.is_whitespace() {
                cursor += character.len_utf8();
            } else {
                break;
            }
        }
        let value = if rest[cursor..].starts_with('"') || rest[cursor..].starts_with('\'') {
            let quote = rest[cursor..].chars().next().unwrap_or('"');
            cursor += quote.len_utf8();
            let mut text = String::new();
            let mut escaped = false;
            while cursor < rest.len() {
                let character = rest[cursor..].chars().next().unwrap_or(' ');
                cursor += character.len_utf8();
                if escaped {
                    text.push(character);
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == quote {
                    break;
                } else {
                    text.push(character);
                }
            }
            Value::String(text)
        } else {
            let value_start = cursor;
            while cursor < rest.len() {
                let character = rest[cursor..].chars().next().unwrap_or(' ');
                if character.is_whitespace() || character == ',' {
                    break;
                }
                cursor += character.len_utf8();
            }
            let token = &rest[value_start..cursor];
            match token {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                "null" => Value::Null,
                other => match Number::parse(other) {
                    Some(number) => Value::Number(number),
                    None => Value::String(other.to_string()),
                },
            }
        };
        entries.push((key.to_string(), value));
    }
    Ok((mime, Some(Value::Object(entries))))
}

/// Splits `name = value` without splitting comparison operators.
fn split_assignment(text: &str) -> Result<(&str, &str), DwError> {
    let bytes = text.as_bytes();
    for (index, c) in text.char_indices() {
        if c != '=' {
            continue;
        }
        let prev = if index > 0 { bytes[index - 1] } else { 0 };
        let next = bytes.get(index + 1).copied().unwrap_or(0);
        if next == b'=' || prev == b'=' || prev == b'<' || prev == b'>' || prev == b'!' {
            continue;
        }
        return Ok((&text[..index], &text[index + 1..]));
    }
    Err(DwError::new("expected '=' in declaration"))
}

/// Parses a `fun name(params) = body` header line.
fn parse_fun(text: &str) -> Result<FunDef, DwError> {
    let open = text
        .find('(')
        .ok_or_else(|| DwError::new("invalid function declaration"))?;
    let name = text[..open].trim().to_string();
    if name.is_empty() {
        return Err(DwError::new("invalid function declaration"));
    }
    let close = text
        .find(')')
        .ok_or_else(|| DwError::new("invalid function declaration"))?;
    let param_text = &text[open + 1..close];
    let mut params = Vec::new();
    for part in param_text
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        params.push(parse_param(part)?);
    }
    let rest = text[close + 1..].trim();
    let equals = rest
        .find('=')
        .ok_or_else(|| DwError::new("invalid function declaration"))?;
    let body = parse_expression(rest[equals + 1..].trim())?;
    Ok(FunDef { name, params, body })
}

/// Parses one parameter with an optional type and default value.
fn parse_param(text: &str) -> Result<Param, DwError> {
    let (left, default) = match text.split_once('=') {
        Some((left, right)) => (left.trim(), Some(parse_expression(right.trim())?)),
        None => (text.trim(), None),
    };
    let (name, type_name) = match left.split_once(':') {
        Some((name, type_name)) => (name.trim(), Some(type_name.trim().to_string())),
        None => (left, None),
    };
    Ok(Param {
        name: name.to_string(),
        default,
        type_name,
    })
}

/// Parses one expression from source text.
pub fn parse_expression(text: &str) -> Result<Expr, DwError> {
    let tokens = lex(text)?;
    parse_tokens(&tokens)
}

/// Parses an expression from an existing token slice, requiring a complete parse.
fn parse_tokens(tokens: &[Token]) -> Result<Expr, DwError> {
    let mut tokens = tokens.to_vec();
    if !matches!(tokens.last(), Some(Token::Eof)) {
        tokens.push(Token::Eof);
    }
    let mut parser = Parser { tokens, pos: 0 };
    let expr = parser.parse_expr(0)?;
    if !matches!(parser.peek(), Token::Eof) {
        return Err(parser.error("unexpected token"));
    }
    Ok(expr)
}

/// Token cursor used while parsing.
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Returns the current token without consuming it.
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    /// Consumes and returns the current token.
    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        token
    }

    /// Builds an error that includes the current token.
    fn error(&self, message: &str) -> DwError {
        DwError::new(format!("{message} near {:?}", self.peek()))
    }

    /// Consumes `expected`, or returns an error.
    fn expect(&mut self, expected: Token) -> Result<(), DwError> {
        if self.peek() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(self.error("unexpected token"))
        }
    }

    /// Returns the current token as an identifier, if it is one.
    fn ident(&self) -> Option<&str> {
        match self.peek() {
            Token::Ident(name) => Some(name),
            _ => None,
        }
    }

    /// Parses an expression using precedence climbing with the given minimum binding power.
    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, DwError> {
        let mut left = self.parse_prefix()?;
        loop {
            match self.peek().clone() {
                Token::Dot => {
                    self.advance();
                    if matches!(self.peek(), Token::Op(op) if op == "^") {
                        self.advance();
                        left = match self.peek().clone() {
                            Token::Ident(name) => {
                                self.advance();
                                Expr::MetaKey(Box::new(left), name)
                            }
                            Token::Str(text) => {
                                self.advance();
                                Expr::MetaKey(Box::new(left), text)
                            }
                            _ => Expr::MetaAll(Box::new(left)),
                        };
                        continue;
                    }
                    let name = match self.advance() {
                        Token::Ident(name) => name,
                        other => {
                            return Err(DwError::new(format!(
                                "expected field name, found {other:?}"
                            )))
                        }
                    };
                    left = Expr::Select(Box::new(left), name);
                }
                Token::Question => {
                    self.advance();
                    left = match left {
                        Expr::Select(target, field) => Expr::SafeSelect(target, field),
                        Expr::Index(target, index) => Expr::SafeIndex(target, index),
                        _ => return Err(self.error("unexpected '?'")),
                    };
                }
                Token::LBracket => {
                    self.advance();
                    let index = self.parse_expr(0)?;
                    self.expect(Token::RBracket)?;
                    left = Expr::Index(Box::new(left), Box::new(index));
                }
                Token::LParen => {
                    if matches!(left, Expr::Array(_) | Expr::Object(_)) {
                        break;
                    }
                    self.advance();
                    let mut args = Vec::new();
                    if self.peek() != &Token::RParen {
                        loop {
                            args.push(self.parse_expr(0)?);
                            if self.peek() == &Token::Comma {
                                self.advance();
                                if self.peek() == &Token::RParen {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen)?;
                    if let Expr::Infix(name, first, second) = &left {
                        if matches!(name.as_str(), "join" | "leftJoin" | "outerJoin") {
                            let mut call_args = vec![(**first).clone(), (**second).clone()];
                            call_args.extend(args);
                            left = Expr::Call(Box::new(Expr::Var(name.clone())), call_args);
                            continue;
                        }
                    }
                    left = Expr::Call(Box::new(left), args);
                }
                Token::Op(op) => {
                    let (lbp, rbp, binary) = match op.as_str() {
                        "+" => (70, 71, BinOp::Add),
                        "-" => (70, 71, BinOp::Subtract),
                        "*" => (80, 81, BinOp::Multiply),
                        "/" => (80, 81, BinOp::Divide),
                        "%" => (80, 81, BinOp::Modulo),
                        "++" => (10, 11, BinOp::Concat),
                        "<<" => (10, 11, BinOp::Append),
                        "--" => (10, 11, BinOp::Difference),
                        "==" => (50, 51, BinOp::Equal),
                        "!=" => (50, 51, BinOp::NotEqual),
                        "<" => (60, 61, BinOp::Less),
                        "<=" => (60, 61, BinOp::LessEqual),
                        ">" => (60, 61, BinOp::Greater),
                        ">=" => (60, 61, BinOp::GreaterEqual),
                        _ => break,
                    };
                    if lbp < min_bp {
                        break;
                    }
                    self.advance();
                    let right = self.parse_expr(rbp)?;
                    left = Expr::Binary(binary, Box::new(left), Box::new(right));
                }
                Token::Ident(name) => {
                    if name == "and" {
                        if 40 < min_bp {
                            break;
                        }
                        self.advance();
                        let right = self.parse_expr(41)?;
                        left = Expr::Binary(BinOp::And, Box::new(left), Box::new(right));
                    } else if name == "or" {
                        if 30 < min_bp {
                            break;
                        }
                        self.advance();
                        let right = self.parse_expr(31)?;
                        left = Expr::Binary(BinOp::Or, Box::new(left), Box::new(right));
                    } else if name == "default" {
                        if 5 < min_bp {
                            break;
                        }
                        self.advance();
                        let right = self.parse_expr(6)?;
                        left = Expr::Default(Box::new(left), Box::new(right));
                    } else if name == "as" {
                        if 65 < min_bp {
                            break;
                        }
                        self.advance();
                        let type_name = match self.advance() {
                            Token::Ident(name) => name,
                            other => {
                                return Err(DwError::new(format!(
                                    "expected type name, found {other:?}"
                                )))
                            }
                        };
                        let options = if self.peek() == &Token::LBrace {
                            Some(Box::new(self.parse_object()?))
                        } else {
                            None
                        };
                        left = Expr::As(Box::new(left), type_name, options);
                    } else if name == "is" {
                        if 60 < min_bp {
                            break;
                        }
                        self.advance();
                        let type_name = match self.advance() {
                            Token::Ident(name) => name,
                            other => {
                                return Err(DwError::new(format!(
                                    "expected type name, found {other:?}"
                                )))
                            }
                        };
                        left = Expr::Is(Box::new(left), type_name);
                    } else if name == "match"
                        && matches!(self.tokens.get(self.pos + 1), Some(Token::LBrace))
                    {
                        if 20 < min_bp {
                            break;
                        }
                        self.advance();
                        left = self.parse_match(left)?;
                    } else if is_reserved(&name) {
                        break;
                    } else if name == "with" && is_with_call(&left) {
                        self.advance();
                        let arg = self.parse_expr(0)?;
                        left = append_replace_arg(left, vec![arg])?;
                        continue;
                    } else {
                        let next_is_terminator = match self.tokens.get(self.pos + 1) {
                            Some(Token::Eof)
                            | Some(Token::Comma)
                            | Some(Token::RBrace)
                            | Some(Token::RBracket)
                            | Some(Token::RParen)
                            | Some(Token::Colon)
                            | Some(Token::Semi) => true,
                            Some(Token::Ident(next)) if next == "else" => true,
                            _ => false,
                        };
                        if next_is_terminator {
                            self.advance();
                            left = Expr::Call(Box::new(Expr::Var(name.clone())), vec![left]);
                            continue;
                        }
                        if 20 < min_bp {
                            break;
                        }
                        self.advance();
                        let right = self.parse_expr(21)?;
                        left = Expr::Infix(name.clone(), Box::new(left), Box::new(right));
                    }
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// Parses a literal, prefix operator, grouped expression, or collection.
    fn parse_prefix(&mut self) -> Result<Expr, DwError> {
        match self.peek().clone() {
            Token::Number(text) => {
                self.advance();
                let number = Number::parse(&text)
                    .ok_or_else(|| DwError::new(format!("invalid number {text}")))?;
                Ok(Expr::Number(number))
            }
            Token::Str(text) => {
                self.advance();
                Ok(Expr::String(text))
            }
            Token::Literal(text) => {
                self.advance();
                Ok(Expr::Literal(text))
            }
            Token::Regex(pattern) => {
                self.advance();
                Ok(Expr::Regex(pattern))
            }
            Token::Interp(parts) => {
                self.advance();
                let mut segments = Vec::new();
                for part in parts {
                    match part {
                        InterpPart::Text(text) => segments.push(InterpSegment::Text(text)),
                        InterpPart::Expr(text) => {
                            segments.push(InterpSegment::Expr(parse_expression(&text)?));
                        }
                    }
                }
                Ok(Expr::Interp(segments))
            }
            Token::Ident(name) => match name.as_str() {
                "null" => {
                    self.advance();
                    Ok(Expr::Null)
                }
                "true" => {
                    self.advance();
                    Ok(Expr::Bool(true))
                }
                "false" => {
                    self.advance();
                    Ok(Expr::Bool(false))
                }
                "if" => self.parse_if(),
                "do" => self.parse_do(),
                "using" => self.parse_using(),
                "not" => {
                    self.advance();
                    let inner = self.parse_expr(90)?;
                    Ok(Expr::Unary(UnOp::Not, Box::new(inner)))
                }
                "else" | "and" | "or" | "default" | "as" => Err(self.error("unexpected keyword")),
                _ => {
                    self.advance();
                    Ok(Expr::Var(name))
                }
            },
            Token::Op(op) if op == "-" => {
                self.advance();
                let inner = self.parse_expr(90)?;
                Ok(Expr::Unary(UnOp::Negate, Box::new(inner)))
            }
            Token::Op(op) if op == "+" => {
                self.advance();
                let inner = self.parse_expr(90)?;
                Ok(inner)
            }
            Token::Op(op) if op == "!" => {
                self.advance();
                let inner = self.parse_expr(90)?;
                Ok(Expr::Unary(UnOp::Not, Box::new(inner)))
            }
            Token::LParen => self.parse_paren(),
            Token::LBracket => self.parse_array(),
            Token::LBrace => self.parse_object(),
            _ => Err(self.error("unexpected token")),
        }
    }

    /// Parses `match { case pattern -> body ... else -> body }`.
    fn parse_match(&mut self, subject: Expr) -> Result<Expr, DwError> {
        self.expect(Token::LBrace)?;
        let mut arms = Vec::new();
        loop {
            if self.ident() == Some("case") {
                self.advance();
                let pattern = self.parse_expr(0)?;
                validate_match_pattern(&pattern)?;
                self.expect(Token::Arrow)?;
                let body = self.parse_expr(0)?;
                arms.push((pattern, body));
            } else if self.ident() == Some("else")
                && matches!(self.tokens.get(self.pos + 1), Some(Token::Arrow))
            {
                self.advance();
                self.advance();
                let body = self.parse_expr(0)?;
                arms.push((Expr::Var("_".to_string()), body));
            } else {
                break;
            }
            if self.peek() == &Token::Comma {
                self.advance();
            }
        }
        self.expect(Token::RBrace)?;
        Ok(Expr::Match(Box::new(subject), arms))
    }

    /// Parses `do { var ... --- body }`, requiring at least one declaration.
    fn parse_do(&mut self) -> Result<Expr, DwError> {
        self.advance();
        self.expect(Token::LBrace)?;
        let mut declarations = Vec::new();
        loop {
            if self.peek() == &Token::DoSeparator {
                self.advance();
                break;
            }
            if self.peek() == &Token::RBrace {
                return Err(self.error("expected '---' in do block"));
            }
            if self.ident() != Some("var") {
                return Err(self.error("expected 'var' or '---' in do block"));
            }
            self.advance();
            let name = match self.advance() {
                Token::Ident(name) => name,
                other => {
                    return Err(DwError::new(format!(
                        "expected variable name, found {other:?}"
                    )))
                }
            };
            if self.peek() == &Token::Colon {
                self.advance();
                match self.advance() {
                    Token::Ident(_) => {}
                    other => {
                        return Err(DwError::new(format!("expected type name, found {other:?}")))
                    }
                }
            }
            if !matches!(self.peek(), Token::Op(op) if op == "=") {
                return Err(self.error("expected '=' in variable declaration"));
            }
            self.advance();
            let start = self.pos;
            let mut index = self.pos;
            let mut depth = 0i32;
            loop {
                match self.tokens.get(index) {
                    Some(Token::DoSeparator) | Some(Token::RBrace) if depth == 0 => break,
                    Some(Token::Ident(word)) if word == "var" && depth == 0 => break,
                    Some(Token::LParen) | Some(Token::LBracket) | Some(Token::LBrace) => {
                        depth += 1;
                    }
                    Some(Token::RParen) | Some(Token::RBracket) => depth -= 1,
                    Some(Token::RBrace) => depth -= 1,
                    Some(Token::Eof) | None => {
                        return Err(self.error("unterminated do block"));
                    }
                    _ => {}
                }
                index += 1;
            }
            let value = parse_tokens(&self.tokens[start..index])?;
            self.pos = index;
            declarations.push((name, value));
        }
        if declarations.is_empty() {
            return Err(DwError::new("do block requires a declaration"));
        }
        let start = self.pos;
        let mut index = self.pos;
        let mut depth = 0i32;
        loop {
            match self.tokens.get(index) {
                Some(Token::RBrace) if depth == 0 => break,
                Some(Token::LBrace) => depth += 1,
                Some(Token::RBrace) => depth -= 1,
                Some(Token::Eof) | None => return Err(self.error("unterminated do block")),
                _ => {}
            }
            index += 1;
        }
        let body = parse_tokens(&self.tokens[start..index])?;
        self.pos = index;
        self.expect(Token::RBrace)?;
        Ok(Expr::Do(declarations, Box::new(body)))
    }

    /// Parses `using (name = expression, ...) body`.
    fn parse_using(&mut self) -> Result<Expr, DwError> {
        self.advance();
        self.expect(Token::LParen)?;
        if self.peek() == &Token::RParen {
            return Err(self.error("using requires at least one declaration"));
        }
        let mut declarations: Vec<(String, Expr)> = Vec::new();
        loop {
            let name = match self.advance() {
                Token::Ident(name) if !is_reserved(&name) => name,
                other => {
                    return Err(DwError::new(format!(
                        "expected variable name, found {other:?}"
                    )))
                }
            };
            if self.peek() == &Token::Colon {
                return Err(DwError::new(
                    "typed using declarations are not supported",
                ));
            }
            if !matches!(self.peek(), Token::Op(op) if op == "=") {
                return Err(self.error("expected '=' in using declaration"));
            }
            self.advance();
            let value = self.parse_expr(0)?;
            if declarations.iter().any(|(existing, _)| existing == &name) {
                return Err(DwError::new(format!("duplicate using name '{name}'")));
            }
            declarations.push((name, value));
            match self.peek().clone() {
                Token::Comma => {
                    self.advance();
                    if self.peek() == &Token::RParen {
                        return Err(self.error("trailing comma in using"));
                    }
                }
                Token::RParen => break,
                _ => return Err(self.error("expected ',' or ')' in using")),
            }
        }
        self.advance();
        let body = self.parse_expr(0)?;
        Ok(Expr::Using(declarations, Box::new(body)))
    }

    /// Parses `if (condition) then else otherwise`.
    fn parse_if(&mut self) -> Result<Expr, DwError> {
        self.advance();
        let condition = if self.peek() == &Token::LParen {
            self.advance();
            let condition = self.parse_expr(0)?;
            self.expect(Token::RParen)?;
            condition
        } else {
            self.parse_expr(0)?
        };
        let then_branch = self.parse_expr(0)?;
        if self.ident() != Some("else") {
            return Err(self.error("expected 'else'"));
        }
        self.advance();
        let else_branch = self.parse_expr(0)?;
        Ok(Expr::If(
            Box::new(condition),
            Box::new(then_branch),
            Box::new(else_branch),
        ))
    }

    /// Parses a parenthesized expression or lambda.
    fn parse_paren(&mut self) -> Result<Expr, DwError> {
        let saved = self.pos;
        if let Some(lambda) = self.try_lambda()? {
            return Ok(lambda);
        }
        self.pos = saved;
        self.advance();
        let expr = self.parse_expr(0)?;
        self.expect(Token::RParen)?;
        Ok(expr)
    }

    /// Tries to parse a parenthesized lambda, returning `None` if the tokens do not match.
    fn try_lambda(&mut self) -> Result<Option<Expr>, DwError> {
        self.advance();
        let mut params = Vec::new();
        if self.peek() != &Token::RParen {
            loop {
                match self.advance() {
                    Token::Ident(name) if !is_reserved(&name) => {
                        let mut type_name = None;
                        if self.peek() == &Token::Colon {
                            self.advance();
                            match self.advance() {
                                Token::Ident(name) => type_name = Some(name),
                                _ => return Ok(None),
                            }
                        }
                        let mut default = None;
                        if matches!(self.peek(), Token::Op(op) if op == "=") {
                            self.advance();
                            default = Some(self.parse_expr(0)?);
                        }
                        params.push(Param {
                            name,
                            default,
                            type_name,
                        });
                    }
                    _ => return Ok(None),
                }
                match self.advance() {
                    Token::Comma => {
                        if self.peek() == &Token::RParen {
                            break;
                        }
                    }
                    Token::RParen => break,
                    _ => return Ok(None),
                }
            }
        } else {
            self.advance();
        }
        if self.peek() != &Token::Arrow {
            return Ok(None);
        }
        self.advance();
        let body = self.parse_expr(0)?;
        Ok(Some(Expr::Lambda(params, Box::new(body))))
    }

    /// Parses an array literal, including array cons with `~`.
    fn parse_array(&mut self) -> Result<Expr, DwError> {
        self.advance();
        if self.peek() == &Token::RBracket {
            self.advance();
            return Ok(Expr::Array(Vec::new()));
        }
        let first = self.parse_expr(0)?;
        if matches!(self.peek(), Token::Op(op) if op == "~") {
            self.advance();
            let tail = self.parse_expr(0)?;
            self.expect(Token::RBracket)?;
            return Ok(Expr::ArrayCons(Box::new(first), Box::new(tail)));
        }
        let mut items = vec![first];
        loop {
            match self.peek().clone() {
                Token::Comma => {
                    self.advance();
                    if self.peek() == &Token::RBracket {
                        self.advance();
                        break;
                    }
                    items.push(self.parse_expr(0)?);
                }
                Token::RBracket => {
                    self.advance();
                    break;
                }
                _ => return Err(self.error("expected ',' or ']'")),
            }
        }
        Ok(Expr::Array(items))
    }

    /// Parses an object literal with static or computed keys.
    fn parse_object(&mut self) -> Result<Expr, DwError> {
        self.advance();
        let mut entries = Vec::new();
        if self.peek() == &Token::RBrace {
            self.advance();
            return Ok(Expr::Object(entries));
        }
        loop {
            let key = match self.peek().clone() {
                Token::Ident(name) => {
                    self.advance();
                    if self.peek() != &Token::Colon {
                        return Err(self.error("expected ':' after key"));
                    }
                    Expr::String(name)
                }
                Token::Str(text) => {
                    self.advance();
                    if self.peek() != &Token::Colon {
                        return Err(self.error("expected ':' after key"));
                    }
                    Expr::String(text)
                }
                Token::Number(text) => {
                    self.advance();
                    if self.peek() != &Token::Colon {
                        return Err(self.error("expected ':' after key"));
                    }
                    Expr::String(text)
                }
                Token::LParen => {
                    self.advance();
                    let key = self.parse_expr(0)?;
                    self.expect(Token::RParen)?;
                    if self.peek() != &Token::Colon {
                        return Err(self.error("expected ':' after key"));
                    }
                    key
                }
                _ => return Err(self.error("expected object key")),
            };
            self.advance();
            let value = self.parse_expr(0)?;
            if matches!(self.peek(), Token::Op(op) if op == "~") {
                if !entries.is_empty() {
                    return Err(self.error("object spread requires exactly one head entry"));
                }
                self.advance();
                let tail = self.parse_expr(0)?;
                self.expect(Token::RBrace)?;
                return Ok(Expr::ObjectSpread(
                    Box::new(key),
                    Box::new(value),
                    Box::new(tail),
                ));
            }
            entries.push((key, value));
            match self.peek().clone() {
                Token::Comma => {
                    self.advance();
                    if self.peek() == &Token::RBrace {
                        self.advance();
                        break;
                    }
                }
                Token::RBrace => {
                    self.advance();
                    break;
                }
                _ => return Err(self.error("expected ',' or '}'")),
            }
        }
        Ok(Expr::Object(entries))
    }
}

/// Rejects match patterns outside literals, `[]`, and `[head ~ tail]`.
fn validate_match_pattern(pattern: &Expr) -> Result<(), DwError> {
    match pattern {
        Expr::Null | Expr::Bool(_) | Expr::Number(_) | Expr::String(_) => Ok(()),
        Expr::Array(items) if items.is_empty() => Ok(()),
        Expr::ArrayCons(head, tail) => {
            validate_match_element(head)?;
            validate_match_tail(tail)
        }
        _ => Err(DwError::new("unsupported match pattern")),
    }
}

/// Validates the head element of an array-cons pattern.
fn validate_match_element(pattern: &Expr) -> Result<(), DwError> {
    match pattern {
        Expr::Var(_) | Expr::Null | Expr::Bool(_) | Expr::Number(_) | Expr::String(_) => Ok(()),
        Expr::Array(items) if items.is_empty() => Ok(()),
        Expr::ArrayCons(head, tail) => {
            validate_match_element(head)?;
            validate_match_tail(tail)
        }
        _ => Err(DwError::new("unsupported match pattern")),
    }
}

/// Validates the tail of an array-cons pattern.
fn validate_match_tail(pattern: &Expr) -> Result<(), DwError> {
    match pattern {
        Expr::Var(_) => Ok(()),
        Expr::Array(items) if items.is_empty() => Ok(()),
        Expr::ArrayCons(head, tail) => {
            validate_match_element(head)?;
            validate_match_tail(tail)
        }
        _ => Err(DwError::new("unsupported match pattern")),
    }
}

/// Returns whether an expression is a call that accepts a trailing `with` argument.
fn is_with_call(expr: &Expr) -> bool {
    let accepted = |name: &str| matches!(name, "replace" | "replaceAll" | "update" | "mask");
    match expr {
        Expr::Infix(name, _, _) => accepted(name),
        Expr::Call(callee, _) => {
            matches!(callee.as_ref(), Expr::Var(name) if accepted(name))
        }
        _ => false,
    }
}

/// Appends the argument following `with` to a replacement call.
fn append_replace_arg(expr: Expr, mut args: Vec<Expr>) -> Result<Expr, DwError> {
    match expr {
        Expr::Infix(name, first, second) => {
            let mut call_args = vec![*first, *second];
            call_args.append(&mut args);
            Ok(Expr::Call(Box::new(Expr::Var(name)), call_args))
        }
        Expr::Call(callee, mut existing) => {
            existing.append(&mut args);
            Ok(Expr::Call(callee, existing))
        }
        _ => Err(DwError::new("unexpected use of with")),
    }
}

/// Returns whether `name` is a reserved word that cannot start an expression.
fn is_reserved(name: &str) -> bool {
    matches!(
        name,
        "if" | "else"
            | "and"
            | "or"
            | "not"
            | "default"
            | "as"
            | "is"
            | "case"
            | "null"
            | "true"
            | "false"
            | "using"
    )
}
