#!/bin/sh

echo "Environment is $DEPLOY_ENV"
if [ "$DEPLOY_ENV" = "dev" ]; then
	[ -e ./.env_dev ] && . ./.env_dev
	npm run start
else
	echo "Not dev env"
	npm run start
fi
