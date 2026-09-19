use reqwest::RequestBuilder;
use serde_json::{Map, Value};

use crate::{cli::params::Parameter, errors::RurlResult};

pub fn apply_parameters(
    mut builder: RequestBuilder,
    form_mode: bool,
    parameters: &[Parameter],
) -> RurlResult<RequestBuilder> {
    let mut json = Map::new();

    for param in parameters {
        match param {
            Parameter::Header { key, value } => {
                builder = builder.header(key, value);
            }
            Parameter::Query { key, value } => {
                builder = builder.query(&[(key, value)]);
            }
            Parameter::Data { key, value } => {
                json.insert(key.clone(), Value::String(value.clone()));
            }
            Parameter::RawJsonData { key, value } => {
                let v: Value = serde_json::from_str(value)?;
                json.insert(key.clone(), v);
            }
            _ => {}
        }
    }

    if !json.is_empty() {
        if form_mode {
            builder = builder.form(&json);
        } else {
            builder = builder.json(&json);
        }
    }

    Ok(builder)
}
