use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::error::ValidationError;
use crate::rules::Rule;

/// Validates that all elements in a collection are unique
pub struct Unique;

impl Rule for Unique {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        // For Vec<T> where T: Eq + Hash
        if let Some(vec) = value.downcast_ref::<Vec<String>>() {
            let mut set = HashSet::new();
            for item in vec {
                if !set.insert(item) {
                    return Err(ValidationError::new(format!(
                        "Duplicate value found: {}", item
                    )));
                }
            }
        } else if let Some(vec) = value.downcast_ref::<Vec<i32>>() {
            let mut set = HashSet::new();
            for item in vec {
                if !set.insert(*item) {
                    return Err(ValidationError::new(format!(
                        "Duplicate value found: {}", item
                    )));
                }
            }
        } else if let Some(vec) = value.downcast_ref::<Vec<i64>>() {
            let mut set = HashSet::new();
            for item in vec {
                if !set.insert(*item) {
                    return Err(ValidationError::new(format!(
                        "Duplicate value found: {}", item
                    )));
                }
            }
        } else {
            return Err(ValidationError::new(
                "Value must be a collection of hashable items"
            ));
        }
        
        Ok(())
    }
}

/// Validates that a collection contains a specific value
pub struct Contains<T: PartialEq + Clone + 'static> {
    pub value: T,
}

impl<T: PartialEq + Clone + Send + Sync + std::fmt::Debug + 'static> Rule for Contains<T> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(vec) = value.downcast_ref::<Vec<T>>() {
            if !vec.contains(&self.value) {
                return Err(ValidationError::new(format!(
                    "Collection must contain {:?}", self.value
                )));
            }
        } else {
            return Err(ValidationError::new(
                "Value must be a collection of the expected type"
            ));
        }
        
        Ok(())
    }
}

/// Applies a validation rule to each element in a collection
pub struct Each<R: Rule + 'static> {
    pub rule: Box<R>,
}

impl<R: Rule + 'static> Rule for Each<R> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(vec) = value.downcast_ref::<Vec<String>>() {
            for (i, item) in vec.iter().enumerate() {
                if let Err(err) = self.rule.validate_any(item) {
                    return Err(ValidationError::new(format!(
                        "Item at index {} failed validation: {}", i, err
                    )));
                }
            }
        } else if let Some(vec) = value.downcast_ref::<Vec<i32>>() {
            for (i, item) in vec.iter().enumerate() {
                if let Err(err) = self.rule.validate_any(item) {
                    return Err(ValidationError::new(format!(
                        "Item at index {} failed validation: {}", i, err
                    )));
                }
            }
        } else if let Some(map) = value.downcast_ref::<HashMap<String, String>>() {
            for (key, val) in map {
                if let Err(err) = self.rule.validate_any(val) {
                    return Err(ValidationError::new(format!(
                        "Value for key '{}' failed validation: {}", key, err
                    )));
                }
            }
        } else {
            return Err(ValidationError::new("Value must be a collection or map"));
        }
        
        Ok(())
    }
}

/// Validates a map's keys and values
pub struct Map {
    pub key_rule: Option<Box<dyn Rule>>,
    pub value_rule: Option<Box<dyn Rule>>,
}

impl Rule for Map {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(map) = value.downcast_ref::<HashMap<String, String>>() {
            for (key, val) in map {
                if let Some(key_rule) = &self.key_rule {
                    if let Err(err) = key_rule.validate_any(key) {
                        return Err(ValidationError::new(format!(
                            "Map key '{}' failed validation: {}", key, err
                        )));
                    }
                }
                
                if let Some(value_rule) = &self.value_rule {
                    if let Err(err) = value_rule.validate_any(val) {
                        return Err(ValidationError::new(format!(
                            "Map value for key '{}' failed validation: {}", key, err
                        )));
                    }
                }
            }
        } else if let Some(map) = value.downcast_ref::<HashMap<String, i32>>() {
            for (key, val) in map {
                if let Some(key_rule) = &self.key_rule {
                    if let Err(err) = key_rule.validate_any(key) {
                        return Err(ValidationError::new(format!(
                            "Map key '{}' failed validation: {}", key, err
                        )));
                    }
                }
                
                if let Some(value_rule) = &self.value_rule {
                    if let Err(err) = value_rule.validate_any(val) {
                        return Err(ValidationError::new(format!(
                            "Map value for key '{}' failed validation: {}", key, err
                        )));
                    }
                }
            }
        } else {
            return Err(ValidationError::new(
                "Value must be a map"
            ));
        }
        
