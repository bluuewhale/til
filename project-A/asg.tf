//resource "aws_placement_group" "webapp" {
//  name     = "webapp"
//  strategy = "cluster"
//}

resource "aws_autoscaling_group" "webapp" {
  name = "webapp"

  // counts
  min_size         = 3
  max_size         = 5
  desired_capacity = 3

  // health check
  health_check_type         = "ELB"
  health_check_grace_period = 300 // required when using "ELB" for health_check_type

  // configurations
  launch_template {
    id      = aws_launch_template.webapp.id
    version = "$Latest"
  }

  target_group_arns = [
    aws_alb_target_group.webapp.arn,
  ]

  vpc_zone_identifier = [ // provision ASG in private subnets only
    aws_subnet.private_subnet01.id,
    aws_subnet.private_subnet02.id,
  ]
  //placement_group      = aws_placement_group.webapp.id

  depends_on = [
    aws_launch_template.webapp
  ]

  lifecycle {
    ignore_changes = [
      launch_template
    ]
  }
}

// Launch Template
resource "aws_launch_template" "webapp" {
  name          = "webapp-launch-template"
  image_id      = data.aws_ami.ubuntu.id
  instance_type = "t2.micro"

  vpc_security_group_ids = [
    aws_security_group.allow_http_all.id,         // allow 80 from all
    aws_security_group.allow_ssh_from_same_vpc.id // allow 22 from bastion instance only
  ]

  iam_instance_profile {
    name = aws_iam_instance_profile.ssm.name
  }

  user_data = filebase64("${path.module}/launch.sh")

  lifecycle {
    ignore_changes = [
      image_id,
      user_data
    ]
  }
}
