use std::fmt::Debug;

use crate::error::ValidationError;
use crate::rules::Rule;

/// Validates that a numeric value is within a specified range
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl<T: PartialOrd + Debug + Clone + Send + Sync + 'static> Rule for Range<T> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(val) = value.downcast_ref::<T>() {
            if *val < self.min {
                return Err(ValidationError::new(format!(
                    "Value must be greater than or equal to {:?}", self.min
                )));
            }
            if *val > self.max {
                return Err(ValidationError::new(format!(
                    "Value must be less than or equal to {:?}", self.max
                )));
            }
            Ok(())
        } else {
            Err(ValidationError::new(format!(
                "Value is not of the expected numeric type"
            )))
        }
    }
}

/// Validates that a numeric value is positive
pub struct Positive;

impl Rule for Positive {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(val) = value.downcast_ref::<i8>() {
            if *val <= 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<i16>() {
            if *val <= 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<i32>() {
            if *val <= 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<i64>() {
            if *val <= 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<f32>() {
            if *val <= 0.0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<f64>() {
            if *val <= 0.0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<u8>() {
            if *val == 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<u16>() {
            if *val == 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<u32>() {
            if *val == 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<u64>() {
            if *val == 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else if let Some(val) = value.downcast_ref::<usize>() {
            if *val == 0 {
                return Err(ValidationError::new("Value must be positive"));
            }
        } else {
            return Err(ValidationError::new("Value is not a numeric type"));
        }
        
        Ok(())
    }
}

/// Validates that a numeric value is greater than or equal to a minimum value
pub struct Min<T> {
    pub value: T,
}

impl<T: PartialOrd + Debug + Clone + Send + Sync + 'static> Rule for Min<T> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(val) = value.downcast_ref::<T>() {
            if *val < self.value {
                return Err(ValidationError::new(format!(
                    "Value must be greater than or equal to {:?}", self.value
                )));
            }
            Ok(())
        } else {
            Err(ValidationError::new("Value is not of the expected numeric type"))
        }
    }
}

/// Validates that a numeric value is less than or equal to a maximum value
pub struct Max<T> {
    pub value: T,
}

impl<T: PartialOrd + Debug + Clone + Send + Sync + 'static> Rule for Max<T> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(val) = value.downcast_ref::<T>() {
            if *val > self.value {
                return Err(ValidationError::new(format!(
                    "Value must be less than or equal to {:?}", self.value
                )));
            }
            Ok(())
        } else {
            Err(ValidationError::new(format!(
                "Value is not of the expected numeric type"
            )))
        }
    }
}

/// Validates that a numeric value is negative
pub struct Negative;

impl Rule for Negative {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(val) = value.downcast_ref::<i8>() {
            if *val >= 0 {
                return Err(ValidationError::new("Value must be negative"));
            }
        } else if let Some(val) = value.downcast_ref::<i16>() {
            if *val >= 0 {
                return Err(ValidationError::new("Value must be negative"));
            }
        } else if let Some(val) = value.downcast_ref::<i32>() {
            if *val >= 0 {
                return Err(ValidationError::new("Value must be negative"));
            }
        } else if let Some(val) = value.downcast_ref::<i64>() {
            if *val >= 0 {
                return Err(ValidationError::new("Value must be negative"));
            }
        } else if let Some(val) = value.downcast_ref::<f32>() {
            if *val >= 0.0 {
                return Err(ValidationError::new("Value must be negative"));
            }
        } else if let Some(val) = value.downcast_ref::<f64>() {
            if *val >= 0.0 {
                return Err(ValidationError::new("Value must be negative"));
            }
        } else {
            return Err(ValidationError::new("Value is not a signed numeric type"));
        }
        
        Ok(())
    }
}

/// Validates that a numeric value is divisible by another value
pub struct DivisibleBy<T> {
    pub divisor: T,
}

impl Rule for DivisibleBy<i32> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(val) = value.downcast_ref::<i32>() {
            if self.divisor == 0 {
                return Err(ValidationError::new("Divisor cannot be zero"));
            }
            if *val % self.divisor != 0 {
                return Err(ValidationError::new(format!(
                    "Value must be divisible by {}", self.divisor
                )));
            }
            Ok(())
        } else {
            Err(ValidationError::new("Value is not of the expected numeric type"))
        }
    }
}
