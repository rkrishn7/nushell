use crate::object::{Dictionary, Primitive, Value};
use crate::prelude::*;
use indexmap::IndexMap;
use std::collections::HashMap;

fn convert_ini_second_to_nu_value(v: &HashMap<String, String>) -> Value {
    let mut second = Dictionary::new(IndexMap::new());
    for (key, value) in v.into_iter() {
        second.add(
            key.clone(),
            Value::Primitive(Primitive::String(value.clone())),
        );
    }
    Value::Object(second)
}
fn convert_ini_top_to_nu_value(v: &HashMap<String, HashMap<String, String>>) -> Value {
    let mut top_level = Dictionary::new(IndexMap::new());
    for (key, value) in v.iter() {
        top_level.add(key.clone(), convert_ini_second_to_nu_value(value));
    }
    Value::Object(top_level)
}

pub fn from_ini_string_to_value(s: String) -> Result<Value, Box<dyn std::error::Error>> {
    let v: HashMap<String, HashMap<String, String>> = serde_ini::from_str(&s)?;
    Ok(convert_ini_top_to_nu_value(&v))
}

pub fn from_ini(args: CommandArgs) -> Result<OutputStream, ShellError> {
    let out = args.input;
    let span = args.name_span;
    Ok(out
        .values
        .map(move |a| match a.item {
            Value::Primitive(Primitive::String(s)) => match from_ini_string_to_value(s) {
                Ok(x) => ReturnSuccess::value(x.spanned(a.span)),
                Err(e) => Err(ShellError::maybe_labeled_error(
                    "Could not parse as INI",
                    format!("{:#?}", e),
                    span,
                )),
            },
            _ => Err(ShellError::maybe_labeled_error(
                "Expected string values from pipeline",
                "expects strings from pipeline",
                span,
            )),
        })
        .to_output_stream())
}
