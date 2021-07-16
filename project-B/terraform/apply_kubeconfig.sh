#!/bin/bash

# [Description]
# merge new kubeconfig from EKS cluster created by Terraform to current kubeconfig.
# Then, move to new context 'aws'
#
# [Maintainer]
# Donghyung Ko <koko8624@pubg.com>

set -e

if [[ -z $KUBECONFIG ]]; then
    OLD_KUBECONFIG="$HOME/.kube/config"
fi

# create new config file from Terraform output
NEW_KUBECONFIG="$HOME/.kube/eks-webapp.config"
terraform output -raw kubeconfig > $NEW_KUBECONFIG
chmod 600 ~/.kube/eks-webapp.config

# merge config files
KUBECONFIG=$OLD_KUBECONFIG:$NEW_CONFIG_PATH 
kubectl config view --merge --flatten >  ~/.kube/merged.config 
KUBECONFIG="$HOME/.kube/merged.config"
echo "Successfully merged KUBECONFIG to ${KUBECONFIG}"

# change context
kubectx aws
echo "You are now able to access EKS cluster by using 'kubectl' command"
