locals {
}
output "kubeconfig" {
  description = "kubectl config file contents for this EKS cluster. Will block on cluster creation until the cluster is really ready."
  sensitive   = true

  # So that calling plans wait for the cluster to be available before attempting to use it.
  # There is no need to duplicate this datasource
  depends_on = [
    aws_eks_cluster.eks,
    data.http.wait_for_cluster
  ]

  value = <<EOT
apiVersion: v1
clusters:
- cluster:
    server: ${aws_eks_cluster.eks.endpoint}
    certificate-authority-data: ${aws_eks_cluster.eks.certificate_authority[0].data}
  name: kubernetes
contexts:
- context:
    cluster: kubernetes
    user: aws
  name: aws
current-context: aws
kind: Config
preferences: {}
users:
- name: aws
  user:
    exec:
      apiVersion: client.authentication.k8s.io/v1alpha1
      command: aws-iam-authenticator
      args:
        - "token"
        - "-i"
        - "${local.cluster_name}"
EOT
}
