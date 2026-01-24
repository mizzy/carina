# Carina Resource Reference

This document describes all supported resources and their attributes.

## Table of Contents

- [S3 Resources](#s3-resources)
  - [aws.s3.bucket](#awss3bucket)
- [VPC Resources](#vpc-resources)
  - [aws.vpc](#awsvpc)
  - [aws.subnet](#awssubnet)
  - [aws.internet_gateway](#awsinternet_gateway)
  - [aws.route_table](#awsroute_table)
  - [aws.security_group](#awssecurity_group)

---

## S3 Resources

### aws.s3.bucket

An S3 bucket for object storage.

#### Attributes

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | String | No | Override bucket name (defaults to resource name) |
| `region` | aws.Region | **Yes** | The AWS region for the bucket |
| `acl` | Enum | No | The canned ACL for the bucket |
| `versioning` | Bool | No | Enable versioning for the bucket |
| `expiration_days` | Int | No | Number of days before objects expire |

#### ACL Values

- `private`
- `public_read`
- `public_read_write`
- `authenticated_read`

#### Example

```crn
provider aws {
    region = aws.Region.ap_northeast_1
}

aws.s3.bucket {
    name            = "my-application-bucket"
    region          = aws.Region.ap_northeast_1
    versioning      = true
    expiration_days = 90
}
```

---

## VPC Resources

### aws.vpc

An AWS VPC (Virtual Private Cloud).

#### Attributes

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | String | **Yes** | VPC name (Name tag) |
| `region` | aws.Region | **Yes** | The AWS region for the VPC |
| `cidr_block` | CidrBlock | **Yes** | The IPv4 CIDR block for the VPC (e.g., "10.0.0.0/16") |
| `enable_dns_support` | Bool | No | Enable DNS resolution support (default: true) |
| `enable_dns_hostnames` | Bool | No | Enable DNS hostnames |

#### Example

```crn
let main_vpc = aws.vpc {
    name                 = "main-vpc"
    region               = aws.Region.ap_northeast_1
    cidr_block           = "10.0.0.0/16"
    enable_dns_support   = true
    enable_dns_hostnames = true
}
```

#### Notes

- `cidr_block` is immutable after creation

---

### aws.subnet

An AWS VPC Subnet.

#### Attributes

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | String | **Yes** | Subnet name (Name tag) |
| `region` | aws.Region | **Yes** | The AWS region for the subnet |
| `vpc` | String | **Yes** | VPC name to create the subnet in |
| `cidr_block` | CidrBlock | **Yes** | The IPv4 CIDR block for the subnet |
| `availability_zone` | String | No | The availability zone (e.g., "ap-northeast-1a") |

#### Example

```crn
let public_subnet_1a = aws.subnet {
    name              = "public-subnet-1a"
    region            = aws.Region.ap_northeast_1
    vpc               = main_vpc.name
    cidr_block        = "10.0.1.0/24"
    availability_zone = "ap-northeast-1a"
}

let public_subnet_1c = aws.subnet {
    name              = "public-subnet-1c"
    region            = aws.Region.ap_northeast_1
    vpc               = main_vpc.name
    cidr_block        = "10.0.2.0/24"
    availability_zone = "ap-northeast-1c"
}
```

#### Notes

- `cidr_block`, `vpc`, and `availability_zone` are immutable after creation

---

### aws.internet_gateway

An AWS Internet Gateway for VPC internet access.

#### Attributes

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | String | **Yes** | Internet Gateway name (Name tag) |
| `region` | aws.Region | **Yes** | The AWS region for the Internet Gateway |
| `vpc` | String | No | VPC name to attach the Internet Gateway to |

#### Example

```crn
let igw = aws.internet_gateway {
    name   = "main-igw"
    region = aws.Region.ap_northeast_1
    vpc    = main_vpc.name
}
```

---

### aws.route_table

An AWS VPC Route Table.

#### Attributes

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | String | **Yes** | Route Table name (Name tag) |
| `region` | aws.Region | **Yes** | The AWS region for the Route Table |
| `vpc` | String | **Yes** | VPC name for the Route Table |
| `routes` | List | No | List of routes |

#### Route Object

| Field | Type | Description |
|-------|------|-------------|
| `destination` | String | Destination CIDR block (e.g., "0.0.0.0/0") |
| `gateway` | String | Gateway name (Internet Gateway name) |

#### Example

```crn
let public_rt = aws.route_table {
    name   = "public-rt"
    region = aws.Region.ap_northeast_1
    vpc    = main_vpc.name
    routes = [
        { destination = "0.0.0.0/0", gateway = igw.name }
    ]
}
```

---

### aws.security_group

An AWS VPC Security Group.

#### Attributes

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | String | **Yes** | Security Group name (Name tag) |
| `region` | aws.Region | **Yes** | The AWS region for the Security Group |
| `vpc` | String | **Yes** | VPC name for the Security Group |
| `description` | String | No | Description of the Security Group |
| `ingress` | List | No | Inbound rules |
| `egress` | List | No | Outbound rules |

#### Ingress/Egress Rule Object

| Field | Type | Description |
|-------|------|-------------|
| `protocol` | String | Protocol: "tcp", "udp", "icmp", or "-1" (all) |
| `from_port` | Int | Start of port range (0-65535) |
| `to_port` | Int | End of port range (0-65535) |
| `cidr` | String | CIDR block to allow (e.g., "0.0.0.0/0") |

#### Example (Terraform-style blocks)

```crn
let web_sg = aws.security_group {
    name        = "web-sg"
    region      = aws.Region.ap_northeast_1
    vpc         = main_vpc.name
    description = "Web server security group"

    ingress {
        protocol  = "tcp"
        from_port = 80
        to_port   = 80
        cidr      = "0.0.0.0/0"
    }

    ingress {
        protocol  = "tcp"
        from_port = 443
        to_port   = 443
        cidr      = "0.0.0.0/0"
    }

    ingress {
        protocol  = "tcp"
        from_port = 22
        to_port   = 22
        cidr      = "10.0.0.0/8"
    }

    egress {
        protocol  = "-1"
        from_port = 0
        to_port   = 0
        cidr      = "0.0.0.0/0"
    }
}
```

#### Example (List syntax)

```crn
let web_sg = aws.security_group {
    name        = "web-sg"
    region      = aws.Region.ap_northeast_1
    vpc         = main_vpc.name
    description = "Web server security group"
    ingress = [
        { protocol = "tcp", from_port = 80,  to_port = 80,  cidr = "0.0.0.0/0" },
        { protocol = "tcp", from_port = 443, to_port = 443, cidr = "0.0.0.0/0" }
    ]
    egress = [
        { protocol = "-1", from_port = 0, to_port = 0, cidr = "0.0.0.0/0" }
    ]
}
```

---

## AWS Regions

The `aws.Region` type supports the following values:

| DSL Value | AWS Region |
|-----------|------------|
| `aws.Region.ap_northeast_1` | ap-northeast-1 (Tokyo) |
| `aws.Region.ap_northeast_2` | ap-northeast-2 (Seoul) |
| `aws.Region.ap_northeast_3` | ap-northeast-3 (Osaka) |
| `aws.Region.ap_southeast_1` | ap-southeast-1 (Singapore) |
| `aws.Region.ap_southeast_2` | ap-southeast-2 (Sydney) |
| `aws.Region.ap_south_1` | ap-south-1 (Mumbai) |
| `aws.Region.us_east_1` | us-east-1 (N. Virginia) |
| `aws.Region.us_east_2` | us-east-2 (Ohio) |
| `aws.Region.us_west_1` | us-west-1 (N. California) |
| `aws.Region.us_west_2` | us-west-2 (Oregon) |
| `aws.Region.eu_west_1` | eu-west-1 (Ireland) |
| `aws.Region.eu_west_2` | eu-west-2 (London) |
| `aws.Region.eu_central_1` | eu-central-1 (Frankfurt) |

---

## Complete Example

```crn
provider aws {
    region = aws.Region.ap_northeast_1
}

// VPC
let main_vpc = aws.vpc {
    name                 = "production-vpc"
    region               = aws.Region.ap_northeast_1
    cidr_block           = "10.0.0.0/16"
    enable_dns_support   = true
    enable_dns_hostnames = true
}

// Subnets
let public_subnet_1a = aws.subnet {
    name              = "public-subnet-1a"
    region            = aws.Region.ap_northeast_1
    vpc               = main_vpc.name
    cidr_block        = "10.0.1.0/24"
    availability_zone = "ap-northeast-1a"
}

let public_subnet_1c = aws.subnet {
    name              = "public-subnet-1c"
    region            = aws.Region.ap_northeast_1
    vpc               = main_vpc.name
    cidr_block        = "10.0.2.0/24"
    availability_zone = "ap-northeast-1c"
}

let private_subnet_1a = aws.subnet {
    name              = "private-subnet-1a"
    region            = aws.Region.ap_northeast_1
    vpc               = main_vpc.name
    cidr_block        = "10.0.10.0/24"
    availability_zone = "ap-northeast-1a"
}

// Internet Gateway
let igw = aws.internet_gateway {
    name   = "production-igw"
    region = aws.Region.ap_northeast_1
    vpc    = main_vpc.name
}

// Route Table for public subnets
let public_rt = aws.route_table {
    name   = "public-rt"
    region = aws.Region.ap_northeast_1
    vpc    = main_vpc.name
    routes = [
        { destination = "0.0.0.0/0", gateway = igw.name }
    ]
}

// Security Groups
let web_sg = aws.security_group {
    name        = "web-sg"
    region      = aws.Region.ap_northeast_1
    vpc         = main_vpc.name
    description = "Web server security group"

    ingress {
        protocol  = "tcp"
        from_port = 80
        to_port   = 80
        cidr      = "0.0.0.0/0"
    }

    ingress {
        protocol  = "tcp"
        from_port = 443
        to_port   = 443
        cidr      = "0.0.0.0/0"
    }

    egress {
        protocol  = "-1"
        from_port = 0
        to_port   = 0
        cidr      = "0.0.0.0/0"
    }
}

let db_sg = aws.security_group {
    name        = "db-sg"
    region      = aws.Region.ap_northeast_1
    vpc         = main_vpc.name
    description = "Database security group"

    ingress {
        protocol  = "tcp"
        from_port = 3306
        to_port   = 3306
        cidr      = "10.0.0.0/16"
    }

    egress {
        protocol  = "-1"
        from_port = 0
        to_port   = 0
        cidr      = "0.0.0.0/0"
    }
}

// S3 Bucket
aws.s3.bucket {
    name       = "production-assets-bucket"
    region     = aws.Region.ap_northeast_1
    versioning = true
}
```
