locals {
  cidr_blocks_sucho = ["103.114.125.1/32", "103.114.125.2/32"]

  // eks cluster
  default_platform = "linux"

  // IAM role
  ec2_principal         = "ec2.${data.aws_partition.current.dns_suffix}" // e.g. amazonaws.com in AWS Commercial, amazonaws.com.cn in AWS China
  eks_principal         = "eks.amazonaws.com"                            // TODO: ec2와 달리 domain-suffix가 변하지 않는지 여부 확인해보기
  cluster_iam_role_name = var.cluster_iam_role_name != "" ? var.cluster_iam_role_name : "${var.cluster_suffix}-cluster-iam-role"

  policy_arn_prefix = "arn:${data.aws_partition.current.partition}:iam::aws:policy"

  // AMI
  worker_ami_name_filter = var.worker_ami_name_filter != "" ? var.worker_ami_name_filter : "amazon-eks-node-${var.cluster_version}-v*"
}
