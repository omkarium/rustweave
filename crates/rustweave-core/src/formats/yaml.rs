// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::error::DwError;
use crate::value::Value;

pub(crate) fn read_yaml(_text: &str, _properties: Option<&Value>) -> Result<Value, DwError> {
    Err(DwError::new("YAML support is not enabled"))
}

pub(crate) fn write_yaml(_value: &Value, _properties: Option<&Value>) -> Result<String, DwError> {
    Err(DwError::new("YAML support is not enabled"))
}
