use crate::error::ValidationError;

pub mod common;
pub mod numeric;
pub mod collection;
pub mod advanced;
pub mod conditional;

/// Trait that all validation rules must implement
pub trait Rule: Send + Sync {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError>;
}

/// Prelude module for commonly used rules
pub mod prelude {
    pub use super::common::*;
    pub use super::numeric::*;
    pub use super::collection::*;
    pub use super::advanced::*;
    pub use super::conditional::*;
}
