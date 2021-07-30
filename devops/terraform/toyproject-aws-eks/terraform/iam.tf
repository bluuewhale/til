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
  path                  = local.iam_path
  force_detach_policies = true
  permissions_boundary  = local.iam_permissions_boundary
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
data "aws_iam_policy_document" "cluster_elb_sl_role_creation" {
  statement {
    effect = "Allow"
    actions = [
      "ec2:DescribeAccountAttributes",
      "ec2:DescribeInternetGateways",
      "ec2:DescribeAddresses"
    ]
    resources = ["*"]
  }

}
resource "aws_iam_policy" "cluster_elb_sl_role_creation" {
  name        = "elb-sl-role-creation"
  description = "Permission for EKS to create AWSServiceRoleForElasticBalancing service-linked role"
  policy      = data.aws_iam_policy_document.cluster_elb_sl_role_creation.json
  path        = local.iam_path
}

resource "aws_iam_role_policy_attachment" "cluster_elb_sl_role_creation" {
  policy_arn = aws_iam_policy.cluster_elb_sl_role_creation.arn
  role       = aws_iam_role.cluster.name
}


// webapp
data "aws_iam_policy_document" "webapp_assume_role_policy" {
  statement {
    sid = "EKSWorkerAssumeRole"

    actions = [
      "sts:AssumeRole"
    ]

    principals {
      type        = "Service"
      identifiers = [local.ec2_principal]
    }
  }
}

resource "aws_iam_role" "webapp" {
  name                  = local.webapp_iam_role_name
  assume_role_policy    = data.aws_iam_policy_document.webapp_assume_role_policy.json
  path                  = local.iam_path
  force_detach_policies = true
  permissions_boundary  = local.iam_permissions_boundary
}

// [Managed policies attached to IAM role]

// (Required) Enable EC2 Describe Actions
// see https://console.aws.amazon.com/iam/home#/policies/arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy$jsonEditor
resource "aws_iam_role_policy_attachment" "webapp_AmazonEKSWorkerNodePolicy" {
  policy_arn = "${local.policy_arn_prefix}/AmazonEKSWorkerNodePolicy"
  role       = aws_iam_role.webapp.name
}

// (Required)
resource "aws_iam_role_policy_attachment" "webapp_AmazonEC2ContainerRegistryReadOnly" {
  policy_arn = "${local.policy_arn_prefix}/AmazonEC2ContainerRegistryReadOnly"
  role       = aws_iam_role.webapp.name
}

// (Optional) Needed for Amazon VPC CNI plugin
// see: https://console.aws.amazon.com/iam/home?#/policies/arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy$jsonEditor
resource "aws_iam_role_policy_attachment" "webapp_AmazonEKSCNIPolicy" {
  policy_arn = "${local.policy_arn_prefix}/AmazonEKS_CNI_Policy"
  role       = aws_iam_role.webapp.name
}




