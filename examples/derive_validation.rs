use rustvalidity::prelude::*;

// Define a struct with validation attributes
#[derive(Debug, Validate)]
struct Product {
    #[validate(required, length(min = 3, max = 50))]
    name: String,
    
    #[validate(min = 0)]
    price: f64,
    
    #[validate(required, email)]
    contact_email: String,
    
    #[validate(min = 1)]
    quantity: i32,
    
    #[validate(url)]
    website: String,
}

fn main() {
    // Create a valid product
    let valid_product = Product {
        name: "Awesome Product".to_string(),
        price: 29.99,
        contact_email: "sales@example.com".to_string(),
        quantity: 10,
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
        quantity: 0,
        website: "not-a-url".to_string(),
    };
    
    // Validate the invalid product
    match invalid_product.validate() {
        Ok(_) => println!("Valid product: {}", invalid_product.name),
        Err(err) => println!("Validation failed: {}", err),
    }
}
