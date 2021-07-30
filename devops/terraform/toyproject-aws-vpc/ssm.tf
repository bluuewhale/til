resource "aws_iam_instance_profile" "ssm" {
  name = "ssm-profile"
  role = aws_iam_role.ssm.name
}

resource "aws_iam_role" "ssm" {
  name = "ssm"

  assume_role_policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": {
      "Effect": "Allow",
      "Principal": {
          "Service": "ssm.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
}
EOF
}

resource "aws_iam_role_policy_attachment" "ssm_attachment" {
  role       = aws_iam_role.ssm.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

//resource "aws_ssm_activation" "foo" {
//  name               = "ssm_activation"
//  description        = "give AmazonSSMManagedInstanceCore policy to EC2 instances"
//  expiration_date    = "2099-12-31T00:00:00+00:00" // prevent expirations
//  iam_role           = aws_iam_role.ssm.id
//  registration_limit = "100"
//  depends_on         = [aws_iam_role_policy_attachment.ssm_attachment]
//}

//resource "aws_iam_instance_profile" "ssm_ec2" {
//
//  name = "ssm-ec2-profile"
//  role = aws_iam_role.ssm_e2c.name
//}
//
//resource "aws_iam_role" "ssm_e2c" {
//  name               = "ssm-e2c-role"
//  path               = "/"
//  assume_role_policy = data.aws_iam_policy_document.instance_assume_role_policy.json
//
//  inline_policy {
//    name = "ssm-role"
//    policy = jsonencode({
//      Version = "2012-10-17"
//      Statement = [
//        {
//          Action   = ["ssm:StartSession"]
//          Effect   = "Allow"
//          Resource = "*"
//        },
//      ]
//    })
//  }
//}
//data "aws_iam_policy_document" "instance_assume_role_policy" {
//  statement {
//    actions = ["sts:AssumeRole"]
//
//    principals {
//      type        = "Service"
//      identifiers = ["ec2.amazonaws.com"]
//    }
//  }
//}
//
