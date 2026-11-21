// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::{HashMap, HashSet};

use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

use super::{bool_option, namespace_key, object_entries, string_option, validate_encoding};
use crate::error::DwError;
use crate::value::Value;

struct XmlNode {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<XmlNode>,
    text: String,
}

struct XmlOptions {
    attribute_prefix: String,
    text_key: String,
    namespace_separator: String,
    write_declaration: bool,
    indent: bool,
    default_namespace: Option<String>,
    write_nil_on_null: bool,
    escape_cr: bool,
    skip_null_on: Option<String>,
}

impl XmlOptions {
    fn read(options: Option<&Value>) -> Result<Self, DwError> {
        validate_encoding(options)?;
        Ok(XmlOptions {
            attribute_prefix: string_option(options, "attributePrefix", Some("@"))?
                .unwrap_or_else(|| "@".to_string()),
            text_key: string_option(options, "textKey", Some("_"))?
                .unwrap_or_else(|| "_".to_string()),
            namespace_separator: string_option(options, "namespaceSeparator", Some(":"))?
                .unwrap_or_else(|| ":".to_string()),
            write_declaration: bool_option(options, "writeDeclaration", true)?,
            indent: bool_option(options, "indent", false)?,
            default_namespace: string_option(options, "defaultNamespace", None)?,
            write_nil_on_null: bool_option(options, "writeNilOnNull", false)?,
            escape_cr: bool_option(options, "escapeCR", false)?,
            skip_null_on: string_option(options, "skipNullOn", None)?,
        })
    }

    fn name(&self, name: &str) -> String {
        if self.namespace_separator == ":" {
            name.to_string()
        } else {
            name.replace(':', &self.namespace_separator)
        }
    }
}

fn xml_name(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn xml_start(element: &BytesStart<'_>, options: &XmlOptions) -> Result<XmlNode, DwError> {
    let mut attributes = Vec::new();
    for attribute in element.attributes() {
        let attribute =
            attribute.map_err(|error| DwError::new(format!("invalid XML attribute: {error}")))?;
        let name = options.name(&xml_name(attribute.key.as_ref()));
        let value = attribute
            .unescape_value()
            .map_err(|error| DwError::new(format!("invalid XML attribute value: {error}")))?;
        attributes.push((name, value.into_owned()));
    }
    Ok(XmlNode {
        name: options.name(&xml_name(element.name().as_ref())),
        attributes,
        children: Vec::new(),
        text: String::new(),
    })
}

fn xml_reference(reference: &quick_xml::events::BytesRef<'_>) -> Result<String, DwError> {
    if let Some(character) = reference
        .resolve_char_ref()
        .map_err(|error| DwError::new(format!("invalid XML reference: {error}")))?
    {
        return Ok(character.to_string());
    }
    let name = reference
        .decode()
        .map_err(|error| DwError::new(format!("invalid XML reference: {error}")))?;
    Ok(match name.as_ref() {
        "lt" => "<".to_string(),
        "gt" => ">".to_string(),
        "amp" => "&".to_string(),
        "quot" => "\"".to_string(),
        "apos" => "'".to_string(),
        other => format!("&{other};"),
    })
}

pub(crate) fn read_xml(text: &str, properties: Option<&Value>) -> Result<Value, DwError> {
    let options = XmlOptions::read(properties)?;
    let mut reader = Reader::from_str(text);
    reader.config_mut().trim_text(false);
    let mut root = None;
    let mut stack: Vec<XmlNode> = Vec::new();
    loop {
        match reader
            .read_event()
            .map_err(|error| DwError::new(format!("invalid XML: {error}")))?
        {
            Event::Start(element) => {
                if stack.is_empty() && root.is_some() {
                    return Err(DwError::new("XML must have a single root element"));
                }
                stack.push(xml_start(&element, &options)?);
            }
            Event::Empty(element) => {
                let node = xml_start(&element, &options)?;
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else if root.replace(node).is_some() {
                    return Err(DwError::new("XML must have a single root element"));
                }
            }
            Event::End(_) => {
                let node = stack
                    .pop()
                    .ok_or_else(|| DwError::new("unexpected XML end tag"))?;
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else if root.replace(node).is_some() {
                    return Err(DwError::new("XML must have a single root element"));
                }
            }
            Event::Text(text) => {
                let content = text
                    .xml_content()
                    .map_err(|error| DwError::new(format!("invalid XML text: {error}")))?;
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&content);
                } else if !content.trim().is_empty() {
                    return Err(DwError::new("XML text outside the root element"));
                }
            }
            Event::CData(text) => {
                let content = text
                    .decode()
                    .map_err(|error| DwError::new(format!("invalid XML CDATA: {error}")))?;
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&content);
                }
            }
            Event::GeneralRef(reference) => {
                if let Some(node) = stack.last_mut() {
                    node.text.push_str(&xml_reference(&reference)?);
                }
            }
            Event::Eof => break,
            Event::Comment(_) | Event::Decl(_) | Event::PI(_) | Event::DocType(_) => {}
        }
    }
    if !stack.is_empty() {
        return Err(DwError::new("unclosed XML element"));
    }
    let root = root.ok_or_else(|| DwError::new("XML document has no root element"))?;
    Ok(Value::Object(vec![(
        root.name.clone(),
        xml_node_value(&root, &options),
    )]))
}

