use std::str::FromStr;
use regex::Regex;
use chrono::{DateTime, NaiveDate};
use url::Url;
use uuid::Uuid;
use serde_json::Value;
use std::fmt::Debug;

use crate::error::ValidationError;
use crate::rules::Rule;

/// Validates that a value is not empty (strings, collections, options)
pub struct Required;

impl Rule for Required {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        // Handle String type
        if let Some(s) = value.downcast_ref::<String>() {
            if s.is_empty() {
                return Err(ValidationError::new("Value is required"));
            }
        } 
        // Handle &str type
        else if let Some(s) = (value as &dyn std::any::Any).downcast_ref::<&str>() {
            if s.is_empty() {
                return Err(ValidationError::new("Value is required"));
            }
        } 
        // Handle Option types
        else if let Some(o) = (value as &dyn std::any::Any).downcast_ref::<Option<String>>() {
            if o.is_none() {
                return Err(ValidationError::new("Value is required"));
            }
        }
        // Handle Vec types
        else if let Some(v) = (value as &dyn std::any::Any).downcast_ref::<Vec<String>>() {
            if v.is_empty() {
                return Err(ValidationError::new("Value is required"));
            }
        }
        
        Ok(())
    }
}

/// Validates string length
pub struct Length {
    pub min: usize,
    pub max: Option<usize>,
}

impl Rule for Length {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        // Handle String type
        if let Some(s) = value.downcast_ref::<String>() {
            let len = s.len();
            if len < self.min {
                return Err(ValidationError::new(format!("Length must be at least {}", self.min)));
            }
            if let Some(max) = self.max {
                if len > max {
                    return Err(ValidationError::new(format!("Length must not exceed {}", max)));
                }
            }
        } 
        // Handle &str type
        else if let Some(s) = (value as &dyn std::any::Any).downcast_ref::<&str>() {
            let len = s.len();
            if len < self.min {
                return Err(ValidationError::new(format!("Length must be at least {}", self.min)));
            }
            if let Some(max) = self.max {
                if len > max {
                    return Err(ValidationError::new(format!("Length must not exceed {}", max)));
                }
            }
        } 
        // Handle Vec types
        else if let Some(v) = (value as &dyn std::any::Any).downcast_ref::<Vec<String>>() {
            let len = v.len();
            if len < self.min {
                return Err(ValidationError::new(format!("Collection must have at least {} items", self.min)));
            }
            if let Some(max) = self.max {
                if len > max {
                    return Err(ValidationError::new(format!("Collection must not exceed {} items", max)));
                }
            }
        } else {
            return Err(ValidationError::new("Value must be a string or collection"));
        }
        
        Ok(())
    }
}

/// Validates that a value is one of the specified options
pub struct OneOf<T: PartialEq + Clone + 'static> {
    pub values: Vec<T>,
}

impl<T: PartialEq + Clone + Send + Sync + 'static> Rule for OneOf<T> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(val) = value.downcast_ref::<T>() {
            if !self.values.contains(val) {
                return Err(ValidationError::new(format!("Value must be one of the allowed options")));
            }
        } else {
            return Err(ValidationError::new("Value is not of the expected type"));
        }
        
        Ok(())
    }
}

/// Validates email format
pub struct Email {
    pub check_dns: bool,
}

