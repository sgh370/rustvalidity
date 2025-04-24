use crate::error::ValidationError;
use crate::rules::Rule;

/// Validates a value only if a condition is true
pub struct If {
    pub condition: Box<dyn for<'a> Fn(&'a dyn std::any::Any) -> bool + Send + Sync>,
    pub then: Box<dyn Rule>,
}

impl Rule for If {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if (self.condition)(value) {
            self.then.validate_any(value)
        } else {
            Ok(())
        }
    }
}

/// Validates a value only if a condition is false
pub struct Unless {
    pub condition: Box<dyn for<'a> Fn(&'a dyn std::any::Any) -> bool + Send + Sync>,
    pub then: Box<dyn Rule>,
}

impl Rule for Unless {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if !(self.condition)(value) {
            self.then.validate_any(value)
        } else {
            Ok(())
        }
    }
}

/// Validates that a value is required if a condition is true
pub struct RequiredIf {
    pub condition: Box<dyn Fn() -> bool + Send + Sync>,
}

impl Rule for RequiredIf {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if (self.condition)() {
            // Check if value is empty or null
            if let Some(s) = value.downcast_ref::<String>() {
                if s.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(s) = value.downcast_ref::<&str>() {
                if s.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(o) = value.downcast_ref::<Option<String>>() {
                if o.is_none() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(v) = value.downcast_ref::<Vec<String>>() {
                if v.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            }
        }
        
        Ok(())
    }
}

/// Validates that a value is required if another field has a specific value
pub struct RequiredWith<T: PartialEq + 'static> {
    pub other_field: Box<dyn Fn() -> Option<T> + Send + Sync>,
    pub expected_value: T,
}

impl<T: PartialEq + Send + Sync + 'static> Rule for RequiredWith<T> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(other_value) = (self.other_field)() {
            if other_value == self.expected_value {
                // Check if value is empty or null
                if let Some(s) = value.downcast_ref::<String>() {
                    if s.is_empty() {
                        return Err(ValidationError::new("Value is required"));
                    }
                } else if let Some(s) = value.downcast_ref::<&str>() {
                    if s.is_empty() {
                        return Err(ValidationError::new("Value is required"));
                    }
                } else if let Some(o) = value.downcast_ref::<Option<String>>() {
                    if o.is_none() {
                        return Err(ValidationError::new("Value is required"));
                    }
                } else if let Some(v) = value.downcast_ref::<Vec<String>>() {
                    if v.is_empty() {
                        return Err(ValidationError::new("Value is required"));
                    }
                }
            }
        }
        
        Ok(())
    }
}

/// Validates that a value is required if another field does not have a specific value
pub struct RequiredWithout<T: PartialEq + 'static> {
    pub other_field: Box<dyn Fn() -> Option<T> + Send + Sync>,
    pub expected_value: T,
}

impl<T: PartialEq + Send + Sync + 'static> Rule for RequiredWithout<T> {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(other_value) = (self.other_field)() {
            if other_value != self.expected_value {
                // Check if value is empty or null
                if let Some(s) = value.downcast_ref::<String>() {
                    if s.is_empty() {
                        return Err(ValidationError::new("Value is required"));
                    }
                } else if let Some(s) = value.downcast_ref::<&str>() {
                    if s.is_empty() {
                        return Err(ValidationError::new("Value is required"));
                    }
                } else if let Some(o) = value.downcast_ref::<Option<String>>() {
                    if o.is_none() {
                        return Err(ValidationError::new("Value is required"));
                    }
                } else if let Some(v) = value.downcast_ref::<Vec<String>>() {
                    if v.is_empty() {
                        return Err(ValidationError::new("Value is required"));
                    }
                }
            }
        }
        
        Ok(())
    }
}

/// Validates that a value is required if any of the specified conditions are true
pub struct RequiredIfAny {
    pub conditions: Vec<Box<dyn Fn() -> bool + Send + Sync>>,
}

impl Rule for RequiredIfAny {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if self.conditions.iter().any(|condition| condition()) {
            // Check if value is empty or null
            if let Some(s) = value.downcast_ref::<String>() {
                if s.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(s) = value.downcast_ref::<&str>() {
                if s.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(o) = value.downcast_ref::<Option<String>>() {
                if o.is_none() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(v) = value.downcast_ref::<Vec<String>>() {
                if v.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            }
        }
        
        Ok(())
    }
}

/// Validates that a value is required if all of the specified conditions are true
pub struct RequiredIfAll {
    pub conditions: Vec<Box<dyn Fn() -> bool + Send + Sync>>,
}

impl Rule for RequiredIfAll {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if self.conditions.iter().all(|condition| condition()) {
            // Check if value is empty or null
            if let Some(s) = value.downcast_ref::<String>() {
                if s.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(s) = value.downcast_ref::<&str>() {
                if s.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(o) = value.downcast_ref::<Option<String>>() {
                if o.is_none() {
                    return Err(ValidationError::new("Value is required"));
                }
            } else if let Some(v) = value.downcast_ref::<Vec<String>>() {
                if v.is_empty() {
                    return Err(ValidationError::new("Value is required"));
                }
            }
        }
        
        Ok(())
    }
}
