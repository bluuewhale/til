resource "aws_eks_node_group" "webapp" {
  node_group_name = local.node_group_webapp_settings.node_group_name

  // [Required]
  cluster_name  = local.node_group_webapp_settings.cluster_name
  node_role_arn = aws_iam_role.webapp.arn
  subnet_ids    = local.node_group_webapp_settings.subnet_ids

  scaling_config {
    desired_size = local.node_group_webapp_settings.scaling_config["desired_size"]
    max_size     = local.node_group_webapp_settings.scaling_config["max_size"]
    min_size     = local.node_group_webapp_settings.scaling_config["min_size"]
  }

  // [Optional]
  ami_type       = local.node_group_webapp_settings.ami_type
  instance_types = local.node_group_webapp_settings.instance_types

  lifecycle {
    ignore_changes = [
      scaling_config[0].desired_size
    ]
  }

  depends_on = [
    aws_iam_role_policy_attachment.webapp_AmazonEKSWorkerNodePolicy,
    aws_iam_role_policy_attachment.webapp_AmazonEC2ContainerRegistryReadOnly,
    //aws_iam_role_policy_attachment.webapp_AmazonEKSCNIPolicy,
  ]
}
