provider "aws" {
  region                  = "sa-east-1" // 남아메리카 상파울로
  shared_credentials_file = var.aws_credentials_path
  profile                 = var.aws_profile

  default_tags {
    tags = {
      Phase     = "Test"
      Owner     = "Donghyung Ko"
      Workspace = "BlueWhale/onboarding-projects/project-B"
    }
  }
}
