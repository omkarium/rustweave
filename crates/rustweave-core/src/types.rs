// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Type values and the type-expression parser used by `dw::core::Types`.

use crate::error::DwError;
use crate::number::Number;
use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeField {
    pub name: String,
    pub namespace: Option<String>,
    pub value: TypeValue,
    pub required: bool,
    pub repeated: bool,
    pub attributes: Vec<TypeField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeParam {
    pub value: TypeValue,
    pub optional: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeValue {
    Any,
    Nothing,
    Null,
    Boolean,
    Number,
    Integer,
    Decimal,
    String,
    Date,
    DateTime,
    LocalDateTime,
    LocalTime,
    Time,
    TimeZone,
    Period,
    Binary,
    Regex,
    Uri,
    Key,
    Namespace,
    Type,
    Range,
    Array(Box<TypeValue>),
    Object(Vec<TypeField>),
    Function(Vec<TypeParam>, Box<TypeValue>),
    Union(Vec<TypeValue>),
    Intersection(Vec<TypeValue>),
    Literal(Value),
    Ref(String),
    Declared { name: String, inner: Box<TypeValue> },
    Meta(Box<TypeValue>, Vec<(String, Value)>),
}

pub fn builtin(name: &str) -> Option<TypeValue> {
    Some(match name {
        "Any" => TypeValue::Any,
        "Nothing" => TypeValue::Nothing,
        "Null" => TypeValue::Null,
        "Boolean" => TypeValue::Boolean,
        "Number" => TypeValue::Number,
        "Integer" => TypeValue::Integer,
        "Decimal" => TypeValue::Decimal,
        "String" => TypeValue::String,
        "Date" => TypeValue::Date,
        "DateTime" => TypeValue::DateTime,
        "LocalDateTime" => TypeValue::LocalDateTime,
        "LocalTime" => TypeValue::LocalTime,
        "Time" => TypeValue::Time,
        "TimeZone" => TypeValue::TimeZone,
        "Period" => TypeValue::Period,
        "Binary" => TypeValue::Binary,
        "Regex" => TypeValue::Regex,
        "Uri" | "URI" => TypeValue::Uri,
        "Key" => TypeValue::Key,
        "Namespace" => TypeValue::Namespace,
        "Type" => TypeValue::Type,
        "Range" => TypeValue::Range,
        "Array" => TypeValue::Array(Box::new(TypeValue::Any)),
        "Object" => TypeValue::Object(Vec::new()),
        "Function" => TypeValue::Function(Vec::new(), Box::new(TypeValue::Any)),
        _ => return None,
    })
}

impl TypeValue {
    pub fn structure(&self) -> &TypeValue {
        match self {
            TypeValue::Declared { inner, .. } => inner.structure(),
            TypeValue::Meta(inner, _) => inner.structure(),
            other => other,
        }
    }

    pub fn declared_name(&self) -> Option<&str> {
        match self {
            TypeValue::Declared { name, .. } => Some(name),
            _ => None,
        }
    }

    pub fn metadata(&self) -> Vec<(String, Value)> {
        match self {
            TypeValue::Declared { inner, .. } => inner.metadata(),
            TypeValue::Meta(_, entries) => entries.clone(),
            _ => Vec::new(),
        }
    }

    pub fn display(&self) -> String {
        match self {
            TypeValue::Any => "Any".to_string(),
            TypeValue::Nothing => "Nothing".to_string(),
            TypeValue::Null => "Null".to_string(),
            TypeValue::Boolean => "Boolean".to_string(),
            TypeValue::Number => "Number".to_string(),
            TypeValue::Integer => "Integer".to_string(),
            TypeValue::Decimal => "Decimal".to_string(),
            TypeValue::String => "String".to_string(),
            TypeValue::Date => "Date".to_string(),
            TypeValue::DateTime => "DateTime".to_string(),
            TypeValue::LocalDateTime => "LocalDateTime".to_string(),
            TypeValue::LocalTime => "LocalTime".to_string(),
            TypeValue::Time => "Time".to_string(),
            TypeValue::TimeZone => "TimeZone".to_string(),
            TypeValue::Period => "Period".to_string(),
            TypeValue::Binary => "Binary".to_string(),
            TypeValue::Regex => "Regex".to_string(),
            TypeValue::Uri => "Uri".to_string(),
            TypeValue::Key => "Key".to_string(),
            TypeValue::Namespace => "Namespace".to_string(),
            TypeValue::Type => "Type".to_string(),
            TypeValue::Range => "Range".to_string(),
            TypeValue::Array(item) => format!("Array<{}>", item.display()),
            TypeValue::Object(_) => "Object".to_string(),
            TypeValue::Function(params, result) => {
                let params = params
                    .iter()
                    .map(|param| param.value.display())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({params}) -> {}", result.display())
            }
            TypeValue::Union(items) => items
                .iter()
                .map(TypeValue::display)
                .collect::<Vec<_>>()
                .join("|"),
            TypeValue::Intersection(items) => items
                .iter()
                .map(TypeValue::display)
                .collect::<Vec<_>>()
                .join(" & "),
            TypeValue::Literal(value) => value.key_string(),
            TypeValue::Ref(name) => name.clone(),
            TypeValue::Declared { name, .. } => name.clone(),
            TypeValue::Meta(inner, _) => inner.display(),
        }
    }

    pub fn base(&self) -> TypeValue {
        match self {
            TypeValue::Declared { inner, .. } => inner.base(),
            TypeValue::Meta(inner, _) => inner.base(),
            TypeValue::Literal(value) => match value {
                Value::String(_) => TypeValue::String,
                Value::Number(number) if number.is_integer() => TypeValue::Integer,
                Value::Number(_) => TypeValue::Decimal,
                Value::Bool(_) => TypeValue::Boolean,
                Value::Null => TypeValue::Null,
                _ => TypeValue::Any,
            },
            other => other.clone(),
        }
    }
}

impl std::fmt::Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display())
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Ident(String),
    String(String),
    Number(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Lt,
    Gt,
    Comma,
    Colon,
    Question,
    Star,
    At,
    Arrow,
    Pipe,
    Amp,
    Hash,
    Eq,
    Eof,
}

fn lex_type(text: &str) -> Result<Vec<Token>, DwError> {
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;
    let mut out = Vec::new();
    while index < chars.len() {
        let c = chars[index];
        match c {
            ' ' | '\t' | '\r' | '\n' => index += 1,
            '(' => {
                out.push(Token::LParen);
                index += 1;
            }
            ')' => {
                out.push(Token::RParen);
                index += 1;
            }
            '{' => {
                out.push(Token::LBrace);
                index += 1;
            }
            '}' => {
                out.push(Token::RBrace);
                index += 1;
            }
            '[' => {
                out.push(Token::LBracket);
                index += 1;
            }
            ']' => {
                out.push(Token::RBracket);
                index += 1;
            }
            '<' => {
                out.push(Token::Lt);
                index += 1;
            }
            '>' => {
                out.push(Token::Gt);
                index += 1;
            }
            ',' => {
                out.push(Token::Comma);
                index += 1;
            }
            ':' => {
                out.push(Token::Colon);
                index += 1;
            }
            '?' => {
                out.push(Token::Question);
                index += 1;
            }
            '*' => {
                out.push(Token::Star);
                index += 1;
            }
            '@' => {
                out.push(Token::At);
                index += 1;
            }
            '#' => {
                out.push(Token::Hash);
                index += 1;
            }
            '=' => {
                out.push(Token::Eq);
                index += 1;
            }
            '|' => {
                out.push(Token::Pipe);
                index += 1;
            }
            '&' => {
                out.push(Token::Amp);
                index += 1;
            }
            '-' if chars.get(index + 1) == Some(&'>') => {
                out.push(Token::Arrow);
                index += 2;
            }
            '"' | '\'' => {
                let quote = c;
                index += 1;
                let mut value = String::new();
                while index < chars.len() {
                    let c = chars[index];
                    index += 1;
                    if c == quote {
                        break;
                    }
                    if c == '\\' {
                        if let Some(next) = chars.get(index).copied() {
                            value.push(next);
                            index += 1;
                        }
                    } else {
                        value.push(c);
                    }
                }
                out.push(Token::String(value));
            }
            '0'..='9' | '-' if c == '-' || c.is_ascii_digit() => {
                let start = index;
                if c == '-' {
                    index += 1;
                }
                while index < chars.len() && (chars[index].is_ascii_digit() || chars[index] == '.')
                {
                    index += 1;
                }
                out.push(Token::Number(chars[start..index].iter().collect()));
            }
            c if c.is_alphanumeric() || c == '_' => {
                let start = index;
                while index < chars.len() && (chars[index].is_alphanumeric() || chars[index] == '_')
                {
                    index += 1;
                }
                let text: String = chars[start..index].iter().collect();
                out.push(match text.as_str() {
                    "true" => Token::Ident("true".to_string()),
                    "false" => Token::Ident("false".to_string()),
                    "null" => Token::Ident("null".to_string()),
                    _ => Token::Ident(text),
                });
            }
            _ => index += 1,
        }
    }
    out.push(Token::Eof);
    Ok(out)
}

struct TypeParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl TypeParser {
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        token
    }

    fn eat(&mut self, token: &Token) -> bool {
        if self.peek() == token {
            self.advance();
            true
        } else {
            false
        }
    }

    fn type_expr(&mut self) -> Result<TypeValue, DwError> {
        let mut items = vec![self.intersection()?];
        while self.eat(&Token::Pipe) {
            items.push(self.intersection()?);
        }
        if items.len() == 1 {
            Ok(items.remove(0))
        } else {
            Ok(TypeValue::Union(items))
        }
    }

    fn intersection(&mut self) -> Result<TypeValue, DwError> {
        let mut items = vec![self.postfix()?];
        while self.eat(&Token::Amp) {
            items.push(self.postfix()?);
        }
        if items.len() == 1 {
            Ok(items.remove(0))
        } else {
            Ok(TypeValue::Intersection(items))
        }
    }

    fn postfix(&mut self) -> Result<TypeValue, DwError> {
        let mut value = self.primary()?;
        while self.peek() == &Token::LBrace {
            let metadata = self.metadata()?;
            value = TypeValue::Meta(Box::new(value), metadata);
        }
        Ok(value)
    }

    fn primary(&mut self) -> Result<TypeValue, DwError> {
        match self.advance() {
            Token::LParen => {
                let mut params = Vec::new();
                if !self.eat(&Token::RParen) {
                    loop {
                        let value = self.type_expr()?;
                        let optional = self.eat(&Token::Question);
                        params.push(TypeParam { value, optional });
                        if self.eat(&Token::Comma) {
                            continue;
                        }
                        if self.eat(&Token::RParen) {
                            break;
                        }
                        return Err(DwError::new("invalid function type parameters"));
                    }
                }
                if !self.eat(&Token::Arrow) {
                    return Err(DwError::new("function type requires '->'"));
                }
                let result = self.type_expr()?;
                Ok(TypeValue::Function(params, Box::new(result)))
            }
            Token::LBrace => self.object_type(),
            Token::String(text) => Ok(TypeValue::Literal(Value::String(text))),
            Token::Number(text) => Number::parse(&text)
                .map(|number| TypeValue::Literal(Value::Number(number)))
                .ok_or_else(|| DwError::new("invalid numeric literal type")),
            Token::Ident(name) => match name.as_str() {
                "true" => Ok(TypeValue::Literal(Value::Bool(true))),
                "false" => Ok(TypeValue::Literal(Value::Bool(false))),
                "null" => Ok(TypeValue::Literal(Value::Null)),
                "Array" if self.peek() == &Token::Lt => {
                    self.advance();
                    let item = self.type_expr()?;
                    if !self.eat(&Token::Gt) {
                        return Err(DwError::new("unterminated Array type"));
                    }
                    Ok(TypeValue::Array(Box::new(item)))
                }
                _ => Ok(builtin(&name).unwrap_or(TypeValue::Ref(name))),
            },
            other => Err(DwError::new(format!(
                "invalid type expression near {other:?}"
            ))),
        }
    }

    fn object_type(&mut self) -> Result<TypeValue, DwError> {
        let mut fields = Vec::new();
        if self.eat(&Token::RBrace) {
            return Ok(TypeValue::Object(fields));
        }
        loop {
            let (name, namespace) = self.field_name()?;
            let mut attributes = Vec::new();
            if self.eat(&Token::At) && self.eat(&Token::LParen) && !self.eat(&Token::RParen) {
                loop {
                    attributes.push(self.field()?);
                    if self.eat(&Token::Comma) {
                        continue;
                    }
                    if self.eat(&Token::RParen) {
                        break;
                    }
                    return Err(DwError::new("invalid attribute type"));
                }
            }
            let optional = self.eat(&Token::Question);
            let repeated = self.eat(&Token::Star);
            if !self.eat(&Token::Colon) {
                return Err(DwError::new("object field requires ':'"));
            }
            let value = self.type_expr()?;
            fields.push(TypeField {
                name,
                namespace,
                value,
                required: !optional,
                repeated,
                attributes,
            });
            if self.eat(&Token::Comma) {
                if self.eat(&Token::RBrace) {
                    break;
                }
                continue;
            }
            if self.eat(&Token::RBrace) {
                break;
            }
            return Err(DwError::new("invalid object type"));
        }
        Ok(TypeValue::Object(fields))
    }

    fn field(&mut self) -> Result<TypeField, DwError> {
        let (name, namespace) = self.field_name()?;
        let optional = self.eat(&Token::Question);
        let repeated = self.eat(&Token::Star);
        if !self.eat(&Token::Colon) {
            return Err(DwError::new("field requires ':'"));
        }
        let value = self.type_expr()?;
        Ok(TypeField {
            name,
            namespace,
            value,
            required: !optional,
            repeated,
            attributes: Vec::new(),
        })
    }

    fn field_name(&mut self) -> Result<(String, Option<String>), DwError> {
        let first = match self.advance() {
            Token::Ident(name) | Token::String(name) => name,
            other => {
                return Err(DwError::new(format!(
                    "expected field name, found {other:?}"
                )))
            }
        };
        if self.eat(&Token::Hash) {
            let local = match self.advance() {
                Token::Ident(name) | Token::String(name) => name,
                other => {
                    return Err(DwError::new(format!(
                        "expected field name, found {other:?}"
                    )))
                }
            };
            Ok((local, Some(first)))
        } else {
            Ok((first, None))
        }
    }

    fn metadata(&mut self) -> Result<Vec<(String, Value)>, DwError> {
        if !self.eat(&Token::LBrace) {
            return Err(DwError::new("expected metadata object"));
        }
        let mut entries = Vec::new();
        if self.eat(&Token::RBrace) {
            return Ok(entries);
        }
        loop {
            let key = match self.advance() {
                Token::Ident(name) | Token::String(name) => name,
                other => return Err(DwError::new(format!("invalid metadata key {other:?}"))),
            };
            if !self.eat(&Token::Colon) {
                return Err(DwError::new("metadata entry requires ':'"));
            }
            let value = self.metadata_value()?;
            entries.push((key, value));
            if self.eat(&Token::Comma) {
                if self.eat(&Token::RBrace) {
                    break;
                }
                continue;
            }
            if self.eat(&Token::RBrace) {
                break;
            }
            return Err(DwError::new("invalid metadata object"));
        }
        Ok(entries)
    }

    fn metadata_value(&mut self) -> Result<Value, DwError> {
        match self.advance() {
            Token::String(text) => Ok(Value::String(text)),
            Token::Number(text) => Number::parse(&text)
                .map(Value::Number)
                .ok_or_else(|| DwError::new("invalid metadata number")),
            Token::Ident(name) if name == "true" => Ok(Value::Bool(true)),
            Token::Ident(name) if name == "false" => Ok(Value::Bool(false)),
            Token::Ident(name) if name == "null" => Ok(Value::Null),
            Token::Ident(name) => Ok(Value::String(name)),
            other => Err(DwError::new(format!("invalid metadata value {other:?}"))),
        }
    }
}

pub fn parse_declaration(name: &str, text: &str) -> Result<TypeValue, DwError> {
    let tokens = lex_type(text)?;
    let mut parser = TypeParser { tokens, pos: 0 };
    let inner = parser.type_expr()?;
    if parser.peek() != &Token::Eof {
        return Err(DwError::new("unexpected token in type declaration"));
    }
    Ok(TypeValue::Declared {
        name: name.to_string(),
        inner: Box::new(inner),
    })
}
