use std::collections::HashMap;

use rustvalidity::prelude::*;
use rustvalidity::error::ValidationError;
use rustvalidity::validator::{Validator, Validate};
use rustvalidity::rules::{Rule, common, numeric, collection};

struct User {
    username: String,
    email: String,
    age: i32,
    interests: Vec<String>,
}

impl Validate for User {
    fn validate(&self) -> Result<(), ValidationError> {
        // Create a new validator instance
        let mut validator = Validator::new();
        
        // Add validation rules
        validator.add_rule("required", common::Required);
        validator.add_rule("username_length", common::Length { min: 3, max: Some(20) });
        validator.add_rule("email", common::Email { check_dns: false });
        validator.add_rule("min_age", numeric::Min { value: 18 });
        validator.add_rule("interests_required", collection::MinSize { min: 1 });
        
        // Validate individual fields
        let mut errors = HashMap::new();
        
        // Validate username
        if let Err(err) = validator.get_rule("required")
            .unwrap()
            .validate_any(&self.username) {
            errors.entry("username".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        } else if let Err(err) = validator.get_rule("username_length")
            .unwrap()
            .validate_any(&self.username) {
            errors.entry("username".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        }
        
        // Validate email
        if let Err(err) = validator.get_rule("required")
            .unwrap()
            .validate_any(&self.email) {
            errors.entry("email".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        } else if let Err(err) = validator.get_rule("email")
            .unwrap()
            .validate_any(&self.email) {
            errors.entry("email".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        }
        
        // Validate age
        if let Err(err) = validator.get_rule("min_age")
            .unwrap()
            .validate_any(&self.age) {
            errors.entry("age".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        }
        
        // Validate interests
        if let Err(err) = validator.get_rule("interests_required")
            .unwrap()
            .validate_any(&self.interests) {
            errors.entry("interests".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        }
        
        // Check if there are any validation errors
        if !errors.is_empty() {
            return Err(ValidationError::Multiple(errors));
        }
        
        Ok(())
    }
}

fn main() {
    // Create a valid user
    let valid_user = User {
        username: "johndoe".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
        interests: vec!["coding".to_string(), "reading".to_string()],
    };
    
    // Validate the user
    match valid_user.validate() {
        Ok(_) => println!("Valid user: {}", valid_user.username),
        Err(err) => println!("Validation failed: {}", err),
    }
    
    // Create an invalid user
    let invalid_user = User {
        username: "jo".to_string(),
        email: "invalid-email".to_string(),
        age: 16,
        interests: vec![],
    };
    
    // Validate the invalid user
    match invalid_user.validate() {
        Ok(_) => println!("Valid user: {}", invalid_user.username),
        Err(err) => println!("Validation failed: {}", err),
    }
}
