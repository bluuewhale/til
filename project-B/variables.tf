// VPC
variable "vpc_name" {
  type    = string
  default = "vpc-main"
}

variable "region" {
  type    = string
  default = "sa-east-1" // 상파울로
}


variable "public_subnets" {
  type    = list(string)
  default = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
}

variable "private_subnets" {
  type    = list(string)
  default = ["10.0.100.0/24", "10.0.101.0/24", "10.0.102.0/24"]
}

// aws provider
variable "aws_credentials_path" {
  type        = string
  description = "path to AWS credential configuration file"
  default     = "~/.aws/credentials"
}

variable "aws_profile" {
  type        = string
  description = "AWS profile configuration. Refers to AWS_PROFILE"
  default     = "mfa"
}

variable "region" {
  type    = string
  default = "sa-east-1" // 상파울로
}


locals {
  cidr_blocks_sucho = ["103.114.125.1/32", "103.114.125.2/32"]
}
