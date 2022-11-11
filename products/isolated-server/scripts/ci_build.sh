#!/bin/bash

echo $(pwd)

docker --version
aws --version

echo "$TRAVIS_COMMIT"
commit=$(git rev-parse --short=7 "$TRAVIS_COMMIT")

accountID=$(aws sts get-caller-identity --output text --query 'Account')
regionID=us-west-2

application=zilliqa-isolated-server
registryURL=${accountID}.dkr.ecr.${regionID}.amazonaws.com/$application

# Uncomment below if we're pulling from zilliqa ecr
#eval "$(aws ecr get-login --no-include-email --region $regionID --registry-ids $zilliqa_ecr_id)"

docker build -t "zilliqa/$application:latest" -t "zilliqa/$application:$commit" -t "$registryURL:latest" -t "$registryURL:$commit" .
eval "$(aws ecr get-login --no-include-email --region $regionID)"
docker push "$registryURL:latest"
docker push "$registryURL:$commit"

echo "$DOCKER_API_TOKEN" | docker login -u "$DOCKER_USERNAME" --password-stdin
docker push "zilliqa/$application:latest"
docker push "zilliqa/$application:$commit"
