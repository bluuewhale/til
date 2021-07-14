locals {
  cidr_blocks_sucho = ["103.114.125.1/32", "103.114.125.2/32"]

  // [Provider]
  aws_credentials_path = var.aws_credentials_path != "" ? var.aws_credentials_path : "~/.aws/credentials" // Path to AWS credential configuration file
  aws_profile          = var.aws_profile != "" ? var.aws_profile : "mfa"                                  // AWS profile configuration. Refers to AWS_PROFILE

  // [VPC]
  vpc_name             = var.vpc_name != "" ? var.vpc_name : "onboarding-project-B"
  region               = var.region != "" ? var.region : "sa-east-1"
  vpc_cidr             = var.vpc_cidr != "" ? var.vpc_cidr : "10.0.0.0/16"                                                                  // CIDR block for VPC
  public_subnet_cidrs  = var.public_subnet_cidrs[0] != "" ? var.public_subnet_cidrs : ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]         // CIDR block for public subnets in VPC
  private_subnet_cidrs = var.private_subnet_cidrs[0] != "" ? var.private_subnet_cidrs : ["10.0.100.0/24", "10.0.101.0/24", "10.0.102.0/24"] // CIDR block for priavate subnets in VPC

  // [EKS]
  cluster_name                 = var.cluster_name != "" ? var.cluster_name : "onboarding-project-B"                          // EKS cluster name
  cluster_version              = var.cluster_version != "" ? var.cluster_version : "1.17"                                    // Version of the EKS cluster
  cluster_suffix               = var.cluster_suffix != "" ? var.cluster_suffix : local.cluster_name                          // EKS cluster resource suffix
  kubernetes_service_ipv4_cidr = var.kubernetes_service_ipv4_cidr != "" ? var.kubernetes_service_ipv4_cidr : "172.20.0.0/16" // CIDR block for Kubernetes Services

  // [Node Group]
  webapp_ami_name_filter = var.webapp_ami_name_filter != "" ? var.webapp_ami_name_filter : "amazon-eks-node-${local.cluster_version}-v*" // AMI Name filter
  webapp_ami_owner_id    = var.webapp_ami_owner_id != "" ? var.webapp_ami_owner_id : "amazon"                                            // AMI Onwer

  node_group_webapp_settings = {
    // (Required)
    cluster_name  = aws_eks_cluster.eks.name   // Name of the EKS cluster that node group associate with
    node_role_arn = aws_iam_role.webapp.arn    // ARN of IAM role that provide permissions for the node group
    subnet_ids    = module.vpc.private_subnets // A list of subnets to place the nodes(ASG)
    scaling_config = {
      desired_size = var.node_group_desired_size != "" ? tonumber(var.node_group_desired_size) : 3 // Desired webapp capacity in ASG
      min_size     = var.node_group_min_size != "" ? tonumber(var.node_group_min_size) : 3         // Minimum webapp capacity in ASG
      max_size     = var.node_group_max_size != "" ? tonumber(var.node_group_max_size) : 5         // Maximum webapp capacity in ASG
    }

    // (Optional)
    node_group_name = var.node_group_name != "" ? var.node_group_name : "webapp"                            # Name of the webapp group
    ami_id          = data.aws_ami.webapp.id                                                                # AMI id for nodes in node group
    ami_type        = "AL2_x86_64"                                                                          # Type of AMI for nodes
    instance_types  = var.node_group_instance_types[0] != "" ? var.node_group_instance_types : ["t2.micro"] # Instance type for nodes in node group
    disk_size       = var.node_group_disk_size != "" ? var.node_group_disk_size : 20
    iam_role_id     = resource.aws_iam_role.webapp.id # A custom IAM role id . 
    labels = {
      "app" : "webapp"
    }
  }



  // [IAM Role]
  ec2_principal            = "ec2.${data.aws_partition.current.dns_suffix}" // e.g. amazonaws.com in AWS Commercial, amazonaws.com.cn in AWS China
  eks_principal            = "eks.amazonaws.com"                            // TODO: ec2와 달리 domain-suffix가 변하지 않는지 여부 확인해보기
  cluster_iam_role_name    = var.cluster_iam_role_name != "" ? var.cluster_iam_role_name : "${var.cluster_suffix}-cluster-iam-role"
  webapp_iam_role_name     = var.webapp_iam_role_name != "" ? var.webapp_iam_role_name : "${var.cluster_suffix}-webapp-iam-role"
  policy_arn_prefix        = "arn:${data.aws_partition.current.partition}:iam::aws:policy"
  iam_path                 = var.iam_path != "" ? var.iam_path : "/"
  iam_permissions_boundary = var.iam_permissions_boundary != "" ? var.iam_permissions_boundary : null

}
