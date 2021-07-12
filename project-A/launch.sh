#!/bin/bash
set -ex

# update & install dependencies
sudo apt update
sudo apt install -y apt-transport-https ca-certificates curl software-properties-common

# install docker
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu bionic stable"
sudo apt update
apt-cache policy docker-ce

sudo apt install -y docker-ce
sudo usermod -a -G docker ubuntu # hard-coded

# run docker image
sudo docker run -d --net=host --privileged koko8624/go-http-server # need --privileged option to run on port 80