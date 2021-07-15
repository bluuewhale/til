terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.0"
    }
    kubernetes = "~> 1.11"
  }

  backend "s3" {
    key    = "webapp.tfstate"
    bucket = "onboarding-project-b-donghyungko"
    region = "sa-east-1"
  }
}
