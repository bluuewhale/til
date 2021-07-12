// VPC
resource "aws_vpc" "main_vpc" {
  cidr_block       = "10.0.0.0/16"
  instance_tenancy = "default"
}

// Internet Gateway
resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.main_vpc.id
}

// AZ
data "aws_availability_zones" "available" {}

// Public Subnets
resource "aws_subnet" "public_subnet01" {
  vpc_id            = aws_vpc.main_vpc.id
  cidr_block        = "10.0.1.0/24"
  availability_zone = data.aws_availability_zones.available.names[0]

  map_public_ip_on_launch = true

  depends_on = [
    aws_internet_gateway.igw
  ]
}

resource "aws_subnet" "public_subnet02" {
  vpc_id            = aws_vpc.main_vpc.id
  cidr_block        = "10.0.2.0/24"
  availability_zone = data.aws_availability_zones.available.names[1]

  map_public_ip_on_launch = true

  depends_on = [
    aws_internet_gateway.igw
  ]
}

// Private Subnets
resource "aws_subnet" "private_subnet01" {
  vpc_id            = aws_vpc.main_vpc.id
  cidr_block        = "10.0.100.0/24"
  availability_zone = data.aws_availability_zones.available.names[0]
}

resource "aws_subnet" "private_subnet02" {
  vpc_id            = aws_vpc.main_vpc.id
  cidr_block        = "10.0.101.0/24"
  availability_zone = data.aws_availability_zones.available.names[1]
}

// NACL (allow all traffic in every subnet)
resource "aws_network_acl" "allow_all" {
  vpc_id = aws_vpc.main_vpc.id
  subnet_ids = [
    aws_subnet.public_subnet01.id,
    aws_subnet.public_subnet02.id,
    aws_subnet.private_subnet01.id,
    aws_subnet.private_subnet02.id,
  ]

  // allow all ingress traffic
  ingress {
    rule_no = 100
    action  = "allow"

    protocol   = "-1"
    from_port  = 0
    to_port    = 0
    cidr_block = "0.0.0.0/0"
  }

  // allow all egress traffic
  egress {
    rule_no = 200
    action  = "allow"

    protocol   = "-1"
    from_port  = 0
    to_port    = 0
    cidr_block = "0.0.0.0/0"

  }
}

// NAT Gateway
resource "aws_eip" "ngw" {
  vpc = true
}
resource "aws_nat_gateway" "ngw" {
  allocation_id = aws_eip.ngw.id
  subnet_id     = aws_subnet.public_subnet01.id
}

// Route Tables
// [NOTE] on Route Tables and Routes:
// Terraform currently provides both a standalone Route resource and a Route Table 
// resource with routes defined in-line. At this time you cannot use a Route Table 
// with in-line routes in conjunction with any Route resources. 
// Doing so will cause a conflict of rule settings and will overwrite rules.
resource "aws_route_table" "public" {
  vpc_id = aws_vpc.main_vpc.id
}

resource "aws_route_table" "private" {
  vpc_id = aws_vpc.main_vpc.id
}

// Route
resource "aws_route" "public" {
  route_table_id         = aws_route_table.public.id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.igw.id
}

resource "aws_route" "private" {
  route_table_id         = aws_route_table.private.id
  destination_cidr_block = "0.0.0.0/0"
  nat_gateway_id         = aws_nat_gateway.ngw.id
}

// Route Table Associations
resource "aws_route_table_association" "public_subnet01" {
  subnet_id      = aws_subnet.public_subnet01.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "public_subnet02" {
  subnet_id      = aws_subnet.public_subnet02.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "private_subnet01" {
  subnet_id      = aws_subnet.private_subnet01.id
  route_table_id = aws_route_table.private.id
}

resource "aws_route_table_association" "private_subnet02" {
  subnet_id      = aws_subnet.private_subnet02.id
  route_table_id = aws_route_table.private.id
}

