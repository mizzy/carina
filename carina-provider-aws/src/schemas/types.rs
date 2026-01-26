//! AWS-specific type definitions

use carina_core::schema::AttributeType;

/// AWS region enum type
pub fn aws_region() -> AttributeType {
    AttributeType::Enum(vec![
        "ap_northeast_1".to_string(),
        "ap_northeast_2".to_string(),
        "ap_northeast_3".to_string(),
        "ap_southeast_1".to_string(),
        "ap_southeast_2".to_string(),
        "ap_south_1".to_string(),
        "us_east_1".to_string(),
        "us_east_2".to_string(),
        "us_west_1".to_string(),
        "us_west_2".to_string(),
        "eu_west_1".to_string(),
        "eu_west_2".to_string(),
        "eu_west_3".to_string(),
        "eu_central_1".to_string(),
        "eu_north_1".to_string(),
        "ca_central_1".to_string(),
        "sa_east_1".to_string(),
    ])
}
