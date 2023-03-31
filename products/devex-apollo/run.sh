#!/bin/bash

echo "Environment is $NODE_ENV"
if [ "$NODE_ENV" = "dev" ]; then
	echo "Dev env"
	yarn start-dev
else
	echo "Not dev env"
	yarn start
fi
