//! Validation using generated CloudFormation types
//!
//! This module provides validation of resource attributes using
//! types generated from CloudFormation JSON schemas.
//!
//! Carina DSL uses snake_case attribute names, which are converted
//! to CamelCase for validation against CloudFormation schemas.

use std::collections::HashMap;

use carina_core::resource::Value;

use crate::case_convert::attributes_to_camel_case;
use crate::generated::aws_s3_bucket::VersioningConfigurationStatus;

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Result type for validation
pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// Validate S3 bucket attributes against CloudFormation schema
/// Accepts both snake_case and CamelCase attribute names
pub fn validate_s3_bucket(attributes: &HashMap<String, Value>) -> ValidationResult {
    let mut errors = Vec::new();

    // Convert to CamelCase for validation
    let attrs = attributes_to_camel_case(attributes);

    // Validate BucketName if present
    if let Some(value) = attrs.get("BucketName") {
        if let Value::String(name) = value {
            // BucketName validation: 3-63 characters, lowercase, alphanumeric and hyphens
            if name.len() < 3 || name.len() > 63 {
                errors.push(ValidationError {
                    path: "bucket_name".to_string(),
                    message: format!(
                        "bucket_name must be between 3 and 63 characters, got {}",
                        name.len()
                    ),
                });
            }
        } else {
            errors.push(ValidationError {
                path: "bucket_name".to_string(),
                message: "bucket_name must be a string".to_string(),
            });
        }
    }

    // Validate VersioningConfiguration if present
    if let Some(value) = attrs.get("VersioningConfiguration") {
        validate_versioning_configuration(value, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_versioning_configuration(value: &Value, errors: &mut Vec<ValidationError>) {
    match value {
        Value::Map(map) => {
            // Validate Status field
            if let Some(status_value) = map.get("Status") {
                if let Value::String(status) = status_value {
                    // Accept both lowercase (enabled) and CamelCase (Enabled)
                    let normalized = capitalize_first(status);
                    if normalized.parse::<VersioningConfigurationStatus>().is_err() {
                        errors.push(ValidationError {
                            path: "versioning_configuration.status".to_string(),
                            message: format!(
                                "Invalid status value '{}'. Must be one of: enabled, suspended",
                                status
                            ),
                        });
                    }
                } else {
                    errors.push(ValidationError {
                        path: "versioning_configuration.status".to_string(),
                        message: "status must be a string".to_string(),
                    });
                }
            }
        }
        _ => {
            errors.push(ValidationError {
                path: "versioning_configuration".to_string(),
                message: "versioning_configuration must be an object".to_string(),
            });
        }
    }
}

/// Capitalize the first letter of a string
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Validate a resource based on its type
pub fn validate_resource(
    resource_type: &str,
    attributes: &HashMap<String, Value>,
) -> ValidationResult {
    match resource_type {
        "s3_bucket" => validate_s3_bucket(attributes),
        _ => Ok(()), // Unknown types pass validation (for extensibility)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_versioning_enabled_lowercase() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "bucket_name".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "versioning_configuration".to_string(),
            Value::Map(HashMap::from([(
                "status".to_string(),
                Value::String("enabled".to_string()),
            )])),
        );

        assert!(validate_s3_bucket(&attrs).is_ok());
    }

    #[test]
    fn test_valid_versioning_enabled_camel_case() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "BucketName".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "VersioningConfiguration".to_string(),
            Value::Map(HashMap::from([(
                "Status".to_string(),
                Value::String("Enabled".to_string()),
            )])),
        );

        assert!(validate_s3_bucket(&attrs).is_ok());
    }

    #[test]
    fn test_valid_versioning_suspended() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "bucket_name".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "versioning_configuration".to_string(),
            Value::Map(HashMap::from([(
                "status".to_string(),
                Value::String("suspended".to_string()),
            )])),
        );

        assert!(validate_s3_bucket(&attrs).is_ok());
    }

    #[test]
    fn test_invalid_versioning_status() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "bucket_name".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "versioning_configuration".to_string(),
            Value::Map(HashMap::from([(
                "status".to_string(),
                Value::String("invalid".to_string()),
            )])),
        );

        let result = validate_s3_bucket(&attrs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("Invalid status value"));
        assert!(errors[0].message.contains("enabled, suspended"));
    }

    #[test]
    fn test_invalid_bucket_name_too_short() {
        let mut attrs = HashMap::new();
        attrs.insert("bucket_name".to_string(), Value::String("ab".to_string()));

        let result = validate_s3_bucket(&attrs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors[0].message.contains("between 3 and 63 characters"));
    }
}
