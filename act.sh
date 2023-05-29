#!/bin/bash

#echo ${WEBHOOK_URL}
#env_file_contents=$(cat .env)
#echo ${env_file_contents}
#architecture="linux/amd64"
#if [ "$(uname -m)" == "arm64" ]; then
#architecture="linux/arm64"
#fi

#  --container-architecture $architecture \
act -j build-chatgpt \
  --secret-file ~/.config/gh/.env.secret \
  -s WEBHOOK_URL=${WEBHOOK_URL} \
  -s DEPLOY_PATH=$DEPLOY_PATH \
  -s SSH_KEY="$(cat ~/.ssh/id_rsa)" \
  --insecure-secrets \
#  -s DEBUG=true \
#  --use-gitignore true \
#  -v

