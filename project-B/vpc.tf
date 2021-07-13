data "aws_availability_zones" "az" {}

module "vpc" {
  source = "../module/vpc"

  name = "main-vpc"
  cidr = "10.0.0.0/16"

  azs = [
    data.aws_availability_zones.az.names[0],
    data.aws_availability_zones.az.names[1],
    data.aws_availability_zones.az.names[2]
  ]

  public_subnets  = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  private_subnets = ["10.0.100.0/24", "10.0.101.0/24", "10.0.102.0/24"]

  enable_nat_gateway = true


  tags = {
    Environment = "Test"
    Project     = "onboarding-project-B"
    Owner       = "DonghyungKo"
  }
}
