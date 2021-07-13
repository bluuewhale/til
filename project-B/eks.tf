// Terraform AWS EKS module 

resource "aws_eks_cluster" "eks" {
  name     = var.cluster_name
  version  = var.cluster_version
  role_arn = aws_iam_role.cluster.arn

  vpc_config {
    public_access_cidrs = local.cidr_blocks_sucho
    subnet_ids          = concat(module.vpc.private_subnets, module.vpc.public_subnets)
  }

  kubernetes_network_config {
    service_ipv4_cidr = var.kubernetes_service_ipv4_cidr
  }

  # Ensure that IAM Role permissions are created before and deleted after EKS Cluster handling.
  # Otherwise, EKS will not be able to properly delete EKS managed EC2 infrastructure such as Security Groups.
  depends_on = [
    aws_iam_role_policy_attachment.cluster_AmazonEKSClusterPolicy,
    aws_iam_role_policy_attachment.cluster_AmazonEKSServicePolicy,
    aws_iam_role_policy_attachment.cluster_AmazonEKSVPCResourceControllerPolicy,
  ]
}
