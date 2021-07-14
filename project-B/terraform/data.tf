data "aws_partition" "current" {} // Use this data source to lookup information about the current AWS partition in which Terraform is working.

data "aws_ami" "webapp" {
  filter {
    name   = "name"
    values = [local.webapp_ami_name_filter]
  }

  most_recent = true

  owners = [local.webapp_ami_owner_id]
}
