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
hasRepo=$(helm repo list | grep ingress-nginx | wc -l)
if ! (( $hasRepo )) ; then
    log_info "Get Helm repo 'ingress-nginx'"
	helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
fi

log_info "Update Helm repo"
helm repo update

# Install/Upgrade ingress-nginx chart
hasIngressNginxInstall=$(helm list --all-namespaces | grep ingress-nginx | wc -l)
if (( hasIngressNginxInstall )) ; then
	log_info "Upgrade helm chart 'ingress-nginx'"
	helm upgrade --values ./values/ingress-nginx-values.yaml ingress-nginx ingress-nginx/ingress-nginx --install > /dev/null
else 
    ctx=$(kubectx --current)
    if [[ "$ctx" == "minikube" ]]; then # explicitly set a random external IP for load-balancer when using minikube
        (
        sleep 5;
        log_info "explicitly set a random external IP for load-balancer when using minikube"
        kubectl patch svc ingress-nginx-controller \
            -n default \
            -p '{"spec": {"type": "LoadBalancer", "externalIPs":["172.31.71.218"]}}'
        ) &
    fi

	log_info "Install helm chart 'ingress-nginx'"
	helm install --values ./values/ingress-nginx-values.yaml ingress-nginx ingress-nginx/ingress-nginx --wait > /dev/null
fi

# Install/Upgrade web application chart
hasWebAppInstall=$(helm list --all-namespaces | grep webapp | wc -l )
if (( hasWebAppInstall )) ; then
	log_info "Upgrade helm chart 'webapp'"
	helm upgrade --values ./values/webapp-values.yaml --namespace webapp webapp ./package/webapp-0.1.0.tgz --install 
else 
	log_info "Install helm chart 'webapp'"
	helm install --values ./values/webapp-values.yaml --namespace webapp --create-namespace webapp ./package/webapp-0.1.0.tgz
fi

log_info "Successfully launched applications"
