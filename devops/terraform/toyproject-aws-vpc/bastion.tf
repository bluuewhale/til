resource "aws_instance" "bastion" {
  ami                         = data.aws_ami.ubuntu.id
  instance_type               = "t2.micro"
  associate_public_ip_address = true
  key_name                    = aws_key_pair.bastion_key.key_name

  subnet_id = aws_subnet.public_subnet01.id
  vpc_security_group_ids = [
    aws_security_group.allow_ssh_from_sucho.id,
    aws_security_group.allow_http_from_sucho.id
  ]

  iam_instance_profile = aws_iam_instance_profile.ssm.name
  user_data            = file("${path.module}/launch.sh")
}

resource "aws_eip" "bastion" {
  vpc      = true
  instance = aws_instance.bastion.id
}

resource "tls_private_key" "bastion_key" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

resource "aws_key_pair" "bastion_key" {
  key_name   = "bastion_key"
  public_key = tls_private_key.bastion_key.public_key_openssh

  provisioner "local-exec" {
    command = <<EOT
      echo '${tls_private_key.bastion_key.private_key_pem}' > ./bastion.pem
      chmod 400 ./bastion.pem
      mv ./bastion.pem $HOME/.ssh/
    EOT
    environment = {
      HOME = pathexpand("~")
    }
  }
}
