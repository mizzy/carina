//! Case conversion utilities for Carina DSL to CloudFormation attribute names
//!
//! Carina uses snake_case (e.g., `bucket_name`, `versioning_configuration`)
//! CloudFormation uses CamelCase (e.g., `BucketName`, `VersioningConfiguration`)
//!
//! Enum values are also converted: `enabled` -> `Enabled`

use std::collections::HashMap;

use carina_core::resource::Value;

/// Convert snake_case to CamelCase (PascalCase)
/// e.g., "bucket_name" -> "BucketName"
pub fn to_camel_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

/// Convert CamelCase to snake_case
/// e.g., "BucketName" -> "bucket_name"
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// Check if a string looks like an enum value (single lowercase word)
/// e.g., "enabled", "suspended", "disabled"
fn is_likely_enum_value(s: &str) -> bool {
    !s.is_empty()
        && s.chars().all(|c| c.is_ascii_lowercase())
        && !s.contains(|c: char| c.is_whitespace() || c == '-' || c == '.')
}

/// Capitalize first letter of a string
/// e.g., "enabled" -> "Enabled"
pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Convert attributes from snake_case to CamelCase (for sending to AWS)
pub fn attributes_to_camel_case(attributes: &HashMap<String, Value>) -> HashMap<String, Value> {
    attributes
        .iter()
        .map(|(k, v)| (to_camel_case(k), value_to_camel_case(v)))
        .collect()
}

/// Convert attributes from CamelCase to snake_case (for reading from AWS)
pub fn attributes_to_snake_case(attributes: &HashMap<String, Value>) -> HashMap<String, Value> {
    attributes
        .iter()
        .map(|(k, v)| (to_snake_case(k), value_to_snake_case(v)))
        .collect()
}

/// Recursively convert Value keys to CamelCase
/// Also capitalizes single-word lowercase strings (likely enum values)
fn value_to_camel_case(value: &Value) -> Value {
    match value {
        Value::Map(map) => {
            let converted: HashMap<String, Value> = map
                .iter()
                .map(|(k, v)| (to_camel_case(k), value_to_camel_case(v)))
                .collect();
            Value::Map(converted)
        }
        Value::List(items) => Value::List(items.iter().map(value_to_camel_case).collect()),
        Value::String(s) => {
            // Capitalize single-word lowercase strings (likely enum values)
            if is_likely_enum_value(s) {
                Value::String(capitalize_first(s))
            } else {
                Value::String(s.clone())
            }
        }
        other => other.clone(),
    }
}

/// Recursively convert Value keys to snake_case
/// Also converts CamelCase enum values to lowercase
fn value_to_snake_case(value: &Value) -> Value {
    match value {
        Value::Map(map) => {
            let converted: HashMap<String, Value> = map
                .iter()
                .map(|(k, v)| (to_snake_case(k), value_to_snake_case(v)))
                .collect();
            Value::Map(converted)
        }
        Value::List(items) => Value::List(items.iter().map(value_to_snake_case).collect()),
        Value::String(s) => {
            // Convert CamelCase single words to lowercase (likely enum values from AWS)
            // e.g., "Enabled" -> "enabled"
            if !s.is_empty()
                && s.chars().next().unwrap().is_uppercase()
                && s.chars().skip(1).all(|c| c.is_ascii_lowercase())
            {
                Value::String(s.to_lowercase())
            } else {
                Value::String(s.clone())
            }
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("bucket_name"), "BucketName");
        assert_eq!(
            to_camel_case("versioning_configuration"),
            "VersioningConfiguration"
        );
        assert_eq!(to_camel_case("status"), "Status");
        assert_eq!(to_camel_case("id"), "Id");
        assert_eq!(to_camel_case("expiration_in_days"), "ExpirationInDays");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("BucketName"), "bucket_name");
        assert_eq!(
            to_snake_case("VersioningConfiguration"),
            "versioning_configuration"
        );
        assert_eq!(to_snake_case("Status"), "status");
        assert_eq!(to_snake_case("Id"), "id");
        assert_eq!(to_snake_case("ExpirationInDays"), "expiration_in_days");
    }

    #[test]
    fn test_is_likely_enum_value() {
        assert!(is_likely_enum_value("enabled"));
        assert!(is_likely_enum_value("suspended"));
        assert!(is_likely_enum_value("disabled"));
        assert!(!is_likely_enum_value("Enabled")); // Already capitalized
        assert!(!is_likely_enum_value("my-bucket")); // Contains hyphen
        assert!(!is_likely_enum_value("my_bucket")); // Contains underscore
        assert!(!is_likely_enum_value("my.bucket")); // Contains dot
        assert!(!is_likely_enum_value("")); // Empty
    }

    #[test]
    fn test_attributes_to_camel_case() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "bucket_name".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "versioning_configuration".to_string(),
            Value::Map(HashMap::from([(
                "status".to_string(),
                Value::String("enabled".to_string()), // lowercase input
            )])),
        );

        let converted = attributes_to_camel_case(&attrs);

        assert!(converted.contains_key("BucketName"));
        assert!(converted.contains_key("VersioningConfiguration"));

        // Bucket name should NOT be capitalized (contains hyphen)
        assert_eq!(
            converted.get("BucketName"),
            Some(&Value::String("my-bucket".to_string()))
        );

        if let Value::Map(vc) = converted.get("VersioningConfiguration").unwrap() {
            assert!(vc.contains_key("Status"));
            // Enum value should be capitalized
            assert_eq!(
                vc.get("Status"),
                Some(&Value::String("Enabled".to_string()))
            );
        } else {
            panic!("Expected Map");
        }
    }

    #[test]
    fn test_attributes_to_snake_case() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "BucketName".to_string(),
            Value::String("my-bucket".to_string()),
        );
        attrs.insert(
            "VersioningConfiguration".to_string(),
            Value::Map(HashMap::from([(
                "Status".to_string(),
                Value::String("Enabled".to_string()), // CamelCase input
            )])),
        );

        let converted = attributes_to_snake_case(&attrs);

        assert!(converted.contains_key("bucket_name"));
        assert!(converted.contains_key("versioning_configuration"));

        if let Value::Map(vc) = converted.get("versioning_configuration").unwrap() {
            assert!(vc.contains_key("status"));
            // Enum value should be converted to lowercase
            assert_eq!(
                vc.get("status"),
                Some(&Value::String("enabled".to_string()))
            );
        } else {
            panic!("Expected Map");
        }
    }

    #[test]
    fn test_lifecycle_configuration_conversion() {
        let mut attrs = HashMap::new();
        attrs.insert(
            "lifecycle_configuration".to_string(),
            Value::Map(HashMap::from([(
                "rules".to_string(),
                Value::List(vec![Value::Map(HashMap::from([
                    ("status".to_string(), Value::String("disabled".to_string())),
                    ("expiration_in_days".to_string(), Value::Int(30)),
                ]))]),
            )])),
        );

        let converted = attributes_to_camel_case(&attrs);

        if let Some(Value::Map(lc)) = converted.get("LifecycleConfiguration") {
            if let Some(Value::List(rules)) = lc.get("Rules") {
                if let Some(Value::Map(rule)) = rules.first() {
                    // "disabled" should be converted to "Disabled"
                    assert_eq!(
                        rule.get("Status"),
                        Some(&Value::String("Disabled".to_string()))
                    );
                } else {
                    panic!("Expected rule map");
                }
            } else {
                panic!("Expected rules list");
            }
        } else {
            panic!("Expected LifecycleConfiguration map");
        }
    }
}
