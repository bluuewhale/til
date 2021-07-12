// Allow ssh from Sucho Office
resource "aws_security_group" "allow_ssh_from_sucho" {
  name        = "allow-ssh-from-sucho"
  description = "Bastion host that allows SSH connection"
  vpc_id      = aws_vpc.main_vpc.id

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = local.cidr_blocks_sucho
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

// Allow ssh from bastion instance
resource "aws_security_group" "allow_ssh_from_same_vpc" {
  name        = "allow-ssh-from-same-vpc"
  description = "Allow SSH connection from instances in same vpc to backend instance in private subnets"
  vpc_id      = aws_vpc.main_vpc.id

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [aws_vpc.main_vpc.cidr_block]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  depends_on = [
    aws_instance.bastion
  ]
}

// Allow HTTP from Sucho Office
resource "aws_security_group" "allow_http_from_sucho" {
  name        = "allow-http-from-sucho"
  description = "Allow all inbound traffic to port 80 from Sucho Office"
  vpc_id      = aws_vpc.main_vpc.id

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = concat(local.cidr_blocks_sucho, [aws_vpc.main_vpc.cidr_block])
  }

  egress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1" // allow all
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
    description      = "allow all outbound traffic"
  }
}


// Allow HTTP from all
resource "aws_security_group" "allow_http_all" {
  name        = "allow-http-from-all"
  description = "Allow all inbound traffic to port 80"
  vpc_id      = aws_vpc.main_vpc.id

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1" // allow all
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
    description      = "allow all outbound traffic"
  }
}
