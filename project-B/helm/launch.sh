#!/bin/bash
#
# [Description]
# This script is for launching a simple web application with Helm
#
# [Maintainer]
# Donghyung Ko <koko8624@pubg.com>

set -e

#====================== log ======================
log_info() {
    # Usage: log_info "this is the info log message"
    NOW=$(date +"%Y-%m-%d %H:%M:%S")
    echo "${NOW} [INFO] $1"
}

log_warnning() {
    # Usage: log_warnning "this is the warning log message"
    NOW=$(date +"%Y-%m-%d %H:%M:%S")
    echo "${NOW} [WARNNING] $1"
}

log_error() {
    # Usage: log_error "this is the error log message"
    NOW=$(date +"%Y-%m-%d %H:%M:%S")
    echo "${NOW} [ERROR] $1"
}

log_exit() {
    # Usage: log_exit "the log message before exit"
    log_error "$1"
    exit 1
}


# ======================== main ===========================

# Check if executable 'helm' exists
if ! [ -x "$(which helm)" ] ; then
	log_exit "'helm' must be installed"
fi

# Check if helm version is v3.*
if ! [[ "$(helm version --short)" == v3.* ]] ; then
	log_exit "You're using helm version of $(helm version --short). Please upgrade your helm version up to v3.*"
fi


# Get Helm repo for ingress-nginx if not exists
log_info "Get Helm repo 'ingress-nginx' if not exists"

hasRepo=$(helm repo list | grep ingress-nginx | wc -l)
if ! (( $hasRepo )) ; then
	helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
fi

log_info "Update Helm repo"
helm repo update

# Install ingress-nginx chart
hasInstall=$(helm list | grep ingress-nginx | wc -l)
if (( hasInstall )) ; then
	log_info "Upgrade helm chart 'ingress-nginx'"
	helm upgrade --values ingress-nginx-values.yaml ingress-nginx ingress-nginx/ingress-nginx --install > /dev/null
else 
	log_info "Install helm chart 'ingress-nginx'"
	helm install --values ingress-nginx-values.yaml ingress-nginx ingress-nginx/ingress-nginx > /dev/null
fi

