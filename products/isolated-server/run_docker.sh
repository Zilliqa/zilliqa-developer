#!/bin/bash

docker stop zilliqaisolatedserver
docker rm zilliqaisolatedserver

docker run -d -p 5555:5555 --name zilliqaisolatedserver zilliqaisolatedserver:1.0