fn xml_node_value(node: &XmlNode, options: &XmlOptions) -> Value {
    let text = node.text.trim();
    if node.attributes.is_empty() && node.children.is_empty() {
        return if text.is_empty() {
            Value::Null
        } else {
            Value::String(text.to_string())
        };
    }
    let mut entries: Vec<(String, Value)> = node
        .attributes
        .iter()
        .map(|(name, value)| {
            (
                format!("{}{name}", options.attribute_prefix),
                Value::String(value.clone()),
            )
        })
        .collect();
    if !text.is_empty() {
        entries.push((options.text_key.clone(), Value::String(text.to_string())));
    }
    let mut positions: HashMap<String, usize> = HashMap::new();
    let mut repeated: HashSet<String> = HashSet::new();
    for child in &node.children {
        let value = xml_node_value(child, options);
        if let Some(&position) = positions.get(&child.name) {
            if repeated.insert(child.name.clone()) {
                let previous = std::mem::replace(&mut entries[position].1, Value::Null);
                entries[position].1 = Value::Array(vec![previous, value]);
            } else if let Value::Array(items) = &mut entries[position].1 {
                items.push(value);
            }
        } else {
            positions.insert(child.name.clone(), entries.len());
            entries.push((child.name.clone(), value));
        }
    }
    Value::Object(entries)
}

fn escape_xml(text: &str, escape_cr: bool) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            '\r' if escape_cr => out.push_str("&#xd;"),
            other => out.push(other),
        }
    }
    out
}

fn xml_scalar(value: &Value) -> Result<String, DwError> {
    match value {
        Value::Null => Ok(String::new()),
        Value::Array(_) | Value::Object(_) => Err(DwError::new(
            "XML text and attributes must be scalar values",
        )),
        other => Ok(other.key_string()),
    }
}