impl Rule for Email {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_email(s, self.check_dns)
        } else if let Some(s) = (value as &dyn std::any::Any).downcast_ref::<&str>() {
            validate_email(s, self.check_dns)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_email(email: &str, _check_dns: bool) -> Result<(), ValidationError> {
    // Basic email validation using regex
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    
    if !email_regex.is_match(email) {
        return Err(ValidationError::new("Invalid email format"));
    }
    
    // DNS validation would be implemented here if check_dns is true
    // For simplicity, we're skipping actual DNS validation
    
    Ok(())
}

/// Validates URL format
pub struct UrlRule {
    pub allowed_schemes: Option<Vec<String>>,
}

impl Rule for UrlRule {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_url(s, &self.allowed_schemes)
        } else if let Some(s) = (value as &dyn std::any::Any).downcast_ref::<&str>() {
            validate_url(s, &self.allowed_schemes)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_url(url_str: &str, allowed_schemes: &Option<Vec<String>>) -> Result<(), ValidationError> {
    match url::Url::parse(url_str) {
        Ok(url) => {
            if let Some(schemes) = allowed_schemes {
                if !schemes.contains(&url.scheme().to_string()) {
                    return Err(ValidationError::new(format!(
                        "URL scheme must be one of: {:?}", schemes
                    )));
                }
            }
            Ok(())
        },
        Err(_) => Err(ValidationError::new("Invalid URL format")),
    }
}

/// Validates JSON format
pub struct Json;

impl Rule for Json {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_json(s)
        } else if let Some(s) = (value as &dyn std::any::Any).downcast_ref::<&str>() {
            validate_json(s)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_json(json_str: &str) -> Result<(), ValidationError> {
    match serde_json::from_str::<Value>(json_str) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Invalid JSON format")),
    }
}

/// Validates UUID format
pub struct UuidRule;

impl Rule for UuidRule {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_uuid(s)
        } else if let Some(s) = (value as &dyn std::any::Any).downcast_ref::<&str>() {
            validate_uuid(s)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_uuid(uuid_str: &str) -> Result<(), ValidationError> {
    match uuid::Uuid::from_str(uuid_str) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Invalid UUID format")),
    }
}

/// Validates date format
pub struct Date {
    pub format: String,
    pub min: Option<NaiveDate>,
    pub max: Option<NaiveDate>,
}

impl Rule for Date {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_date(s, &self.format, &self.min, &self.max)
        } else if let Some(s) = (value as &dyn std::any::Any).downcast_ref::<&str>() {
            validate_date(s, &self.format, &self.min, &self.max)
        } else if let Some(date) = (value as &dyn std::any::Any).downcast_ref::<NaiveDate>() {
            validate_naive_date(date, &self.min, &self.max)
        } else {
            Err(ValidationError::new("Value must be a string or date"))
        }
    }
}

fn validate_date(
    date_str: &str, 
    format: &str, 
    min: &Option<NaiveDate>, 
    max: &Option<NaiveDate>
) -> Result<(), ValidationError> {
    match NaiveDate::parse_from_str(date_str, format) {
        Ok(date) => validate_naive_date(&date, min, max),
        Err(_) => Err(ValidationError::new(format!("Invalid date format, expected {}", format))),
    }
}

fn validate_naive_date(
    date: &NaiveDate, 
    min: &Option<NaiveDate>, 
    max: &Option<NaiveDate>
) -> Result<(), ValidationError> {
    if let Some(min_date) = min {
        if date < min_date {
            return Err(ValidationError::new(format!("Date must not be before {}", min_date)));
        }
    }
    
    if let Some(max_date) = max {
        if date > max_date {
            return Err(ValidationError::new(format!("Date must not be after {}", max_date)));
        }
    }
    
    Ok(())
}

/// Custom validation rule using a closure
pub struct Custom<F>
where
    F: for<'a> Fn(&'a dyn std::any::Any) -> Result<(), ValidationError> + Send + Sync,
{
    pub validator: F,
}

impl<F> Rule for Custom<F>
where
    F: for<'a> Fn(&'a dyn std::any::Any) -> Result<(), ValidationError> + Send + Sync,
{
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        (self.validator)(value)
    }
}

/// Phone number validation
pub struct Phone {
    pub allow_empty: bool,
}

impl Rule for Phone {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_phone(s, self.allow_empty)
        } else if let Some(s) = (value as &dyn std::any::Any).downcast_ref::<&str>() {
            validate_phone(s, self.allow_empty)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_phone(phone: &str, allow_empty: bool) -> Result<(), ValidationError> {
    if phone.is_empty() && allow_empty {
        return Ok(());
    }
    
    // Basic phone validation: +1234567890 or 1234567890
    let phone_regex = Regex::new(r"^\+?\d{10,15}$").unwrap();
    
    if !phone_regex.is_match(phone) {
        return Err(ValidationError::new("Invalid phone number format"));
    }
    
    Ok(())
}
