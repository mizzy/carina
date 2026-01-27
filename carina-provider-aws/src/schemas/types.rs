//! AWS-specific type definitions

use carina_core::resource::Value;
use carina_core::schema::AttributeType;

/// Valid AWS regions (in AWS format with hyphens)
const VALID_REGIONS: &[&str] = &[
    "ap-northeast-1",
    "ap-northeast-2",
    "ap-northeast-3",
    "ap-southeast-1",
    "ap-southeast-2",
    "ap-south-1",
    "us-east-1",
    "us-east-2",
    "us-west-1",
    "us-west-2",
    "eu-west-1",
    "eu-west-2",
    "eu-west-3",
    "eu-central-1",
    "eu-north-1",
    "ca-central-1",
    "sa-east-1",
];

/// AWS region type with custom validation
/// Accepts:
/// - DSL format: aws.Region.ap_northeast_1
/// - AWS string format: "ap-northeast-1"
pub fn aws_region() -> AttributeType {
    AttributeType::Custom {
        name: "Region".to_string(),
        base: Box::new(AttributeType::String),
        validate: |value| {
            if let Value::String(s) = value {
                // Normalize the input to AWS format (hyphens)
                let normalized = normalize_region(s);
                if VALID_REGIONS.contains(&normalized.as_str()) {
                    Ok(())
                } else {
                    Err(format!(
                        "Invalid region '{}', expected one of: {} or DSL format like aws.Region.ap_northeast_1",
                        s,
                        VALID_REGIONS.join(", ")
                    ))
                }
            } else {
                Err("Expected string".to_string())
            }
        },
    }
}

/// Normalize region string to AWS format (hyphens)
/// - "aws.Region.ap_northeast_1" -> "ap-northeast-1"
/// - "ap_northeast_1" -> "ap-northeast-1"
/// - "ap-northeast-1" -> "ap-northeast-1"
fn normalize_region(s: &str) -> String {
    // Extract region part from DSL format (aws.Region.xxx)
    let region_part = if s.contains('.') {
        s.split('.').next_back().unwrap_or(s)
    } else {
        s
    };
    // Convert underscores to hyphens
    region_part.replace('_', "-")
}

/// S3 bucket versioning status
/// - Enabled: Versioning is enabled
/// - Suspended: Versioning is suspended (previously enabled)
pub fn versioning_status() -> AttributeType {
    AttributeType::Enum(vec!["Enabled".to_string(), "Suspended".to_string()])
}

#[cfg(test)]
mod tests {
    use super::*;

    // Region validation tests

    #[test]
    fn region_accepts_aws_format() {
        let region_type = aws_region();
        assert!(
            region_type
                .validate(&Value::String("ap-northeast-1".to_string()))
                .is_ok()
        );
    }

    #[test]
    fn region_accepts_dsl_format() {
        let region_type = aws_region();
        assert!(
            region_type
                .validate(&Value::String("aws.Region.ap_northeast_1".to_string()))
                .is_ok()
        );
    }

    #[test]
    fn region_accepts_dsl_format_without_aws_prefix() {
        let region_type = aws_region();
        assert!(
            region_type
                .validate(&Value::String("Region.ap_northeast_1".to_string()))
                .is_ok()
        );
    }

    #[test]
    fn region_rejects_invalid_region() {
        let region_type = aws_region();
        let result = region_type.validate(&Value::String("invalid-region".to_string()));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid region"));
        assert!(err.contains("ap-northeast-1")); // Should suggest valid regions
    }

    #[test]
    fn region_rejects_availability_zone() {
        let region_type = aws_region();
        // ap-northeast-1a is an AZ, not a region
        assert!(
            region_type
                .validate(&Value::String("ap-northeast-1a".to_string()))
                .is_err()
        );
    }

    #[test]
    fn region_validates_all_valid_regions() {
        let region_type = aws_region();
        for region in VALID_REGIONS {
            assert!(
                region_type
                    .validate(&Value::String(region.to_string()))
                    .is_ok(),
                "Region {} should be valid",
                region
            );
        }
    }

    // Versioning status tests

    #[test]
    fn versioning_accepts_enabled() {
        let versioning = versioning_status();
        assert!(
            versioning
                .validate(&Value::String("Enabled".to_string()))
                .is_ok()
        );
    }

    #[test]
    fn versioning_accepts_suspended() {
        let versioning = versioning_status();
        assert!(
            versioning
                .validate(&Value::String("Suspended".to_string()))
                .is_ok()
        );
    }

    #[test]
    fn versioning_rejects_lowercase() {
        let versioning = versioning_status();
        assert!(
            versioning
                .validate(&Value::String("enabled".to_string()))
                .is_err()
        );
    }

    #[test]
    fn versioning_rejects_bool() {
        let versioning = versioning_status();
        assert!(versioning.validate(&Value::Bool(true)).is_err());
        assert!(versioning.validate(&Value::Bool(false)).is_err());
    }

    #[test]
    fn versioning_rejects_disabled() {
        let versioning = versioning_status();
        assert!(
            versioning
                .validate(&Value::String("Disabled".to_string()))
                .is_err()
        );
    }
}
