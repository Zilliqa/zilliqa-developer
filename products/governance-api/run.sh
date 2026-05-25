#!/bin/bash

echo "Postgres Database: $POSTGRES_DB"
echo "Postgres Host: $POSTGRES_HOST"
echo "Postgres username: $POSTGRES_USER"

echo "Environment is $DEPLOY_ENV"
echo "run script $SCRIPT"
echo "Callback: $CALLBACK"
date
yarn db:create
yarn db:migrate
yarn db:seed
yarn $SCRIPT
