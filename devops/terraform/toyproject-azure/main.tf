provider "azurerm" {
  version = "2.2.0"
  features {}
}

provider "random" {
  version = "2.2"
}

module "location_koreacentral" {
  source = "./location"

  web_server_location      = "koreacentral"
  web_server_rg            = "${var.web_server_rg}-koreacentral"
  resource_prefix          = "${var.resource_prefix}-koreacentral"
  web_server_address_space = "1.0.0.0/16"
  web_server_name          = "web-server"
  environment              = var.environment
  web_server_count         = var.web_server_count
  web_server_subnets = {
    "web-server"         = "1.0.1.0/24"
    "AzureBastionSubnet" = "1.0.2.0/24"
  }
  terraform_script_version = var.terraform_script_version
  admin_password           = var.admin_password
}

