#!/bin/bash

# [Description]
# merge new kubeconfig from EKS cluster created by Terraform to current kubeconfig.
# Then, move to new context 'aws'
#
# [Maintainer]
# Donghyung Ko <koko8624@pubg.com>

set -e

if [[ -z $KUBECONFIG ]]; then
    OLD_KUBECONFIG=~/.kube/config
fi

# create new config file from Terraform output
NEW_KUBECONFIG=~/.kube/eks-webapp.config
#terraform output -raw kubeconfig > $NEW_KUBECONFIG

# create backup file for original kubeconfig file
BACKUP_FILE_PATH="$OLD_KUBECONFIG.backup.$(date +%Y-%m-%d.%H:%M:%S)"
cp $OLD_KUBECONFIG $BACKUP_FILE_PATH
echo "Create '$BACKUP_FILE_PATH' as backup of original kubeconfig file located at $OLD_KUBECONFIG"

# merge kubeconfigs
export KUBECONFIG=$OLD_KUBECONFIG:$NEW_KUBECONFIG
kubectl config view --merge --flatten > ~/.kube/tmptmp.config && mv ~/.kube/tmptmp.config $OLD_KUBECONFIG
KUBECONFIG=$OLD_KUBECONFIG
chmod 600 $KUBECONFIG
echo "Successfully merged KUBECONFIG to ${KUBECONFIG}"

# change context
kubectx aws
echo "You are now able to access EKS cluster by using 'kubectl' command"
