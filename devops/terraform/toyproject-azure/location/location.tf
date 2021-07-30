provider "azurerm" {
  version = "2.2.0"
  features {}
}

locals { # define local variables
  web_server_name  = var.environment == "production" ? "${var.web_server_name}-production" : "${var.web_server_name}-dev"
  build_enviroment = var.environment == "production" ? "production" : "development"
}

resource "azurerm_resource_group" "web_server_rg" {
  name     = var.web_server_rg
  location = var.web_server_location

  tags = {
    environment   = local.build_enviroment
    build-version = var.terraform_script_version
  }
}

resource "random_string" "random" {
  length  = 10
  upper   = false
  special = false
  number  = false
}

resource "azurerm_storage_account" "web_server_storage_account" {
  name                     = "bootdiaggnos${random_string.random.result}"
  resource_group_name      = azurerm_resource_group.web_server_rg.name
  location                 = azurerm_resource_group.web_server_rg.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

resource "azurerm_virtual_network" "web_server_vnet" {
  name                = "${var.resource_prefix}-vnet"
  location            = var.web_server_location
  resource_group_name = azurerm_resource_group.web_server_rg.name
  address_space       = [var.web_server_address_space]
}


#resource "azurerm_subnet" "web_server_subnet" {
#  name                 = "${var.resource_prefix}-subnet"
#  resource_group_name  = azurerm_resource_group.web_server_rg.name
#  virtual_network_name = azurerm_virtual_network.web_server_vnet.name
#  address_prefix       = var.web_server_address_prefix # vnet의 address_space에 포함되어야 함! CIDR 잘 정하기
#}

# iteratively create subnets
resource "azurerm_subnet" "web_server_subnet" {
  for_each = var.web_server_subnets # used for "map" iteration

  name                 = each.key
  address_prefix       = each.value # vnet의 address_space에 포함되어야 함! CIDR 잘 정하기;
  resource_group_name  = azurerm_resource_group.web_server_rg.name
  virtual_network_name = azurerm_virtual_network.web_server_vnet.name
}

#resource "azurerm_network_interface" "web_server_nic" {                             # NIC는 VM마다 하나씩 필요
#  name                = "${var.web_server_name}-nic-${format("%02d", count.index)}" # must be unique
#  location            = var.web_server_location
#  resource_group_name = azurerm_resource_group.web_server_rg.name
#  count               = var.web_server_count
#
#  ip_configuration {
#    name                          = "${var.web_server_name}-ip"
#    subnet_id                     = azurerm_subnet.web_server_subnet["web-server"].id
#    private_ip_address_allocation = "dynamic"
#    public_ip_address_id          = count.index == 0 ? azurerm_public_ip.web_server_public_ip.id : null
#  }
#}
#
resource "azurerm_public_ip" "web_server_public_ip" {
  name                = "${var.resource_prefix}-public-ip"
  location            = var.web_server_location
  resource_group_name = azurerm_resource_group.web_server_rg.name
  allocation_method   = var.environment == "production" ? "Static" : "Dynamic"
}

resource "azurerm_network_security_group" "web_server_nsg" {
  name                = "${var.resource_prefix}-nsg"
  location            = var.web_server_location
  resource_group_name = azurerm_resource_group.web_server_rg.name
}

resource "azurerm_network_security_rule" "web_server_nsg_rule_ssh" {
  name                       = "SSH Inbound"
  priority                   = 100
  direction                  = "Inbound"
  access                     = "Allow"
  protocol                   = "Tcp"
  source_address_prefix      = "*"
  source_port_range          = "*"
  destination_address_prefix = "*"
  destination_port_range     = "22"

  resource_group_name         = azurerm_resource_group.web_server_rg.name
  network_security_group_name = azurerm_network_security_group.web_server_nsg.name
  #count                       = var.environment == "production" ? 0 : 1 # do not create ssh rule when in development mode
}

resource "azurerm_network_security_rule" "web_server_nsg_rule_http" {
  name                       = "HTTP Inbound"
  priority                   = 110
  direction                  = "Inbound"
  access                     = "Allow"
  protocol                   = "Tcp"
  source_address_prefix      = "*"
  source_port_range          = "*"
  destination_address_prefix = "*"
  destination_port_range     = "80"

  resource_group_name         = azurerm_resource_group.web_server_rg.name
  network_security_group_name = azurerm_network_security_group.web_server_nsg.name
  #count                       = var.environment == "production" ? 0 : 1 # do not create http rule when in development mode
}

# use NISGA when there is only one VM(network interface). 
#resource "azurerm_network_interface_security_group_association" "web_server_nisga" {
#  network_security_group_id = azurerm_network_security_group.web_server_nsg.id
#  network_interface_id      = azurerm_network_interface.web_server_nic.id
#}

# use SNSGA when there are more than 2 VMs because NIs are gathered into a subnet
resource "azurerm_subnet_network_security_group_association" "web_server_snsga" {
  network_security_group_id = azurerm_network_security_group.web_server_nsg.id
  subnet_id                 = azurerm_subnet.web_server_subnet["web-server"].id
}

#resource "azurerm_windows_virtual_machine" "web_server" {
#  name                  = "${var.web_server_name}-${format("%02d", count.index)}" # must be unique
#  count                 = var.web_server_count
#  location              = var.web_server_location
#  resource_group_name   = azurerm_resource_group.web_server_rg.name
#  size                  = "Standard_B1s"
#  admin_username        = "web_server"
#  admin_password        = "Passw0rd1234"
#  network_interface_ids = [azurerm_network_interface.web_server_nic[count.index].id] # 1:1 match to NI, you can use slicing when there are more than 2 NI(VI)s
#  availability_set_id   = azurerm_availability_set.web_server_availability_set.id
#
#  os_disk {
#    caching              = "ReadWrite"
#    storage_account_type = "Standard_LRS"
#  }
#
#  source_image_reference {
#    publisher = "MicrosoftWindowsServer"
#    offer     = "WindowsServerSemiAnnual"
#    sku       = "Datacenter-Core-1709-smalldisk"
#    version   = "latest"
#  }
#}

#resource "azurerm_availability_set" "web_server_availability_set" {
#  name                        = "${var.resource_prefix}-availability-set"
#  location                    = var.web_server_location
#  resource_group_name         = azurerm_resource_group.web_server_rg.name
#  managed                     = true
#  platform_fault_domain_count = 2
#}

# window VM with virtual machine scale set
#resource "azurerm_virtual_machine_scale_set" "web_server" {
#  name                = "${var.resource_prefix}-scale-set"
#  location            = var.web_server_location
#  resource_group_name = azurerm_resource_group.web_server_rg.name
#  upgrade_policy_mode = "manual"
#
#  network_profile {
#    name    = "${var.resource_prefix}-network-profile"
#    primary = true
#
#    ip_configuration {
#      name      = "${var.resource_prefix}-ip-configuration"
#      subnet_id = azurerm_subnet.web_server_subnet["web-server"].id
#      primary   = true
#    }
#  }
#
#  sku {
#    name     = "Standard_B1s"
#    tier     = "Standard"
#    capacity = var.web_server_count
#  }
#
#  os_profile {
#    computer_name_prefix = var.web_server_name
#    admin_username       = "webserver"
#    admin_password       = "passw0rd1234"
#  }
#
#  storage_profile_image_reference {
#    publisher = "Canonical"
#    offer     = "UbuntuServer"
#    sku       = "16.04-LTS"
#    version   = "latest"
#  }
#
#  storage_profile_os_disk {
#    name              = ""
#    caching           = "ReadWrite"
#    create_option     = "FromImage"
#    managed_disk_type = "Standard_LRS"
#  }
#
#  os_profile_windows_config {
#    provision_vm_agent = true
#  }
#}

# linux VM with azurerm_linux_virtual_machine_scale_set
resource "azurerm_linux_virtual_machine_scale_set" "web-server" {
  name                = "${var.resource_prefix}-linux-vm"
  resource_group_name = azurerm_resource_group.web_server_rg.name
  location            = azurerm_resource_group.web_server_rg.location
  depends_on = [
    azurerm_lb_backend_address_pool.web_server_lb_backend_pool # lb backend pool
  ]

  sku                             = "Standard_B1s"
  instances                       = 2
  admin_username                  = "webserver"
  admin_password                  = var.admin_password
  disable_password_authentication = false

  source_image_reference {
    publisher = "Canonical"
    offer     = "UbuntuServer"
    sku       = "16.04-LTS"
    version   = "latest"
  }

  network_interface {
    name    = "${local.web_server_name}-network-interface"
    primary = true

    ip_configuration {
      name                                   = "${local.web_server_name}-network-interface-ip-configuration"
      primary                                = true
      subnet_id                              = azurerm_subnet.web_server_subnet["web-server"].id
      load_balancer_backend_address_pool_ids = [azurerm_lb_backend_address_pool.web_server_lb_backend_pool.id]
    }
  }

  os_disk {
    storage_account_type = "Standard_LRS"
    caching              = "ReadWrite"
  }

  boot_diagnostics {
    storage_account_uri = azurerm_storage_account.web_server_storage_account.primary_blob_endpoint
  }
}

resource "azurerm_lb" "web_server_lb" {
  name                = "${local.web_server_name}-lb-${local.build_enviroment}"
  resource_group_name = azurerm_resource_group.web_server_rg.name
  location            = azurerm_resource_group.web_server_rg.location

  frontend_ip_configuration {
    name                 = "${local.web_server_name}-frontend-ip-configuration"
    public_ip_address_id = azurerm_public_ip.web_server_public_ip.id
  }
}

resource "azurerm_lb_backend_address_pool" "web_server_lb_backend_pool" {
  name                = "${local.web_server_name}-lb-backend-pool"
  resource_group_name = azurerm_resource_group.web_server_rg.name
  loadbalancer_id     = azurerm_lb.web_server_lb.id
}

resource "azurerm_lb_probe" "web_server_lb_probe_http" {
  name                = "${local.web_server_name}-lb-probe-http"
  resource_group_name = azurerm_resource_group.web_server_rg.name
  loadbalancer_id     = azurerm_lb.web_server_lb.id
  protocol            = "Tcp"
  port                = "80"
}

resource "azurerm_lb_rule" "web_server_lb_rule" {
  name                = "${local.web_server_name}-lb-rule-inbound"
  resource_group_name = azurerm_resource_group.web_server_rg.name
  loadbalancer_id     = azurerm_lb.web_server_lb.id
  protocol            = "Tcp"
  frontend_port       = "80"
  backend_port        = "80"

  frontend_ip_configuration_name = azurerm_lb.web_server_lb.frontend_ip_configuration[0].name
  probe_id                       = azurerm_lb_probe.web_server_lb_probe_http.id
  backend_address_pool_id        = azurerm_lb_backend_address_pool.web_server_lb_backend_pool.id
}

