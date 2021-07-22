// ALB
resource "aws_alb" "main_alb" {
  name               = "main-alb"
  internal           = false
  load_balancer_type = "application"
  ip_address_type    = "ipv4"

  security_groups = [
    aws_security_group.allow_ssh_from_sucho.id,
    aws_security_group.allow_http_from_sucho.id
  ]

  subnets = [
    aws_subnet.public_subnet01.id,
    aws_subnet.public_subnet02.id,
  ]

  depends_on = [
    aws_internet_gateway.igw
  ]
}

// ALB Listener
resource "aws_alb_listener" "http" {
  load_balancer_arn = aws_alb.main_alb.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_alb_target_group.webapp.arn
  }
}

// ALB Target Group
resource "aws_alb_target_group" "webapp" {
  name        = "webapp"
  port        = 80
  protocol    = "HTTP"
  target_type = "instance"
  vpc_id      = aws_vpc.main_vpc.id

  health_check {
    enabled             = true
    protocol            = "HTTP"
    port                = 80
    path                = "/"
    interval            = 10
    healthy_threshold   = 5
    unhealthy_threshold = 5
  }
}
