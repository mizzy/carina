#!/bin/bash
# Fetch CloudFormation resource schemas for type generation
#
# Usage: ./scripts/fetch-cf-schemas.sh [resource-type...]
# Example: ./scripts/fetch-cf-schemas.sh AWS::S3::Bucket AWS::EC2::Instance
#
# If no resource types are provided, fetches a default set of common resources.
#
# Requirements:
# - AWS CLI configured with valid credentials
# - Or use with aws-vault: aws-vault exec <profile> -- ./scripts/fetch-cf-schemas.sh

set -euo pipefail

SCHEMA_DIR="$(dirname "$0")/../schemas"
mkdir -p "$SCHEMA_DIR"

# Default resource types to fetch
DEFAULT_TYPES=(
    "AWS::S3::Bucket"
)

# Use provided types or defaults
if [ $# -eq 0 ]; then
    TYPES=("${DEFAULT_TYPES[@]}")
else
    TYPES=("$@")
fi

for TYPE_NAME in "${TYPES[@]}"; do
    # Convert type name to filename: AWS::S3::Bucket -> AWS_S3_Bucket.json
    FILENAME="${TYPE_NAME//::/_}.json"
    OUTPUT_PATH="$SCHEMA_DIR/$FILENAME"

    echo "Fetching schema for $TYPE_NAME..."

    # Try AWS CLI first (requires credentials)
    if aws cloudformation describe-type \
        --type RESOURCE \
        --type-name "$TYPE_NAME" \
        --query 'Schema' \
        --output text > "$OUTPUT_PATH" 2>/dev/null; then
        echo "  -> Saved to $OUTPUT_PATH"
    else
        echo "  -> Failed to fetch schema for $TYPE_NAME (requires AWS credentials)" >&2
        echo "     Run with: aws-vault exec <profile> -- $0 $TYPE_NAME"
        rm -f "$OUTPUT_PATH"
    fi
done

echo "Done."
