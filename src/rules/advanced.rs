use regex::Regex;

use crate::error::ValidationError;
use crate::rules::Rule;

/// Validates password complexity
pub struct Password {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_digit: bool,
    pub require_special: bool,
}

impl Rule for Password {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            if s.len() < self.min_length {
                return Err(ValidationError::new(format!(
                    "Password must be at least {} characters long", self.min_length
                )));
            }
            
            if self.require_uppercase && !s.chars().any(|c| c.is_uppercase()) {
                return Err(ValidationError::new(
                    "Password must contain at least one uppercase letter"
                ));
            }
            
            if self.require_lowercase && !s.chars().any(|c| c.is_lowercase()) {
                return Err(ValidationError::new(
                    "Password must contain at least one lowercase letter"
                ));
            }
            
            if self.require_digit && !s.chars().any(|c| c.is_digit(10)) {
                return Err(ValidationError::new(
                    "Password must contain at least one digit"
                ));
            }
            
            if self.require_special && !s.chars().any(|c| !c.is_alphanumeric()) {
                return Err(ValidationError::new(
                    "Password must contain at least one special character"
                ));
            }
            
            Ok(())
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

/// Validates credit card numbers
pub struct CreditCard;

impl Rule for CreditCard {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_credit_card(s)
        } else if let Some(s) = value.downcast_ref::<&str>() {
            validate_credit_card(s)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_credit_card(card: &str) -> Result<(), ValidationError> {
    // Remove spaces and dashes
    let card = card.replace([' ', '-'], "");
    
    // Check if the card number contains only digits
    if !card.chars().all(|c| c.is_digit(10)) {
        return Err(ValidationError::new("Credit card number must contain only digits"));
    }
    
    // Check length (most cards are 13-19 digits)
    if card.len() < 13 || card.len() > 19 {
        return Err(ValidationError::new("Credit card number has invalid length"));
    }
    
    // Luhn algorithm validation
    let mut sum = 0;
    let mut double = false;
    
    for c in card.chars().rev() {
        let mut digit = c.to_digit(10).unwrap();
        
        if double {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        
        sum += digit;
        double = !double;
    }
    
    if sum % 10 != 0 {
        return Err(ValidationError::new("Invalid credit card number"));
    }
    
    Ok(())
}

/// Validates semantic version strings
pub struct SemVer;

impl Rule for SemVer {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_semver(s)
        } else if let Some(s) = value.downcast_ref::<&str>() {
            validate_semver(s)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_semver(version: &str) -> Result<(), ValidationError> {
    let semver_regex = regex::Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
    
    if !semver_regex.is_match(version) {
        return Err(ValidationError::new("Invalid semantic version format"));
    }
    
    Ok(())
}

/// Validates domain names
pub struct Domain;

impl Rule for Domain {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_domain(s)
        } else if let Some(s) = value.downcast_ref::<&str>() {
            validate_domain(s)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_domain(domain: &str) -> Result<(), ValidationError> {
    let domain_regex = regex::Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9]$").unwrap();
    
    if !domain_regex.is_match(domain) {
        return Err(ValidationError::new("Invalid domain name format"));
    }
    
    Ok(())
}

/// Validates port numbers
pub struct Port;

impl Rule for Port {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(port) = value.downcast_ref::<u16>() {
            if *port == 0 {
                return Err(ValidationError::new("Port number cannot be 0"));
            }
            Ok(())
        } else if let Some(port) = value.downcast_ref::<i32>() {
            if *port <= 0 || *port > 65535 {
                return Err(ValidationError::new("Port number must be between 1 and 65535"));
            }
            Ok(())
        } else if let Some(s) = value.downcast_ref::<String>() {
            match s.parse::<u16>() {
                Ok(port) => {
                    if port == 0 {
                        return Err(ValidationError::new("Port number cannot be 0"));
                    }
                    Ok(())
                },
                Err(_) => Err(ValidationError::new("Invalid port number format")),
            }
        } else if let Some(s) = value.downcast_ref::<&str>() {
            match s.parse::<u16>() {
                Ok(port) => {
                    if port == 0 {
                        return Err(ValidationError::new("Port number cannot be 0"));
                    }
                    Ok(())
                },
                Err(_) => Err(ValidationError::new("Invalid port number format")),
            }
        } else {
            Err(ValidationError::new("Value must be a port number (u16, i32, or string)"))
        }
    }
}

/// Validates IP addresses
pub struct IP {
    pub allow_v4: bool,
    pub allow_v6: bool,
}

impl Rule for IP {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            validate_ip(s, self)
        } else if let Some(s) = value.downcast_ref::<&str>() {
            validate_ip(s, self)
        } else {
            Err(ValidationError::new("Value must be a string"))
        }
    }
}

fn validate_ip(ip: &str, ip_rule: &IP) -> Result<(), ValidationError> {
    let ipv4_regex = regex::Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    let ipv6_regex = regex::Regex::new(r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$").unwrap();
    
    let is_ipv4 = ipv4_regex.is_match(ip);
    let is_ipv6 = ipv6_regex.is_match(ip);
    
    if (is_ipv4 && ip_rule.allow_v4) || (is_ipv6 && ip_rule.allow_v6) {
        Ok(())
    } else if is_ipv4 && !ip_rule.allow_v4 {
        Err(ValidationError::new("IPv4 addresses are not allowed"))
    } else if is_ipv6 && !ip_rule.allow_v6 {
        Err(ValidationError::new("IPv6 addresses are not allowed"))
    } else {
        Err(ValidationError::new("Invalid IP address format"))
    }
}

/// Validates against a regular expression
pub struct RegexRule {
    pub pattern: String,
    pub regex: regex::Regex,
}

impl RegexRule {
    pub fn new(pattern: &str) -> Result<Self, ValidationError> {
        match regex::Regex::new(pattern) {
            Ok(regex) => Ok(RegexRule {
                pattern: pattern.to_string(),
                regex,
            }),
            Err(_) => Err(ValidationError::new("Invalid regex pattern")),
        }
    }
}

impl Rule for RegexRule {
    fn validate_any(&self, value: &dyn std::any::Any) -> Result<(), ValidationError> {
        if let Some(s) = value.downcast_ref::<String>() {
            if !self.regex.is_match(s) {
                return Err(ValidationError::new(format!(
                    "Value does not match pattern: {}", self.pattern
                )));
            }
        } else if let Some(s) = value.downcast_ref::<&str>() {
            if !self.regex.is_match(s) {
                return Err(ValidationError::new(format!(
                    "Value does not match pattern: {}", self.pattern
                )));
            }
        } else {
            return Err(ValidationError::new("Value must be a string"));
        }
        
        Ok(())
    }
}
