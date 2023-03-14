#!/bin/bash


echo $(pwd)

docker --version
aws --version

echo $TRAVIS_COMMIT
commit=$(git rev-parse --short=7 $TRAVIS_COMMIT)

accountID=$(aws sts get-caller-identity --output text --query 'Account')
regionID=ap-southeast-2

#registryURL=${accountID}.dkr.ecr.${regionID}.amazonaws.com/$application

rm -rf savant-artifact
mkdir -p savant-artifact/stg/
mkdir -p savant-artifact/prd/
docker build --build-arg DEPLOY_ENV="stg" -t "tempimagestg:$commit" .
docker create --name extractstg "tempimagestg:$commit"
docker cp extractstg:/usr/share/nginx/html/. $(pwd)/savant-artifact/stg/
docker rm extractstg

docker build --build-arg DEPLOY_ENV="prd" -t "tempimageprd:$commit" .
docker create --name extractprd "tempimageprd:$commit"
docker cp extractprd:/usr/share/nginx/html/. $(pwd)/savant-artifact/prd/
docker rm extractprd

cd savant-artifact

cd stg
echo $commit > savant-artifact-commit.txt
#tar -czvf savant-artifact-stg.gz .
zip -r savant-artifact-stg.zip .
aws s3 sync . s3://neo-savant-static-artifact --exclude='*' --include='savant-artifact-stg.zip'
cd ..

cd prd
echo $commit > savant-artifact-commit.txt
#tar -czvf savant-artifact-prd.gz .
zip -r savant-artifact-prd.zip .
aws s3 sync . s3://neo-savant-static-artifact --exclude='*' --include='savant-artifact-prd.zip'
cd ..

echo $(date) > date_created.txt
aws s3 sync . s3://neo-savant-static-artifact --exclude='*' --include='date_created.txt'

# Push to s3 staging