        Ok(())
    }
}

/// Validates that a collection has a minimum size
pub struct MinSize {
    pub min: usize,
}

impl Rule for MinSize {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(vec) = value.downcast_ref::<Vec<String>>() {
            if vec.len() < self.min {
                return Err(ValidationError::new(format!(
                    "Collection must have at least {} items", self.min
                )));
            }
        } else if let Some(vec) = value.downcast_ref::<Vec<i32>>() {
            if vec.len() < self.min {
                return Err(ValidationError::new(format!(
                    "Collection must have at least {} items", self.min
                )));
            }
        } else if let Some(map) = value.downcast_ref::<HashMap<String, String>>() {
            if map.len() < self.min {
                return Err(ValidationError::new(format!(
                    "Map must have at least {} entries", self.min
                )));
            }
        } else if let Some(s) = value.downcast_ref::<String>() {
            if s.len() < self.min {
                return Err(ValidationError::new(format!(
                    "String must have at least {} characters", self.min
                )));
            }
        } else {
            return Err(ValidationError::new(
                "Value must be a collection, map, or string"
            ));
        }
        
        Ok(())
    }
}

/// Validates that a collection has a maximum size
pub struct MaxSize {
    pub max: usize,
}

impl Rule for MaxSize {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(vec) = value.downcast_ref::<Vec<String>>() {
            if vec.len() > self.max {
                return Err(ValidationError::new(format!(
                    "Collection must have at most {} items", self.max
                )));
            }
        } else if let Some(vec) = value.downcast_ref::<Vec<i32>>() {
            if vec.len() > self.max {
                return Err(ValidationError::new(format!(
                    "Collection must have at most {} items", self.max
                )));
            }
        } else if let Some(map) = value.downcast_ref::<HashMap<String, String>>() {
            if map.len() > self.max {
                return Err(ValidationError::new(format!(
                    "Map must have at most {} entries", self.max
                )));
            }
        } else if let Some(s) = value.downcast_ref::<String>() {
            if s.len() > self.max {
                return Err(ValidationError::new(format!(
                    "String must have at most {} characters", self.max
                )));
            }
        } else {
            return Err(ValidationError::new(
                "Value must be a collection, map, or string"
            ));
        }
        
        Ok(())
    }
}

/// Validates that a collection has an exact size
pub struct ExactSize {
    pub size: usize,
}

impl Rule for ExactSize {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(vec) = value.downcast_ref::<Vec<String>>() {
            if vec.len() != self.size {
                return Err(ValidationError::new(format!(
                    "Collection must have exactly {} items", self.size
                )));
            }
        } else if let Some(vec) = value.downcast_ref::<Vec<i32>>() {
            if vec.len() != self.size {
                return Err(ValidationError::new(format!(
                    "Collection must have exactly {} items", self.size
                )));
            }
        } else if let Some(map) = value.downcast_ref::<HashMap<String, String>>() {
            if map.len() != self.size {
                return Err(ValidationError::new(format!(
                    "Map must have exactly {} entries", self.size
                )));
            }
        } else if let Some(s) = value.downcast_ref::<String>() {
            if s.len() != self.size {
                return Err(ValidationError::new(format!(
                    "String must have exactly {} characters", self.size
                )));
            }
        } else {
            return Err(ValidationError::new(
                "Value must be a collection, map, or string"
            ));
        }
        
        Ok(())
    }
}
