//! AWS Cloud Control API Provider
//!
//! Uses AWS Cloud Control API for resource management, enabling a unified
//! approach to managing AWS resources with CloudFormation-compatible schemas.
//!
//! Carina DSL uses snake_case for attribute names, which are automatically
//! converted to CloudFormation's CamelCase format.

use std::collections::HashMap;
use std::time::Duration;

use aws_sdk_cloudcontrol::Client as CloudControlClient;
use aws_sdk_cloudcontrol::operation::get_resource_request_status::GetResourceRequestStatusOutput;
use aws_sdk_cloudcontrol::types::OperationStatus;
use carina_core::provider::{
    BoxFuture, Provider, ProviderError, ProviderResult, ResourceSchema, ResourceType,
};
use carina_core::resource::{Resource, ResourceId, State, Value};

pub mod case_convert;
pub mod generated;
pub mod validation;

use case_convert::{attributes_to_camel_case, attributes_to_snake_case};

/// Resource type for S3 Bucket
struct S3BucketType;

impl ResourceType for S3BucketType {
    fn name(&self) -> &'static str {
        "s3_bucket"
    }

    fn schema(&self) -> ResourceSchema {
        ResourceSchema::default()
    }
}

/// AWS Provider using Cloud Control API
pub struct AwsProvider {
    client: CloudControlClient,
    region: String,
}

impl AwsProvider {
    /// Create a new AWS Provider
    pub async fn new(region: impl Into<String>) -> Self {
        let region_str = region.into();
        let config = aws_config::from_env()
            .region(aws_config::Region::new(region_str.clone()))
            .load()
            .await;
        let client = CloudControlClient::new(&config);

        Self {
            client,
            region: region_str,
        }
    }

    /// Create an AWS Provider with a custom client (for testing)
    pub fn with_client(client: CloudControlClient, region: impl Into<String>) -> Self {
        Self {
            client,
            region: region.into(),
        }
    }

