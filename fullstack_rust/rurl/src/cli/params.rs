use tracing::debug;

use crate::errors::{RurlError, RurlResult};

#[derive(Debug, Clone)]
pub enum Parameter {
    Header { key: String, value: String },
    Query { key: String, value: String },
    Data { key: String, value: String },
    RawJsonData { key: String, value: String },
    DataFile { key: String, filename: String },
    FormFile { key: String, filename: String },
}

impl Parameter {
    pub fn is_data(&self) -> bool {
        !matches!(self, Parameter::Header { .. } | Parameter::Query { .. })
    }

    pub fn is_form_file(&self) -> bool {
        matches!(self, Parameter::FormFile { .. })
    }
}

/// parse an arg to Parameter
/// order matters: ':=' and '==' before '='
pub fn parse_param(src: &str) -> RurlResult<Parameter> {
    debug!("parsing parameter: {}", src);

    if let Some((k, v)) = src.split_once("==") {
        return Ok(Parameter::Query {
            key: k.into(),
            value: v.into(),
        });
    }

    if let Some((k, v)) = src.split_once(":=") {
        return Ok(Parameter::RawJsonData {
            key: k.into(),
            value: v.into(),
        });
    }

    if let Some((k, v)) = src.split_once("=") {
        return Ok(Parameter::Data {
            key: k.into(),
            value: v.into(),
        });
    }

    if let Some((k, v)) = src.split_once(":") {
        return Ok(Parameter::Header {
            key: k.into(),
            value: v.into(),
        });
    }

    Err(RurlError::ParameterMissingSeparator(src.into()))
}
