provider "aws" {
  region                  = local.region
  shared_credentials_file = local.aws_credentials_path
  profile                 = local.aws_profile

  default_tags {
    tags = {
      Phase     = "Test"
      Owner     = "Donghyung Ko"
      Workspace = "BlueWhale/onboarding-projects/project-B"
    }
  }
}

provider "http" {

}
