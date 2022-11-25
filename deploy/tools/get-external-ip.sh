#!/usr/bin/env bash
# Pass the name of a service to check: sh wait-for-service.sh example
set -euo pipefail
EXTERNAL_IP=""
SERVICE="$1"
while [ -z $EXTERNAL_IP ]; do
  EXTERNAL_IP=$(kubectl get service ${SERVICE} --template="{{range .status.loadBalancer.ingress}}{{.ip}}{{end}}")
  if [ -z "${EXTERNAL_IP}" ]; then
    echo "Waiting for external IP..."
    sleep 10
  fi
done
echo "Ready: ${EXTERNAL_IP}"