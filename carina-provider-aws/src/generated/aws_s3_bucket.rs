// Auto-generated from ../schemas/AWS_S3_Bucket.json - do not edit
#![allow(dead_code, unused_imports, clippy::all)]

use serde::{Deserialize, Serialize};

/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`AccelerateConfiguration`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "AccelerationStatus"
///  ],
///  "properties": {
///    "AccelerationStatus": {
///      "description": "Specifies the transfer acceleration status of the bucket.",
///      "type": "string",
///      "enum": [
///        "Enabled",
///        "Suspended"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AccelerateConfiguration {
    ///Specifies the transfer acceleration status of the bucket.
    #[serde(rename = "AccelerationStatus")]
    pub acceleration_status: AccelerateConfigurationAccelerationStatus,
}
impl ::std::convert::From<&AccelerateConfiguration> for AccelerateConfiguration {
    fn from(value: &AccelerateConfiguration) -> Self {
        value.clone()
    }
}
impl AccelerateConfiguration {
    pub fn builder() -> builder::AccelerateConfiguration {
        Default::default()
    }
}
///Specifies the transfer acceleration status of the bucket.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies the transfer acceleration status of the bucket.",
///  "type": "string",
///  "enum": [
///    "Enabled",
///    "Suspended"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AccelerateConfigurationAccelerationStatus {
    Enabled,
    Suspended,
}
impl ::std::convert::From<&Self> for AccelerateConfigurationAccelerationStatus {
    fn from(value: &AccelerateConfigurationAccelerationStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccelerateConfigurationAccelerationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Enabled => f.write_str("Enabled"),
            Self::Suspended => f.write_str("Suspended"),
        }
    }
}
impl ::std::str::FromStr for AccelerateConfigurationAccelerationStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Enabled" => Ok(Self::Enabled),
            "Suspended" => Ok(Self::Suspended),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccelerateConfigurationAccelerationStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccelerateConfigurationAccelerationStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccelerateConfigurationAccelerationStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`LifecycleConfiguration`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "Rules"
