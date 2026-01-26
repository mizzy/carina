# Carina

> [!CAUTION]
> This is an experimental project. The DSL syntax, APIs, and features are subject to change without notice.

A functional infrastructure management tool written in Rust. Carina treats infrastructure changes as values (Effects) rather than immediately executing side effects, enabling safer and more predictable infrastructure management.

## Features

- **Custom DSL**: Simple, expressive syntax for defining infrastructure
- **Effects as Values**: Side effects are represented as data structures, not immediately executed
- **Strong Typing**: Catch configuration errors at parse time with schema validation
- **Provider Architecture**: Extensible provider system for multi-cloud support
- **Modules**: Reusable infrastructure components with typed inputs/outputs
- **Terraform-like Workflow**: Familiar `validate`, `plan`, `apply`, `destroy` commands

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/carina`.

## Quick Start

### 1. Define your infrastructure

Create a `.crn` file:

```
# example.crn

provider aws {
    region = aws.Region.ap_northeast_1
}

# Anonymous resource (ID derived from name attribute)
aws.s3.bucket {
    name = "my-app-logs"
    region = aws.Region.ap_northeast_1
    versioning = true
    expiration_days = 90
}

# Named resource (for referencing)
let backup = aws.s3.bucket {
    name = "my-app-backup"
    region = aws.Region.ap_northeast_1
}
```

### 2. Validate

```bash
$ carina validate example.crn
Validating...
✓ 2 resources validated successfully.
  • s3_bucket.my-app-logs
  • s3_bucket.my-app-backup
```

### 3. Plan

```bash
$ carina plan example.crn
Execution Plan:

  + s3_bucket
      name: "my-app-logs"
      expiration_days: 90
      region: "ap-northeast-1"
      versioning: true
  + s3_bucket
      name: "my-app-backup"
      region: "ap-northeast-1"

Plan: 2 to add, 0 to change, 0 to destroy.
```

Resources with dependencies are displayed as a tree:

```
  + vpc
      name: "main-vpc"
      cidr_block: "10.0.0.0/16"
        └─ + security_group
              name: "web-sg"
              vpc_id: main_vpc.id
              └─ + security_group.ingress_rule
                    name: "http"
                    security_group_id: web_sg.id
```

### 4. Apply

```bash
$ carina apply example.crn
Using AWS provider (region: ap-northeast-1)
Applying changes...

  ✓ Create s3_bucket.my-app-logs
  ✓ Create s3_bucket.my-app-backup

Apply complete! 2 changes applied.
```

## DSL Syntax

### Provider Block

```
provider aws {
    region = aws.Region.ap_northeast_1
}
```

### Resources

**Anonymous resources** - ID is derived from the `name` attribute:

```
aws.s3.bucket {
    name = "my-bucket"
    region = aws.Region.ap_northeast_1
    versioning = true
}
```

**Named resources** - Use `let` binding for referencing:

```
let logs = aws.s3.bucket {
    name = "my-logs"
    region = aws.Region.ap_northeast_1
}
```

### Modules

Modules enable reusable infrastructure components with typed inputs and outputs.

**Module definition** (`modules/web_tier/main.crn`):

```
input {
    vpc: ref(aws.vpc)
    cidr_blocks: list(cidr)
    enable_https: bool = true
}

output {
    security_group: ref(aws.security_group) = web_sg.id
}

let web_sg = aws.security_group {
    name        = "web-sg"
    vpc_id      = input.vpc
    description = "Security group for web servers"
}
```

**Using modules**:

```
import "./modules/web_tier" as web_tier

let main_vpc = aws.vpc {
    name       = "main-vpc"
    cidr_block = "10.0.0.0/16"
}

web_tier {
    vpc         = main_vpc.id
    cidr_blocks = ["10.0.1.0/24", "10.0.2.0/25"]
}
```

**Inspect module structure**:

```bash
$ carina module info modules/web_tier
Module: web_tier

=== REQUIRES ===

  vpc: ref(aws.vpc)  (required)
  cidr_blocks: list(cidr)  (required)
  enable_https: bool = true

=== CREATES ===

  input { vpc: ref(aws.vpc) }
    └── web_sg: aws.security_group
       ├── http: aws.security_group.ingress_rule
       └── https: aws.security_group.ingress_rule

=== EXPOSES ===

  security_group: ref(aws.security_group)
    <- from: web_sg
```

## Architecture

Carina follows a functional architecture where side effects are treated as values:

```
DSL File (.crn)
     │
     ▼
┌─────────┐
│ Parser  │  Parse DSL into Resources
└────┬────┘
     │
     ▼
┌─────────┐
│ Differ  │  Compare desired vs current state
└────┬────┘
     │
     ▼
┌─────────┐
│  Plan   │  Collection of Effects (Create/Update/Delete)
└────┬────┘
     │
     ▼
┌─────────────┐
│ Interpreter │  Execute Effects through Provider
└──────┬──────┘
       │
       ▼
┌──────────┐
│ Provider │  AWS, GCP, etc.
└──────────┘
```

### Core Concepts

- **Resource**: Desired state declared in DSL
- **State**: Current state fetched from infrastructure
- **Effect**: Represents a side effect (Create, Update, Delete, Read)
- **Plan**: Collection of Effects to be executed
- **Provider**: Abstraction for infrastructure operations
- **Interpreter**: Executes Plan through Provider

## Project Structure

```
carina/
├── carina-cli/          # CLI application
├── carina-core/         # Core library
│   ├── src/
│   │   ├── effect.rs    # Effect type definitions
│   │   ├── plan.rs      # Plan (collection of Effects)
│   │   ├── resource.rs  # Resource and State types
│   │   ├── provider.rs  # Provider trait
│   │   ├── interpreter.rs # Effect interpreter
│   │   ├── differ.rs    # State comparison
│   │   ├── parser/      # DSL parser (pest-based)
│   │   ├── schema.rs    # Type validation
│   │   └── providers/   # Built-in provider schemas
│   └── ...
└── carina-provider-aws/ # AWS provider implementation
```

## AWS Provider

The AWS provider requires valid AWS credentials. Configure via:

- Environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
- AWS credentials file (`~/.aws/credentials`)
- IAM roles (when running on AWS)

### Using with aws-vault

```bash
aws-vault exec myprofile -- carina apply example.crn
```

## Commands

### Format

Format `.crn` files:

```bash
# Format a single file
$ carina fmt example.crn

# Format all .crn files in current directory
$ carina fmt

# Format recursively
$ carina fmt -r

# Check formatting without modifying files
$ carina fmt --check

# Show diff of formatting changes
$ carina fmt --diff
```

### Destroy

Remove all resources defined in a configuration:

```bash
$ carina destroy example.crn
Destroy Plan:

  - s3_bucket.my-app-logs
  - s3_bucket.my-app-backup

Plan: 2 to destroy.

Do you really want to destroy all resources?
  This action cannot be undone. Type 'yes' to confirm.

  Enter a value: yes

Destroying resources...

  ✓ Delete s3_bucket.my-app-logs
  ✓ Delete s3_bucket.my-app-backup

Destroy complete! 2 resources destroyed.
```

Use `--auto-approve` to skip the confirmation prompt.

### Module Info

Inspect module structure and dependencies:

```bash
$ carina module info modules/web_tier
```

## Development

### Run tests

```bash
cargo test
```

### Build

```bash
cargo build
```

## License

MIT

## Roadmap

- [x] Resource dependencies and references
- [x] Modules and reusability
- [x] Destroy command
- [ ] More AWS resources (EC2, IAM, Lambda, etc.)
- [ ] GCP provider
- [ ] State file management
- [ ] Import existing resources
