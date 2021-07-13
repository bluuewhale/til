// [Cluster]
data "aws_iam_policy_document" "cluster_assume_role_policy" {
  statement {
    sid = "EKSClusterAssumeRole"

    actions = [
      "sts:AssumeRole"
    ]

    principals {
      type        = "Service"
      identifiers = [local.eks_principal]
    }
  }
}

resource "aws_iam_role" "cluster" {
  name                  = local.cluster_iam_role_name
  assume_role_policy    = data.aws_iam_policy_document.cluster_assume_role_policy.json
  path                  = var.iam_path
  force_detach_policies = true
  permissions_boundary  = var.iam_permissions_boundary
}

// Managed policies attached to IAM role
resource "aws_iam_role_policy_attachment" "cluster_AmazonEKSClusterPolicy" {
  policy_arn = "${local.policy_arn_prefix}/AmazonEKSClusterPolicy"
  role       = aws_iam_role.cluster.name
}

resource "aws_iam_role_policy_attachment" "cluster_AmazonEKSServicePolicy" {
  policy_arn = "${local.policy_arn_prefix}/AmazonEKSServicePolicy"
  role       = aws_iam_role.cluster.name
}


// (Optional)Enable Security Group for Pods
// see, https://docs.aws.amazon.com/eks/latest/userguide/security-groups-for-pods.html
resource "aws_iam_role_policy_attachment" "cluster_AmazonEKSVPCResourceControllerPolicy" {
  policy_arn = "${local.policy_arn_prefix}/AmazonEKSVPCResourceController"
  role       = aws_iam_role.cluster.name
}

// Custom policy to provision ELB
//data "aws_iam_policy_document" "cluster_elb_sl_role_creation" {
//  statement {
//    effect = "Allow"
//    actions = [
//      "ec2:DescribeAccountAttributes",
//      "ec2:DescribeInternetGateways",
//      "ec2:DescribeAddresses"
//    ]
//    resources = ["*"]
//  }
//
//}
//resource "aws_iam_policy" "cluster_elb_sl_role_creation" {
//  name        = "${var.cluster_prefix}-elb-sl-role-creation"
//  description = "Permission for EKS to create AWSServiceRoleForElasticBalancing service-linked role"
//  policy      = data.aws_iam_policy_document.cluster_elb_sl_role_creation.json
//  path        = var.iam_path
//}
//
//resource "aws_iam_role_policy_attachment" "cluster_elb_sl_role_creation" {
//  policy_arn = aws_iam_policy.cluster_elb_sl_role_creation.arn
//  role       = aws_iam_role.cluster.name
//}


//// worker
//data "aws_iam_policy_document" "worker_assume_role_policy" {
//  statement {
//    sid = "EKSWorkerAssumeRole"
//
//    actions = [
//      "sts:AssumeRole"
//    ]
//
//    principals {
//      type        = "Service"
//      identifiers = [local.ec2_principal]
//    }
//  }
//}
//
