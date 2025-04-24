# RustValidity - Rust Object Validator

RustValidity is a powerful, flexible, and easy-to-use validation library for Rust that provides struct-level validation. It offers a wide range of built-in validation rules and supports custom validation logic.

## Author

**Saeed Ghanbari** - [GitHub](https://github.com/sgh370)

## Features

- **Comprehensive Rule Set**: Includes common validations for strings, numbers, collections, and more
- **Extensible**: Easily create custom validation rules
- **Flexible Error Handling**: Support for single and multiple validation errors
- **Attribute-Based Validation**: Optional derive macro for struct field validation (with the `derive` feature)
- **Conditional Validation**: Rules that apply only under specific conditions
- **Cross-field Validation**: Validate fields based on the values of other fields
- **Collection Validation**: Validate arrays, vectors, maps, and other collections
- Multiple error handling
- Nested struct validation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustvalidity = "0.1.0"
```

## Quick Start

```rust
use rustvalidity::error::ValidationError;
use rustvalidity::validator::{Validator, Validate};
use rustvalidity::rules::{common, numeric, collection};

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
        
        // Validate fields
        if let Err(err) = validator.get_rule("required").unwrap().validate_any(&self.username) {
            return Err(err);
        }
        
        if let Err(err) = validator.get_rule("username_length").unwrap().validate_any(&self.username) {
            return Err(err);
        }
        
        if let Err(err) = validator.get_rule("email").unwrap().validate_any(&self.email) {
            return Err(err);
        }
        
        if let Err(err) = validator.get_rule("min_age").unwrap().validate_any(&self.age) {
            return Err(err);
        }
        
        if let Err(err) = validator.get_rule("interests_required").unwrap().validate_any(&self.interests) {
            return Err(err);
        }
        
        Ok(())
    }
}

fn main() {
    let user = User {
        username: "johndoe".to_string(),
        email: "john@example.com".to_string(),
        age: 25,
        interests: vec!["coding".to_string(), "reading".to_string()],
    };
    
    match user.validate() {
        Ok(_) => println!("User is valid!"),
        Err(err) => println!("Validation failed: {}", err),
    }
}
```

## Available Validation Rules

### Common Rules

- `Required`: Validates that a value is not empty
- `Length`: Validates string length (min, max)
- `Email`: Validates email format
- `UrlRule`: Validates URL format with optional scheme restrictions
- `UuidRule`: Validates UUID format
- `Json`: Validates JSON format
- `Date`: Validates date format and range
- `Phone`: Validates phone number format
- `OneOf`: Validates that a value is one of a set of allowed values
- `Custom`: Create custom validation rules with closures

### Numeric Rules

- `Min`: Validates minimum value
- `Max`: Validates maximum value
- `Range`: Validates value within a range
- `Positive`: Validates positive numbers
- `Negative`: Validates negative numbers
- `DivisibleBy`: Validates divisibility

### Collection Rules

- `Unique`: Validates collection elements are unique
- `Contains`: Validates collection contains a specific value
- `Each`: Applies a validation rule to each element
- `Map`: Validates map keys and values
- `MinSize`: Validates minimum collection size
- `MaxSize`: Validates maximum collection size
- `ExactSize`: Validates exact collection size

### Conditional Rules

- `If`: Validates a value only if a condition is true
- `Unless`: Validates a value only if a condition is false
- `RequiredIf`: Validates that a value is required if a condition is true
- `RequiredWith`: Validates that a value is required if another field has a specific value
- `RequiredWithout`: Validates that a value is required if another field does not have a specific value
- `RequiredIfAny`: Validates that a value is required if any of the specified conditions are true
- `RequiredIfAll`: Validates that a value is required if all of the specified conditions are true

### Advanced Rules

- `Password`: Validates password complexity
- `CreditCard`: Validates credit card numbers
- `SemVer`: Validates semantic version strings
- `Domain`: Validates domain names
- `Port`: Validates port numbers
- `IP`: Validates IP addresses
- `RegexRule`: Validates against a regular expression

## Custom Validation Rules

You can create custom validation rules by implementing the `Rule` trait:

```rust
use rustvalidity::error::ValidationError;
use rustvalidity::rules::Rule;

struct MyCustomRule;

impl Rule for MyCustomRule {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        // Your custom validation logic here
        Ok(())
    }
}
```

## Error Handling

Rustvalidity provides two types of validation errors:

1. `ValidationError::Single` - A single validation error with a message
2. `ValidationError::Multiple` - Multiple validation errors grouped by field

You can collect all validation errors using the `validate_all` method:

```rust
let errors = validator.validate_all(&value);
```

## Examples

Check out the examples directory for more usage examples:

- `user_validation.rs` - Basic validation example
- `attribute_validation.rs` - Advanced validation with struct attributes

## License

This project is licensed under the MIT License - see the LICENSE file for details.