    /// Get the region this provider is configured for
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Convert internal resource type to CloudFormation type name
    fn to_cf_type_name(resource_type: &str) -> Result<&'static str, ProviderError> {
        match resource_type {
            "s3_bucket" => Ok("AWS::S3::Bucket"),
            _ => Err(ProviderError::new(format!(
                "Unknown resource type: {}",
                resource_type
            ))),
        }
    }

    /// Convert Value to serde_json::Value
    fn value_to_json(value: &Value) -> serde_json::Value {
        match value {
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Int(i) => serde_json::Value::Number((*i).into()),
            Value::Bool(b) => serde_json::Value::Bool(*b),
            Value::List(items) => {
                serde_json::Value::Array(items.iter().map(Self::value_to_json).collect())
            }
            Value::Map(map) => {
                let obj: serde_json::Map<String, serde_json::Value> = map
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::value_to_json(v)))
                    .collect();
                serde_json::Value::Object(obj)
            }
            Value::ResourceRef(binding, attr) => {
                // Resource references should be resolved before this point
                serde_json::Value::String(format!("${{{}:{}}}", binding, attr))
            }
        }
    }

    /// Convert serde_json::Value to Value
    fn json_to_value(json: &serde_json::Value) -> Option<Value> {
        match json {
            serde_json::Value::Null => None,
            serde_json::Value::Bool(b) => Some(Value::Bool(*b)),
            serde_json::Value::Number(n) => n.as_i64().map(Value::Int),
            serde_json::Value::String(s) => Some(Value::String(s.clone())),
            serde_json::Value::Array(arr) => {
                let items: Vec<Value> = arr.iter().filter_map(Self::json_to_value).collect();
                Some(Value::List(items))
            }
            serde_json::Value::Object(obj) => {
                let map: HashMap<String, Value> = obj
                    .iter()
                    .filter_map(|(k, v)| Self::json_to_value(v).map(|val| (k.clone(), val)))
                    .collect();
                Some(Value::Map(map))
            }
        }
    }

    /// Convert resource attributes to JSON for Cloud Control API
    /// Converts snake_case keys to CamelCase for CloudFormation compatibility
    /// Filters out internal properties (_type, _provider)
    fn attributes_to_json(attributes: &HashMap<String, Value>) -> String {
        let camel_case_attrs = attributes_to_camel_case(attributes);
        let obj: serde_json::Map<String, serde_json::Value> = camel_case_attrs
            .iter()
            .filter(|(k, _)| k.as_str() != "Type" && k.as_str() != "Provider")
            .map(|(k, v)| (k.clone(), Self::value_to_json(v)))
            .collect();
        serde_json::to_string(&serde_json::Value::Object(obj)).unwrap_or_default()
    }

    /// Parse JSON properties from Cloud Control API response
    /// Converts CamelCase keys to snake_case for Carina DSL compatibility
    /// Filters out read-only properties
    fn parse_properties(properties: &str) -> HashMap<String, Value> {
        match serde_json::from_str::<serde_json::Value>(properties) {
            Ok(serde_json::Value::Object(obj)) => {
                let camel_case_attrs: HashMap<String, Value> = obj
                    .iter()
                    .filter(|(k, _)| !Self::READ_ONLY_PROPERTIES.contains(&k.as_str()))
                    .filter_map(|(k, v)| Self::json_to_value(v).map(|val| (k.clone(), val)))
                    .collect();
                attributes_to_snake_case(&camel_case_attrs)
            }
            _ => HashMap::new(),
        }
    }

    /// Get the identifier for a resource
    /// Supports both snake_case (bucket_name) and CamelCase (BucketName) attribute names
    fn get_identifier(resource_type: &str, resource: &Resource) -> Result<String, ProviderError> {
        match resource_type {
            "s3_bucket" => {
                // For S3 buckets, the identifier is the BucketName (or bucket_name)
                resource
                    .attributes
                    .get("bucket_name")
                    .or_else(|| resource.attributes.get("BucketName"))
                    .and_then(|v| match v {
                        Value::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .ok_or_else(|| ProviderError::new("bucket_name is required for s3_bucket"))
            }
            _ => Err(ProviderError::new(format!(
                "Unknown resource type: {}",
                resource_type
            ))),
        }
    }

    /// Wait for an operation to complete
    async fn wait_for_completion(
        &self,
        request_token: &str,
    ) -> ProviderResult<GetResourceRequestStatusOutput> {
        let max_attempts = 60; // 2 minutes with 2-second intervals
        let mut attempts = 0;

        loop {
            let result = self
                .client
                .get_resource_request_status()
                .request_token(request_token)
                .send()
                .await
                .map_err(|e| {
                    ProviderError::new(format!("Failed to get operation status: {}", e))
                })?;

            if let Some(event) = result.progress_event() {
                match event.operation_status() {
                    Some(OperationStatus::Success) => return Ok(result),
                    Some(OperationStatus::Failed) => {
                        let msg = event.status_message().unwrap_or("Unknown error");
                        return Err(ProviderError::new(format!("Operation failed: {}", msg)));
                    }
                    Some(OperationStatus::CancelComplete) => {
                        return Err(ProviderError::new("Operation was cancelled"));
                    }
                    _ => {
                        // Still in progress (Pending, InProgress, CancelInProgress)
                    }
                }
            }

            attempts += 1;
            if attempts >= max_attempts {
                return Err(ProviderError::new("Operation timed out"));
            }

            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    /// Read-only properties that should be excluded from patches
    /// These are returned by AWS but cannot be updated
    const READ_ONLY_PROPERTIES: &'static [&'static str] = &[
        "Arn",
        "DomainName",
        "DualStackDomainName",
        "RegionalDomainName",
        "WebsiteURL",
    ];

    /// Generate JSON Patch document for update operation
    /// Converts snake_case keys to CamelCase for CloudFormation compatibility
    fn generate_patch(from: &State, to: &Resource) -> Result<String, ProviderError> {
        let mut patches = Vec::new();

        // Convert to CamelCase for comparison and patching
        let from_camel = attributes_to_camel_case(&from.attributes);
        let to_camel = attributes_to_camel_case(&to.attributes);

        // Find attributes that need to be added or modified
        for (key, new_value) in &to_camel {
            // Skip read-only and internal properties
            // Internal properties like _type become Type after CamelCase conversion
            if Self::READ_ONLY_PROPERTIES.contains(&key.as_str())
                || key == "Type"
                || key == "Provider"
            {
                continue;
            }

            let new_json = Self::value_to_json(new_value);

            if let Some(old_value) = from_camel.get(key) {
                let old_json = Self::value_to_json(old_value);
                if old_json != new_json {
                    // Replace existing value
                    patches.push(serde_json::json!({
                        "op": "replace",
                        "path": format!("/{}", key),
                        "value": new_json
                    }));
                }
            } else {
                // Add new attribute
                patches.push(serde_json::json!({
                    "op": "add",
                    "path": format!("/{}", key),
                    "value": new_json
                }));
            }
        }

        // Find attributes that need to be removed
        for key in from_camel.keys() {
            // Skip read-only and internal properties
            if Self::READ_ONLY_PROPERTIES.contains(&key.as_str())
                || key == "Type"
                || key == "Provider"
            {
                continue;
            }

            if !to_camel.contains_key(key) {
                patches.push(serde_json::json!({
                    "op": "remove",
                    "path": format!("/{}", key)
                }));
            }
        }

        serde_json::to_string(&patches)
            .map_err(|e| ProviderError::new(format!("Failed to serialize patch: {}", e)))
    }
}

impl Provider for AwsProvider {
    fn name(&self) -> &'static str {
        "aws"
    }

    fn resource_types(&self) -> Vec<Box<dyn ResourceType>> {
        vec![Box::new(S3BucketType)]
    }

    fn read(&self, id: &ResourceId) -> BoxFuture<'_, ProviderResult<State>> {
        let id = id.clone();
        Box::pin(async move {
            let type_name = Self::to_cf_type_name(&id.resource_type)?;

            let result = self
                .client
                .get_resource()
                .type_name(type_name)
                .identifier(&id.name)
                .send()
                .await;

            match result {
                Ok(output) => {
                    let attributes = output
                        .resource_description()
                        .and_then(|d| d.properties())
                        .map(Self::parse_properties)
                        .unwrap_or_default();

                    Ok(State::existing(id, attributes))
                }
                Err(e) => {
                    // Check if this is a "not found" error
                    let error_str = e.to_string();
                    if error_str.contains("ResourceNotFoundException")
                        || error_str.contains("NotFound")
                        || error_str.contains("does not exist")
                    {
                        Ok(State::not_found(id))
                    } else {
                        Err(
                            ProviderError::new(format!("Failed to read resource: {}", e))
                                .for_resource(id),
                        )
                    }
                }
            }
        })
    }

    fn create(&self, resource: &Resource) -> BoxFuture<'_, ProviderResult<State>> {
        let resource = resource.clone();
        Box::pin(async move {
            let type_name = Self::to_cf_type_name(&resource.id.resource_type)?;
            let desired_state = Self::attributes_to_json(&resource.attributes);
            let identifier = Self::get_identifier(&resource.id.resource_type, &resource)?;

            let result = self
                .client
                .create_resource()
                .type_name(type_name)
                .desired_state(&desired_state)
                .send()
                .await
                .map_err(|e| {
                    ProviderError::new(format!("Failed to create resource: {}", e))
                        .for_resource(resource.id.clone())
                })?;

            // Wait for completion
            if let Some(event) = result.progress_event()
                && let Some(token) = event.request_token()
            {
                self.wait_for_completion(token).await?;
            }

            // Read back the created resource
            let new_id = ResourceId::new(&resource.id.resource_type, identifier);
            self.read(&new_id).await
        })
    }

    fn update(
        &self,
        id: &ResourceId,
        from: &State,
        to: &Resource,
    ) -> BoxFuture<'_, ProviderResult<State>> {
        let id = id.clone();
        let from = from.clone();
        let to = to.clone();
        Box::pin(async move {
            let type_name = Self::to_cf_type_name(&id.resource_type)?;
            let patch = Self::generate_patch(&from, &to)?;

            let result = self
                .client
                .update_resource()
                .type_name(type_name)
                .identifier(&id.name)
                .patch_document(&patch)
                .send()
                .await
                .map_err(|e| {
                    ProviderError::new(format!("Failed to update resource: {:?}", e))
                        .for_resource(id.clone())
                })?;

            // Wait for completion
            if let Some(event) = result.progress_event()
                && let Some(token) = event.request_token()
            {
                self.wait_for_completion(token).await?;
            }

            // Read back the updated resource
            self.read(&id).await
        })
    }

    fn delete(&self, id: &ResourceId) -> BoxFuture<'_, ProviderResult<()>> {
        let id = id.clone();
        Box::pin(async move {
            let type_name = Self::to_cf_type_name(&id.resource_type)?;

            let result = self
                .client
                .delete_resource()
                .type_name(type_name)
                .identifier(&id.name)
                .send()
                .await
                .map_err(|e| {
                    ProviderError::new(format!("Failed to delete resource: {}", e))
                        .for_resource(id.clone())
                })?;

            // Wait for completion
            if let Some(event) = result.progress_event()
                && let Some(token) = event.request_token()
            {
                self.wait_for_completion(token).await?;
            }

            Ok(())
        })
    }
}

