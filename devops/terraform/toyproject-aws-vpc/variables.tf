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

locals {
  cidr_blocks_sucho = ["103.114.125.1/32", "103.114.125.2/32"]
}
