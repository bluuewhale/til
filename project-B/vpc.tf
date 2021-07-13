data "aws_availability_zones" "az" {}

module "vpc" {
  source = "../module/vpc"

  name = var.vpc_name
  cidr = "10.0.0.0/16"

  azs = [
    data.aws_availability_zones.az.names[0],
    data.aws_availability_zones.az.names[1],
    data.aws_availability_zones.az.names[2]
  ]

  public_subnets  = var.public_subnets
  private_subnets = var.private_subnets

  enable_nat_gateway = true // NAT Gateway per public subnets

  tags = {
    Phase     = "Test"
    Owner     = "Donghyung Ko"
    Workspace = "BlueWhale/onboarding-projects/project-B"
  }
}
