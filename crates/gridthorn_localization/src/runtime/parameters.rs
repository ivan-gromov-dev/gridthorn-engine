use std::collections::BTreeMap;

use super::LocalizationError;
use crate::{catalog::validation::numeric, identity::valid_identifier};

/// Owned text and bounded finite numeric message arguments. No backend types escape.
#[derive(Clone, Debug, Default)]
pub struct MessageParameters(pub(super) BTreeMap<String, ParameterValue>);

#[derive(Clone, Debug)]
pub(super) enum ParameterValue {
    Text(String),
    Number(f64),
}

impl MessageParameters {
    /// Empty parameters for messages without variables.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set or replace a text parameter; text is data, never parsed as Fluent.
    ///
    /// # Errors
    /// Rejects invalid Fluent variable names and text over 64 KiB.
    pub fn set_text(
        &mut self,
        name: &str,
        value: impl Into<String>,
    ) -> Result<(), LocalizationError> {
        validate_name(name)?;
        let value = value.into();
        if value.len() > 64 * 1024 {
            return Err(parameter_error(name, "text exceeds 64 KiB"));
        }
        self.0.insert(name.to_owned(), ParameterValue::Text(value));
        Ok(())
    }

    /// Set or replace a finite number, magnitude <= 10^12, at most six decimals.
    /// Numbers use binary64; this presentation API is not an exact money codec.
    ///
    /// # Errors
    /// Rejects invalid variable names, non-finite and out-of-contract numbers.
    pub fn set_number(&mut self, name: &str, value: f64) -> Result<(), LocalizationError> {
        validate_name(name)?;
        numeric(&value.to_string()).map_err(|reason| parameter_error(name, &reason))?;
        self.0
            .insert(name.to_owned(), ParameterValue::Number(value));
        Ok(())
    }
}

fn validate_name(name: &str) -> Result<(), LocalizationError> {
    if !valid_identifier(name) {
        return Err(parameter_error(name, "invalid Fluent variable name"));
    }
    Ok(())
}

pub(super) fn parameter_error(name: &str, reason: &str) -> LocalizationError {
    LocalizationError::Parameter {
        name: name.to_owned(),
        reason: reason.to_owned(),
    }
}