fn write_xml_element(
    out: &mut String,
    name: &str,
    value: &Value,
    options: &XmlOptions,
    namespaces: &[(String, String)],
    scope: &[String],
    level: usize,
) -> Result<(), DwError> {
    if let Value::Array(items) = value {
        for item in items {
            write_xml_element(out, name, item, options, namespaces, scope, level)?;
        }
        return Ok(());
    }

    let mut attributes = Vec::new();
    let mut text = None;
    let mut children = Vec::new();
    let mut child_scope = scope.to_vec();
    let element_name = match namespace_key(name, namespaces) {
        Some((prefix, local)) => {
            if !scope.iter().any(|existing| existing == prefix) {
                let uri = namespaces
                    .iter()
                    .find(|(declared, _)| declared == prefix)
                    .map(|(_, uri)| uri.clone())
                    .unwrap_or_default();
                attributes.push((format!("xmlns:{prefix}"), uri));
                child_scope.push(prefix.to_string());
            }
            format!("{prefix}:{local}")
        }
        None => name.to_string(),
    };
    if level == 0 {
        if let Some(uri) = &options.default_namespace {
            attributes.push(("xmlns".to_string(), uri.clone()));
        }
    }
    match value {
        Value::Object(entries) => {
            for (key, item) in entries {
                if let Some(attribute) = key.strip_prefix(&options.attribute_prefix) {
                    let attribute_value = xml_scalar(item)?;
                    attributes.push((attribute.to_string(), attribute_value));
                } else if key == &options.text_key {
                    text = Some(xml_scalar(item)?);
                } else if !(item.is_null()
                    && matches!(
                        options.skip_null_on.as_deref(),
                        Some("elements" | "everywhere")
                    ))
                {
                    children.push((key.as_str(), item));
                }
            }
        }
        Value::Null => {
            if options.write_nil_on_null {
                if !child_scope.iter().any(|existing| existing == "xsi") {
                    attributes.push((
                        "xmlns:xsi".to_string(),
                        "http://www.w3.org/2001/XMLSchema-instance".to_string(),
                    ));
                    child_scope.push("xsi".to_string());
                }
                attributes.push(("xsi:nil".to_string(), "true".to_string()));
            }
        }
        other => text = Some(xml_scalar(other)?),
    }

    let has_body = !children.is_empty()
        || text
            .as_deref()
            .map(|text| !text.is_empty())
            .unwrap_or(false);
    if options.indent && level > 0 {
        out.push('\n');
        out.push_str(&"  ".repeat(level));
    }
    out.push('<');
    out.push_str(&element_name);
    for (attribute, value) in &attributes {
        out.push(' ');
        out.push_str(attribute);
        out.push_str("=\"");
        out.push_str(&escape_xml(value, options.escape_cr));
        out.push('"');
    }
    if !has_body {
        out.push_str("/>");
        return Ok(());
    }
    out.push('>');
    if let Some(text) = text {
        out.push_str(&escape_xml(&text, options.escape_cr));
    }
    let had_children = !children.is_empty();
    for (child_name, child) in children {
        write_xml_element(
            out,
            child_name,
            child,
            options,
            namespaces,
            &child_scope,
            level + 1,
        )?;
    }
    if options.indent && had_children {
        out.push('\n');
        out.push_str(&"  ".repeat(level));
    }
    out.push_str("</");
    out.push_str(&element_name);
    out.push('>');
    Ok(())
}

pub(crate) fn write_xml(
    value: &Value,
    properties: Option<&Value>,
    namespaces: &[(String, String)],
) -> Result<String, DwError> {
    let options = XmlOptions::read(properties)?;
    let all_entries = object_entries(value)?;
    let entries: Vec<&(String, Value)> = all_entries
        .iter()
        .filter(|(_, item)| {
            !(item.is_null()
                && matches!(
                    options.skip_null_on.as_deref(),
                    Some("elements" | "everywhere")
                ))
        })
        .collect();
    if entries.len() != 1 {
        return Err(DwError::new(
            "XML output requires an object with exactly one root key",
        ));
    }
    let (name, root) = entries[0];
    let mut out = String::new();
    if options.write_declaration {
        out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    }
    if let Value::Array(items) = root {
        if items.is_empty() {
            return Err(DwError::new("XML output requires a root element"));
        }
        for item in items {
            write_xml_element(&mut out, name, item, &options, namespaces, &[], 0)?;
        }
    } else {
        write_xml_element(&mut out, name, root, &options, namespaces, &[], 0)?;
    }
    Ok(out)
}
