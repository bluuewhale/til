// Terraform AWS EKS module 
resource "aws_eks_cluster" "eks" {
  name     = local.cluster_name
  version  = local.cluster_version
  role_arn = aws_iam_role.cluster.arn

  vpc_config {
    public_access_cidrs     = local.cidr_blocks_sucho
    subnet_ids              = concat(module.vpc.public_subnets, module.vpc.private_subnets)
    endpoint_private_access = local.endpoint_private_access
    endpoint_public_access  = local.endpoint_public_access
  }

  kubernetes_network_config {
    service_ipv4_cidr = local.kubernetes_service_ipv4_cidr
  }

  # Ensure that IAM Role permissions are created before and deleted after EKS Cluster handling.
  # Otherwise, EKS will not be able to properly delete EKS managed EC2 infrastructure such as Security Groups.
  depends_on = [
    module.vpc,
    aws_iam_role_policy_attachment.cluster_AmazonEKSClusterPolicy,
    aws_iam_role_policy_attachment.cluster_AmazonEKSServicePolicy,
    aws_iam_role_policy_attachment.cluster_AmazonEKSVPCResourceControllerPolicy,
  ]
}

data "http" "wait_for_cluster" {
  url            = format("%s/healthz", aws_eks_cluster.eks.endpoint)
  ca_certificate = base64decode(aws_eks_cluster.eks.certificate_authority[0].data)
  timeout        = local.wait_for_cluster_timeout

  depends_on = [
    aws_eks_cluster.eks,
  ]
}
