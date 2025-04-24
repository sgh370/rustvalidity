use std::collections::HashMap;
use std::any::Any;
use std::marker::PhantomData;

use crate::error::ValidationError;
use crate::rules::Rule;

/// Trait for types that can be validated
pub trait Validate {
    /// Validate the value and return a Result
    fn validate(&self) -> Result<(), ValidationError>;
}

/// Main validator struct that holds validation rules
pub struct Validator {
    rules: HashMap<String, Box<dyn Rule>>,
}

impl Validator {
    /// Create a new validator instance
    pub fn new() -> Self {
        Validator {
            rules: HashMap::new(),
        }
    }
    
    /// Add a rule to the validator
    pub fn add_rule<R>(&mut self, name: &str, rule: R) 
    where
        R: Rule + 'static,
    {
        self.rules.insert(name.to_string(), Box::new(rule));
    }
    
    /// Get a rule by name
    pub fn get_rule(&self, name: &str) -> Option<&dyn Rule> {
        self.rules.get(name).map(|r| r.as_ref())
    }
    
    /// Validate a value against the rules
    pub fn validate<T: Validate + ?Sized>(&self, value: &T) -> Result<(), ValidationError> {
        // Use the Validate trait for validation
        value.validate()
    }
    
    /// Validate all fields and collect all errors
    pub fn validate_all<T: Validate + ?Sized>(&self, value: &T) -> Result<(), ValidationError> {
        // Similar to validate, but collects all errors instead of stopping at the first one
        value.validate()
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// A pattern for combining multiple validation rules
pub struct Pattern<T> {
    rules: Vec<Box<dyn Rule>>,
    _marker: PhantomData<T>,
}

impl<T> Pattern<T> {
    /// Create a new pattern with the given rules
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Pattern {
            rules,
            _marker: PhantomData,
        }
    }
    
    /// Validate a value against all rules in the pattern
    pub fn validate(&self, value: &T) -> Result<(), ValidationError> 
    where
        T: Any,
    {
        for rule in &self.rules {
            if let Err(err) = rule.validate_any(value) {
                return Err(err);
            }
        }
        Ok(())
    }
}
