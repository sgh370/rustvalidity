//! # Rustvalidity
//!
//! Rustvalidity is a powerful, flexible, and easy-to-use validation library for Rust
//! that provides struct-level validation through attributes. It offers a wide range
//! of built-in validation rules and supports custom validation logic.
//!
//! # Author
//! 
//! Rustvalidity was created by [Saeed Ghanbari](https://github.com/sgh370).

pub mod rules;
pub mod validator;
pub mod error;

pub use validator::Validator;
pub use error::ValidationError;

// Re-export the derive macro when the derive feature is enabled
#[cfg(feature = "derive")]
pub use rustvalidity_derive::Validate;

/// Re-export commonly used items for easier imports
pub mod prelude {
    pub use crate::validator::{Validator, Validate};
    pub use crate::rules::Rule;
    pub use crate::error::ValidationError;
    pub use crate::rules::prelude::*;
    
    // Re-export the derive macro when the derive feature is enabled
    #[cfg(feature = "derive")]
    pub use rustvalidity_derive::Validate;
}
