#!/bin/bash

echo $(pwd)

docker --version
aws --version

echo $COMMIT_SHA
commit=$(git rev-parse --short=7 $COMMIT_SHA)

regionID=us-west-2
application=stakez
registryURL="zilliqa/$application"
registryURL_AWS="$AWS_ACCOUNT_ID.dkr.ecr.$regionID.amazonaws.com/staking-viewer"

#eval "$(aws ecr get-login --no-include-email --region $regionID)"
# echo "$DOCKER_API_TOKEN" | docker login -u "$DOCKER_USERNAME" --password-stdin

rm -rf "$application"-artifact
mkdir -p "$application"-artifact/build/

docker build --build-arg REACT_APP_DEPLOY_ENV="stg" -t "tempimagebuild:$commit" -t "$registryURL_AWS:$commit" .
docker create --name extractbuild "tempimagebuild:$commit"
docker cp extractbuild:/usr/share/nginx/html/. $(pwd)/"$application"-artifact/build/
docker rm extractbuild
docker push "$registryURL"

aws ecr get-login-password --region $regionID | docker login --username AWS --password-stdin $registryURL_AWS
docker push "$registryURL_AWS:$commit"

cd "$application"-artifact
cd build
echo $commit >"$application"-artifact-commit.txt
zip -r "$application"-artifact.zip .
aws s3 sync . s3://"$application"-static-artifact --exclude='*' --include=''"$application"'-artifact.zip'

cd ..
echo $(date) >date_created.txt
aws s3 sync . s3://"$application"-static-artifact --exclude='*' --include='date_created.txt'
