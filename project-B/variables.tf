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

// aws cluster
variable "cluster_version" {
  type = string
}

variable "cluster_name" {
  type    = string
  default = "onboarding-project-B"
}

variable "cluster_suffix" {
  type    = string
  default = ""
}

variable "kubernetes_service_ipv4_cidr" {
  type    = string
  default = "172.20.0.0/16"
}


// AMI
variable "worker_ami_name_filter" {
  type    = string
  default = ""
}

variable "worker_ami_owner_id" {
  description = "The ID of the owner for the AMI to use for the AWS EKS Worker"
  type        = string
  default     = "amazon"
}


// aws iam
variable "cluster_iam_role_name" {
  type    = string
  default = ""
}

variable "iam_path" {
  type    = string
  default = "/"
}

variable "iam_permissions_boundary" {
  type    = string
  default = null
}