///  ],
///  "properties": {
///    "Rules": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Rule"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub rules: ::std::vec::Vec<Rule>,
}
impl ::std::convert::From<&LifecycleConfiguration> for LifecycleConfiguration {
    fn from(value: &LifecycleConfiguration) -> Self {
        value.clone()
    }
}
impl LifecycleConfiguration {
    pub fn builder() -> builder::LifecycleConfiguration {
        Default::default()
    }
}
///`PublicAccessBlockConfiguration`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "BlockPublicAcls": {
///      "type": "boolean"
///    },
///    "BlockPublicPolicy": {
///      "type": "boolean"
///    },
///    "IgnorePublicAcls": {
///      "type": "boolean"
///    },
///    "RestrictPublicBuckets": {
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PublicAccessBlockConfiguration {
    #[serde(
        rename = "BlockPublicAcls",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub block_public_acls: ::std::option::Option<bool>,
    #[serde(
        rename = "BlockPublicPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub block_public_policy: ::std::option::Option<bool>,
    #[serde(
        rename = "IgnorePublicAcls",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ignore_public_acls: ::std::option::Option<bool>,
    #[serde(
        rename = "RestrictPublicBuckets",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub restrict_public_buckets: ::std::option::Option<bool>,
}
impl ::std::convert::From<&PublicAccessBlockConfiguration> for PublicAccessBlockConfiguration {
    fn from(value: &PublicAccessBlockConfiguration) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PublicAccessBlockConfiguration {
    fn default() -> Self {
        Self {
            block_public_acls: Default::default(),
            block_public_policy: Default::default(),
            ignore_public_acls: Default::default(),
            restrict_public_buckets: Default::default(),
        }
    }
}
impl PublicAccessBlockConfiguration {
    pub fn builder() -> builder::PublicAccessBlockConfiguration {
        Default::default()
    }
}
///`Rule`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "Status"
///  ],
///  "properties": {
///    "ExpirationInDays": {
///      "type": "integer",
///      "minimum": 1.0
///    },
///    "Id": {
///      "type": "string"
///    },
///    "Prefix": {
///      "type": "string"
///    },
///    "Status": {
///      "type": "string",
///      "enum": [
///        "Enabled",
///        "Disabled"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    #[serde(
        rename = "ExpirationInDays",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expiration_in_days: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "Prefix",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Status")]
    pub status: RuleStatus,
}
impl ::std::convert::From<&Rule> for Rule {
    fn from(value: &Rule) -> Self {
        value.clone()
    }
}
impl Rule {
    pub fn builder() -> builder::Rule {
        Default::default()
    }
}
///`RuleStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "Enabled",
///    "Disabled"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RuleStatus {
    Enabled,
    Disabled,
}
impl ::std::convert::From<&Self> for RuleStatus {
    fn from(value: &RuleStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RuleStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Enabled => f.write_str("Enabled"),
            Self::Disabled => f.write_str("Disabled"),
        }
    }
}
impl ::std::str::FromStr for RuleStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RuleStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuleStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuleStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`Tag`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "Key",
///    "Value"
///  ],
///  "properties": {
///    "Key": {
///      "type": "string",
///      "maxLength": 128,
///      "minLength": 1
///    },
///    "Value": {
///      "type": "string",
///      "maxLength": 256
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: TagKey,
    #[serde(rename = "Value")]
    pub value: TagValue,
}
impl ::std::convert::From<&Tag> for Tag {
    fn from(value: &Tag) -> Self {
        value.clone()
    }
}
impl Tag {
    pub fn builder() -> builder::Tag {
        Default::default()
    }
}
///`TagKey`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 128,
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TagKey(::std::string::String);
impl ::std::ops::Deref for TagKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TagKey> for ::std::string::String {
    fn from(value: TagKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TagKey> for TagKey {
    fn from(value: &TagKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TagKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TagKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TagKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TagKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TagKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`TagValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "maxLength": 256
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TagValue(::std::string::String);
impl ::std::ops::Deref for TagValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TagValue> for ::std::string::String {
    fn from(value: TagValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TagValue> for TagValue {
    fn from(value: &TagValue) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TagValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TagValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TagValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TagValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TagValue {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`VersioningConfiguration`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "Status"
///  ],
///  "properties": {
///    "Status": {
///      "type": "string",
///      "enum": [
///        "Enabled",
///        "Suspended"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VersioningConfiguration {
    #[serde(rename = "Status")]
    pub status: VersioningConfigurationStatus,
}
impl ::std::convert::From<&VersioningConfiguration> for VersioningConfiguration {
    fn from(value: &VersioningConfiguration) -> Self {
        value.clone()
    }
}
impl VersioningConfiguration {
    pub fn builder() -> builder::VersioningConfiguration {
        Default::default()
    }
}
///`VersioningConfigurationStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "Enabled",
///    "Suspended"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VersioningConfigurationStatus {
    Enabled,
    Suspended,
}
impl ::std::convert::From<&Self> for VersioningConfigurationStatus {
    fn from(value: &VersioningConfigurationStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for VersioningConfigurationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Enabled => f.write_str("Enabled"),
            Self::Suspended => f.write_str("Suspended"),
        }
    }
}
impl ::std::str::FromStr for VersioningConfigurationStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Enabled" => Ok(Self::Enabled),
            "Suspended" => Ok(Self::Suspended),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VersioningConfigurationStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VersioningConfigurationStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VersioningConfigurationStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AccelerateConfiguration {
        acceleration_status: ::std::result::Result<
            super::AccelerateConfigurationAccelerationStatus,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AccelerateConfiguration {
        fn default() -> Self {
            Self {
                acceleration_status: Err("no value supplied for acceleration_status".to_string()),
            }
        }
    }
    impl AccelerateConfiguration {
        pub fn acceleration_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccelerateConfigurationAccelerationStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.acceleration_status = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for acceleration_status: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AccelerateConfiguration> for super::AccelerateConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AccelerateConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acceleration_status: value.acceleration_status?,
            })
        }
    }
    impl ::std::convert::From<super::AccelerateConfiguration> for AccelerateConfiguration {
        fn from(value: super::AccelerateConfiguration) -> Self {
            Self {
                acceleration_status: Ok(value.acceleration_status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LifecycleConfiguration {
        rules: ::std::result::Result<::std::vec::Vec<super::Rule>, ::std::string::String>,
    }
    impl ::std::default::Default for LifecycleConfiguration {
        fn default() -> Self {
            Self {
                rules: Err("no value supplied for rules".to_string()),
            }
        }
    }
    impl LifecycleConfiguration {
        pub fn rules<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Rule>>,
            T::Error: ::std::fmt::Display,
        {
            self.rules = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rules: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LifecycleConfiguration> for super::LifecycleConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LifecycleConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                rules: value.rules?,
            })
        }
    }
    impl ::std::convert::From<super::LifecycleConfiguration> for LifecycleConfiguration {
        fn from(value: super::LifecycleConfiguration) -> Self {
            Self {
                rules: Ok(value.rules),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PublicAccessBlockConfiguration {
        block_public_acls:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        block_public_policy:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        ignore_public_acls:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        restrict_public_buckets:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for PublicAccessBlockConfiguration {
        fn default() -> Self {
            Self {
                block_public_acls: Ok(Default::default()),
                block_public_policy: Ok(Default::default()),
                ignore_public_acls: Ok(Default::default()),
                restrict_public_buckets: Ok(Default::default()),
            }
        }
    }
    impl PublicAccessBlockConfiguration {
        pub fn block_public_acls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.block_public_acls = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for block_public_acls: {}",
                    e
                )
            });
            self
        }
        pub fn block_public_policy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.block_public_policy = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for block_public_policy: {}",
                    e
                )
            });
            self
        }
        pub fn ignore_public_acls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.ignore_public_acls = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for ignore_public_acls: {}",
                    e
                )
            });
            self
        }
        pub fn restrict_public_buckets<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.restrict_public_buckets = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for restrict_public_buckets: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<PublicAccessBlockConfiguration>
        for super::PublicAccessBlockConfiguration
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PublicAccessBlockConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                block_public_acls: value.block_public_acls?,
                block_public_policy: value.block_public_policy?,
                ignore_public_acls: value.ignore_public_acls?,
                restrict_public_buckets: value.restrict_public_buckets?,
            })
        }
    }
    impl ::std::convert::From<super::PublicAccessBlockConfiguration>
        for PublicAccessBlockConfiguration
    {
        fn from(value: super::PublicAccessBlockConfiguration) -> Self {
            Self {
                block_public_acls: Ok(value.block_public_acls),
                block_public_policy: Ok(value.block_public_policy),
                ignore_public_acls: Ok(value.ignore_public_acls),
                restrict_public_buckets: Ok(value.restrict_public_buckets),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Rule {
        expiration_in_days: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prefix: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::RuleStatus, ::std::string::String>,
    }
    impl ::std::default::Default for Rule {
        fn default() -> Self {
            Self {
                expiration_in_days: Ok(Default::default()),
                id: Ok(Default::default()),
                prefix: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl Rule {
        pub fn expiration_in_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.expiration_in_days = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for expiration_in_days: {}",
                    e
                )
            });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn prefix<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prefix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prefix: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RuleStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Rule> for super::Rule {
        type Error = super::error::ConversionError;
        fn try_from(value: Rule) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expiration_in_days: value.expiration_in_days?,
                id: value.id?,
                prefix: value.prefix?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::Rule> for Rule {
        fn from(value: super::Rule) -> Self {
            Self {
                expiration_in_days: Ok(value.expiration_in_days),
                id: Ok(value.id),
                prefix: Ok(value.prefix),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tag {
        key: ::std::result::Result<super::TagKey, ::std::string::String>,
        value: ::std::result::Result<super::TagValue, ::std::string::String>,
    }
    impl ::std::default::Default for Tag {
        fn default() -> Self {
            Self {
                key: Err("no value supplied for key".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Tag {
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TagKey>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TagValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Tag> for super::Tag {
        type Error = super::error::ConversionError;
        fn try_from(value: Tag) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                key: value.key?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::Tag> for Tag {
        fn from(value: super::Tag) -> Self {
            Self {
                key: Ok(value.key),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VersioningConfiguration {
        status: ::std::result::Result<super::VersioningConfigurationStatus, ::std::string::String>,
    }
    impl ::std::default::Default for VersioningConfiguration {
        fn default() -> Self {
            Self {
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl VersioningConfiguration {
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VersioningConfigurationStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<VersioningConfiguration> for super::VersioningConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VersioningConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::VersioningConfiguration> for VersioningConfiguration {
        fn from(value: super::VersioningConfiguration) -> Self {
            Self {
                status: Ok(value.status),
            }
        }
    }
}
