#!/bin/bash

docker stop zilliqa-isolated-server
docker rm zilliqa-isolated-server
docker rmi zilliqa-isolated-server:1.0
docker build -t zilliqa-isolated-server:1.0 .
