use std::error::Error;
use std::fmt;

use crate::config::DataType;

pub struct WrongValueError {
    field: String,
    expected_type: DataType,
}

impl fmt::Debug for WrongValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(
            &format!(
                "Wrong value for field: {}, expected a {:?}",
                self.field, self.expected_type
            ),
            f,
        )
    }
}

impl fmt::Display for WrongValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(
            &format!(
                "Wrong value for field: {}, expected a {:?}",
                self.field, self.expected_type
            ),
            f,
        )
    }
}

impl Error for WrongValueError {}

impl WrongValueError {
    pub fn new(field: &str, expected_type: &DataType) -> Self {
        Self {
            field: field.to_string(),
            expected_type: expected_type.clone(),
        }
    }
}

pub struct NotFoundError {
    field: String,
}

impl fmt::Debug for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&format!("Feature with name {} not found!", self.field), f)
    }
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&format!("Feature with name {} not found!", self.field), f)
    }
}

impl Error for NotFoundError {}

impl NotFoundError {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
        }
    }
}

pub struct NotPreparedError;

impl fmt::Debug for NotPreparedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt("features are not prepared", f)
    }
}

impl fmt::Display for NotPreparedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt("features are not prepared", f)
    }
}

impl Error for NotPreparedError {}

impl NotPreparedError {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub enum FeatureError {
    WrongValue(WrongValueError),
    NotFound(NotFoundError),
    NotPrepared(NotPreparedError),
}

impl From<WrongValueError> for FeatureError {
    fn from(err: WrongValueError) -> Self {
        FeatureError::WrongValue(err)
    }
}

impl From<NotFoundError> for FeatureError {
    fn from(err: NotFoundError) -> Self {
        FeatureError::NotFound(err)
    }
}

impl From<NotPreparedError> for FeatureError {
    fn from(err: NotPreparedError) -> Self {
        FeatureError::NotPrepared(err)
    }
}
