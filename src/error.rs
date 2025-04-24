use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Represents validation errors that can occur during validation
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// A single validation error with a message
    Single(String),
    
    /// Multiple validation errors grouped by field
    Multiple(HashMap<String, Vec<String>>),
}

impl ValidationError {
    /// Create a new single validation error
    pub fn new<S: Into<String>>(message: S) -> Self {
        ValidationError::Single(message.into())
    }
    
    /// Create a new validation error for a specific field
    pub fn field<S: Into<String>, M: Into<String>>(field: S, message: M) -> Self {
        let mut errors = HashMap::new();
        errors.insert(field.into(), vec![message.into()]);
        ValidationError::Multiple(errors)
    }
    
    /// Merge multiple validation errors
    pub fn merge(self, other: ValidationError) -> ValidationError {
        match (self, other) {
            (ValidationError::Single(msg1), ValidationError::Single(msg2)) => {
                let mut errors = HashMap::new();
                errors.insert("_".to_string(), vec![msg1, msg2]);
                ValidationError::Multiple(errors)
            },
            (ValidationError::Single(msg), ValidationError::Multiple(mut errs)) => {
                let entry = errs.entry("_".to_string()).or_insert_with(Vec::new);
                entry.push(msg);
                ValidationError::Multiple(errs)
            },
            (ValidationError::Multiple(mut errs), ValidationError::Single(msg)) => {
                let entry = errs.entry("_".to_string()).or_insert_with(Vec::new);
                entry.push(msg);
                ValidationError::Multiple(errs)
            },
            (ValidationError::Multiple(mut errs1), ValidationError::Multiple(errs2)) => {
                for (field, messages) in errs2 {
                    let entry = errs1.entry(field).or_insert_with(Vec::new);
                    entry.extend(messages);
                }
                ValidationError::Multiple(errs1)
            }
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::Single(msg) => write!(f, "{}", msg),
            ValidationError::Multiple(errors) => {
                writeln!(f, "Validation errors:")?;
                for (field, messages) in errors {
                    for msg in messages {
                        writeln!(f, "  {}: {}", field, msg)?;
                    }
                }
                Ok(())
            }
        }
    }
}
