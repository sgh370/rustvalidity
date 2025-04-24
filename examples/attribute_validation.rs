use std::any::Any;
use std::collections::HashMap;

use rustvalidity::prelude::*;
use rustvalidity::error::ValidationError;
use rustvalidity::validator::{Validator, Validate};
use rustvalidity::rules::{Rule, common, numeric, collection, advanced};

#[derive(Debug)]
struct Product {
    // #[validate(required, length(3, 50))]
    name: String,
    
    // #[validate(min(0))]
    price: f64,
    
    // #[validate(required, email)]
    contact_email: String,
    
    // #[validate(min_size(1), each(required))]
    categories: Vec<String>,
    
    // #[validate(url)]
    website: String,
}

impl Validate for Product {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut validator = Validator::new();
        
        // Register validation rules
        validator.add_rule("required", common::Required);
        validator.add_rule("name_length", common::Length { min: 3, max: Some(50) });
        validator.add_rule("min_price", numeric::Min { value: 0.0 });
        validator.add_rule("email", common::Email { check_dns: false });
        validator.add_rule("categories_required", collection::MinSize { min: 1 });
        validator.add_rule("url", common::Url { allowed_schemes: Some(vec!["http".to_string(), "https".to_string()]) });
        
        // Validate fields
        let mut errors = HashMap::new();
        
        // Validate name (required, length between 3 and 50)
        if let Err(err) = validator.get_rule("required").unwrap().validate(&self.name as &dyn Any) {
            errors.entry("name".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        } else if let Err(err) = validator.get_rule("name_length").unwrap().validate(&self.name as &dyn Any) {
            errors.entry("name".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        }
        
        // Validate price (min 0)
        if let Err(err) = validator.get_rule("min_price").unwrap().validate(&self.price as &dyn Any) {
            errors.entry("price".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        }
        
        // Validate contact_email (required, email format)
        if let Err(err) = validator.get_rule("required").unwrap().validate(&self.contact_email as &dyn Any) {
            errors.entry("contact_email".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        } else if let Err(err) = validator.get_rule("email").unwrap().validate(&self.contact_email as &dyn Any) {
            errors.entry("contact_email".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        }
        
        // Validate categories (min_size 1)
        if let Err(err) = validator.get_rule("categories_required").unwrap().validate(&self.categories as &dyn Any) {
            errors.entry("categories".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
        }
        
        // Validate each category (required)
        for (i, category) in self.categories.iter().enumerate() {
            if let Err(err) = validator.get_rule("required").unwrap().validate(category as &dyn Any) {
                errors.entry(format!("categories[{}]", i)).or_insert_with(Vec::new).push(format!("{}", err));
            }
        }
        
        // Validate website (url format)
        if !self.website.is_empty() {
            if let Err(err) = validator.get_rule("url").unwrap().validate(&self.website as &dyn Any) {
                errors.entry("website".to_string()).or_insert_with(Vec::new).push(format!("{}", err));
            }
        }
        
        // Return errors if any
        if !errors.is_empty() {
            return Err(ValidationError::Multiple(errors));
        }
        
        Ok(())
    }
}

fn main() {
    // Create a valid product
    let valid_product = Product {
        name: "Awesome Product".to_string(),
        price: 29.99,
        contact_email: "sales@example.com".to_string(),
        categories: vec!["Electronics".to_string(), "Gadgets".to_string()],
        website: "https://example.com/products".to_string(),
    };
    
    // Validate the product
    match valid_product.validate() {
        Ok(_) => println!("Valid product: {}", valid_product.name),
        Err(err) => println!("Validation failed: {}", err),
    }
    
    // Create an invalid product
    let invalid_product = Product {
        name: "Ab".to_string(),
        price: -10.0,
        contact_email: "not-an-email".to_string(),
        categories: vec![],
        website: "not-a-url".to_string(),
    };
    
    // Validate the invalid product
    match invalid_product.validate() {
        Ok(_) => println!("Valid product: {}", invalid_product.name),
        Err(err) => println!("Validation failed: {}", err),
    }
}