/// Convert DSL region format to AWS SDK format
/// "aws.Region.ap_northeast_1" -> "ap-northeast-1"
pub fn convert_region_value(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => {
            if s.starts_with("aws.Region.") {
                let region_part = s.strip_prefix("aws.Region.")?;
                Some(region_part.replace('_', "-"))
            } else {
                Some(s.clone())
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_region_value() {
        let value = Value::String("aws.Region.ap_northeast_1".to_string());
        assert_eq!(
            convert_region_value(&value),
            Some("ap-northeast-1".to_string())
        );

        let value = Value::String("us-west-2".to_string());
        assert_eq!(convert_region_value(&value), Some("us-west-2".to_string()));
    }

    #[test]
    fn test_s3_bucket_type_name() {
        let bucket_type = S3BucketType;
        assert_eq!(bucket_type.name(), "s3_bucket");
    }

    #[test]
    fn test_to_cf_type_name() {
        assert_eq!(
            AwsProvider::to_cf_type_name("s3_bucket").unwrap(),
            "AWS::S3::Bucket"
        );
        assert!(AwsProvider::to_cf_type_name("unknown").is_err());
    }

    #[test]
    fn test_value_to_json() {
        let value = Value::String("test".to_string());
        assert_eq!(
            AwsProvider::value_to_json(&value),
            serde_json::Value::String("test".to_string())
        );

        let value = Value::Int(42);
        assert_eq!(
            AwsProvider::value_to_json(&value),
            serde_json::Value::Number(42.into())
        );

        let value = Value::Bool(true);
        assert_eq!(
            AwsProvider::value_to_json(&value),
            serde_json::Value::Bool(true)
        );
    }

    #[test]
    fn test_json_to_value() {
        let json = serde_json::Value::String("test".to_string());
        assert_eq!(
            AwsProvider::json_to_value(&json),
            Some(Value::String("test".to_string()))
        );

        let json = serde_json::Value::Number(42.into());
        assert_eq!(AwsProvider::json_to_value(&json), Some(Value::Int(42)));

        let json = serde_json::Value::Bool(true);
        assert_eq!(AwsProvider::json_to_value(&json), Some(Value::Bool(true)));
    }

    #[test]
    fn test_generate_patch() {
        let from = State::existing(
            ResourceId::new("s3_bucket", "test-bucket"),
            HashMap::from([
                (
                    "BucketName".to_string(),
                    Value::String("test-bucket".to_string()),
                ),
                ("Old".to_string(), Value::String("value".to_string())),
            ]),
        );

        let to = Resource::new("s3_bucket", "test-bucket")
            .with_attribute("BucketName", Value::String("test-bucket".to_string()))
            .with_attribute("New", Value::String("value".to_string()));

        let patch = AwsProvider::generate_patch(&from, &to).unwrap();
        let patches: Vec<serde_json::Value> = serde_json::from_str(&patch).unwrap();

        // Should have add for "New" and remove for "Old"
        assert_eq!(patches.len(), 2);
    }
}
