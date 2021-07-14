data "aws_partition" "current" {} // Use this data source to lookup information about the current AWS partition in which Terraform is working.

data "aws_ami" "worker" {
  filter {
    name   = "name"
    values = [local.worker_ami_name_filter]
  }

  most_recent = true

  owners = [local.worker_ami_owner_id]
}
