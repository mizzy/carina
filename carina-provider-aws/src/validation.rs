//! Validation using generated CloudFormation types
//!
//! This module provides schema-based validation of resource attributes using
//! types generated from CloudFormation JSON schemas via typify.
//!
//! The validation process:
//! 1. Convert snake_case attributes to CamelCase
//! 2. Convert Carina Value to serde_json::Value
//! 3. Deserialize into generated types to validate against schema

use std::collections::HashMap;

use carina_core::resource::Value;

use crate::case_convert::attributes_to_camel_case;
use crate::generated::aws_s3_bucket::{
    AccelerateConfiguration, LifecycleConfiguration, PublicAccessBlockConfiguration,
    VersioningConfiguration,
};

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.message)
        } else {
            write!(f, "{}: {}", self.path, self.message)
        }
    }
}

impl std::error::Error for ValidationError {}

/// Result type for validation
pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// Convert Carina Value to serde_json::Value
fn value_to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Int(i) => serde_json::Value::Number((*i).into()),
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::List(items) => serde_json::Value::Array(items.iter().map(value_to_json).collect()),
        Value::Map(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
        Value::ResourceRef(binding, attr) => {
            serde_json::Value::String(format!("${{{}:{}}}", binding, attr))
        }
    }
}

/// Validate S3 bucket attributes against CloudFormation schema
/// Uses generated types for schema-based validation
pub fn validate_s3_bucket(attributes: &HashMap<String, Value>) -> ValidationResult {
    let mut errors = Vec::new();

    // Convert to CamelCase (keys and enum values)
    let attrs = attributes_to_camel_case(attributes);

    // Validate BucketName if present
    if let Some(value) = attrs.get("BucketName") {
        if let Value::String(name) = value {
            // BucketName validation based on schema:
            // minLength: 3, maxLength: 63, pattern: ^[a-z0-9][a-z0-9.-]*[a-z0-9]$
            if name.len() < 3 {
                errors.push(ValidationError {
                    path: "bucket_name".to_string(),
                    message: format!(
                        "bucket_name must be at least 3 characters, got {}",
                        name.len()
                    ),
                });
            }
            if name.len() > 63 {
                errors.push(ValidationError {
                    path: "bucket_name".to_string(),
                    message: format!(
                        "bucket_name must be at most 63 characters, got {}",
                        name.len()
                    ),
                });
            }
            // Pattern validation
            let pattern = regex::Regex::new(r"^[a-z0-9][a-z0-9.-]*[a-z0-9]$").unwrap();
            if name.len() >= 2 && !pattern.is_match(name) {
                errors.push(ValidationError {
                    path: "bucket_name".to_string(),
                    message: "bucket_name must match pattern: lowercase letters, numbers, dots, and hyphens".to_string(),
                });
            }
        } else {
            errors.push(ValidationError {
                path: "bucket_name".to_string(),
                message: "bucket_name must be a string".to_string(),
            });
        }
    }

    // Validate VersioningConfiguration using generated type
    if let Some(value) = attrs.get("VersioningConfiguration") {
        let json = value_to_json(value);
        if let Err(e) = serde_json::from_value::<VersioningConfiguration>(json) {
            errors.push(ValidationError {
                path: "versioning_configuration".to_string(),
                message: format_serde_error(&e),
            });
        }
    }

    // Validate LifecycleConfiguration using generated type
    if let Some(value) = attrs.get("LifecycleConfiguration") {
        let json = value_to_json(value);
        if let Err(e) = serde_json::from_value::<LifecycleConfiguration>(json) {
            errors.push(ValidationError {
                path: "lifecycle_configuration".to_string(),
                message: format_serde_error(&e),
            });
        }
    }

    // Validate AccelerateConfiguration using generated type
    if let Some(value) = attrs.get("AccelerateConfiguration") {
        let json = value_to_json(value);
        if let Err(e) = serde_json::from_value::<AccelerateConfiguration>(json) {
            errors.push(ValidationError {
                path: "accelerate_configuration".to_string(),
                message: format_serde_error(&e),
            });
        }
    }

    // Validate PublicAccessBlockConfiguration using generated type
    if let Some(value) = attrs.get("PublicAccessBlockConfiguration") {
        let json = value_to_json(value);
        if let Err(e) = serde_json::from_value::<PublicAccessBlockConfiguration>(json) {
            errors.push(ValidationError {
                path: "public_access_block_configuration".to_string(),
                message: format_serde_error(&e),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Format serde deserialization error for user display
fn format_serde_error(e: &serde_json::Error) -> String {
    let msg = e.to_string();
    // Make error messages more user-friendly
    if msg.contains("unknown variant") {
        // Extract the invalid value and expected values from the error
        msg.replace("unknown variant", "invalid value")
    } else if msg.contains("missing field") {
        msg.replace("missing field", "missing required field")
    } else {
        msg
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
    fn test_valid_versioning_enabled() {
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
        assert!(errors[0].path.contains("versioning_configuration"));
    }

    #[test]
    fn test_invalid_bucket_name_too_short() {
        let mut attrs = HashMap::new();
        attrs.insert("bucket_name".to_string(), Value::String("ab".to_string()));

        let result = validate_s3_bucket(&attrs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors[0].message.contains("at least 3 characters"));
    }

    #[test]
    fn test_valid_lifecycle_configuration_enabled() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "bucket_name".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "lifecycle_configuration".to_string(),
            Value::Map(HashMap::from([(
                "rules".to_string(),
                Value::List(vec![Value::Map(HashMap::from([(
                    "status".to_string(),
                    Value::String("enabled".to_string()),
                )]))]),
            )])),
        );

        assert!(validate_s3_bucket(&attrs).is_ok());
    }

    #[test]
    fn test_valid_lifecycle_configuration_disabled() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "bucket_name".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "lifecycle_configuration".to_string(),
            Value::Map(HashMap::from([(
                "rules".to_string(),
                Value::List(vec![Value::Map(HashMap::from([(
                    "status".to_string(),
                    Value::String("disabled".to_string()),
                )]))]),
            )])),
        );

        assert!(validate_s3_bucket(&attrs).is_ok());
    }

    #[test]
    fn test_invalid_lifecycle_status() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "bucket_name".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "lifecycle_configuration".to_string(),
            Value::Map(HashMap::from([(
                "rules".to_string(),
                Value::List(vec![Value::Map(HashMap::from([
                    ("status".to_string(), Value::String("suspended".to_string())), // Invalid for Rule
                ]))]),
            )])),
        );

        let result = validate_s3_bucket(&attrs);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors[0].path.contains("lifecycle_configuration"));
    }
}
