#!/bin/bash
# Generate awscc provider schemas from CloudFormation
#
# Usage:
#   aws-vault exec <profile> -- ./scripts/generate-awscc-schemas.sh
#
# This script generates Rust schema code from CloudFormation resource type schemas.

set -e

OUTPUT_DIR="carina-provider-awscc/src/schemas/generated"
mkdir -p "$OUTPUT_DIR"

# List of resource types to generate
RESOURCE_TYPES=(
    "AWS::EC2::VPC"
    "AWS::EC2::Subnet"
    "AWS::EC2::InternetGateway"
    "AWS::EC2::RouteTable"
    "AWS::EC2::Route"
    "AWS::EC2::SubnetRouteTableAssociation"
    "AWS::EC2::EIP"
    "AWS::EC2::NatGateway"
    "AWS::EC2::SecurityGroup"
    "AWS::EC2::VPCEndpoint"
)

echo "Generating awscc provider schemas..."
echo "Output directory: $OUTPUT_DIR"
echo ""

# Build codegen tool first
cargo build -p carina-codegen --quiet

for TYPE_NAME in "${RESOURCE_TYPES[@]}"; do
    # Convert AWS::EC2::VPC -> vpc.rs
    FILENAME=$(echo "$TYPE_NAME" | sed 's/AWS::EC2:://' | tr '[:upper:]' '[:lower:]')
    # Handle special cases
    FILENAME=$(echo "$FILENAME" | sed 's/subnetroutetableassociation/route_table_association/')
    FILENAME=$(echo "$FILENAME" | sed 's/vpcendpoint/vpc_endpoint/')
    FILENAME=$(echo "$FILENAME" | sed 's/natgateway/nat_gateway/')
    FILENAME=$(echo "$FILENAME" | sed 's/internetgateway/internet_gateway/')
    FILENAME=$(echo "$FILENAME" | sed 's/routetable/route_table/')
    FILENAME=$(echo "$FILENAME" | sed 's/securitygroup/security_group/')

    OUTPUT_FILE="$OUTPUT_DIR/${FILENAME}.rs"

    echo "Generating $TYPE_NAME -> $OUTPUT_FILE"

    aws cloudformation describe-type \
        --type RESOURCE \
        --type-name "$TYPE_NAME" \
        --query 'Schema' \
        --output text 2>/dev/null | \
    cargo run -p carina-codegen --quiet -- --type-name "$TYPE_NAME" > "$OUTPUT_FILE"

    if [ $? -ne 0 ]; then
        echo "  ERROR: Failed to generate $TYPE_NAME"
        rm -f "$OUTPUT_FILE"
    fi
done

# Generate mod.rs
echo ""
echo "Generating $OUTPUT_DIR/mod.rs"

cat > "$OUTPUT_DIR/mod.rs" << 'EOF'
//! Auto-generated AWS Cloud Control resource schemas
//!
//! DO NOT EDIT MANUALLY - regenerate with:
//!   aws-vault exec <profile> -- ./scripts/generate-awscc-schemas.sh

use carina_core::schema::{AttributeType, ResourceSchema};

/// Tags type for AWS resources (Terraform-style map)
pub fn tags_type() -> AttributeType {
    AttributeType::Map(Box::new(AttributeType::String))
}

EOF

# Add module declarations
for TYPE_NAME in "${RESOURCE_TYPES[@]}"; do
    MODNAME=$(echo "$TYPE_NAME" | sed 's/AWS::EC2:://' | tr '[:upper:]' '[:lower:]')
    MODNAME=$(echo "$MODNAME" | sed 's/subnetroutetableassociation/route_table_association/')
    MODNAME=$(echo "$MODNAME" | sed 's/vpcendpoint/vpc_endpoint/')
    MODNAME=$(echo "$MODNAME" | sed 's/natgateway/nat_gateway/')
    MODNAME=$(echo "$MODNAME" | sed 's/internetgateway/internet_gateway/')
    MODNAME=$(echo "$MODNAME" | sed 's/routetable/route_table/')
    MODNAME=$(echo "$MODNAME" | sed 's/securitygroup/security_group/')

    echo "pub mod ${MODNAME};" >> "$OUTPUT_DIR/mod.rs"
done

# Add schemas() function
cat >> "$OUTPUT_DIR/mod.rs" << 'EOF'

/// Returns all generated schemas
pub fn schemas() -> Vec<ResourceSchema> {
    vec![
        vpc::vpc_schema(),
        subnet::subnet_schema(),
        internet_gateway::internet_gateway_schema(),
        route_table::route_table_schema(),
        route::route_schema(),
        route_table_association::subnet_route_table_association_schema(),
        eip::eip_schema(),
        nat_gateway::nat_gateway_schema(),
        security_group::security_group_schema(),
        vpc_endpoint::vpc_endpoint_schema(),
    ]
}
EOF

echo ""
echo "Done! Generated schemas in $OUTPUT_DIR"
echo ""
echo "To use the generated schemas, update carina-provider-awscc/src/schemas/mod.rs"
