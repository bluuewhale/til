output "bastion_ip" {
  description = "bastion instance에 할당된 EIP"
  value       = aws_eip.bastion.public_ip
}
