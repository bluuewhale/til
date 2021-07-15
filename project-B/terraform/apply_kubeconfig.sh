#!/bin/bash
set -xe

terraform output -raw kubeconfig > ~/.kube/eks-webapp.config
export KUBECONFIG=~/.kube/eks-webapp.config
