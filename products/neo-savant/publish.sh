#!/bin/bash

# It might look something like this:
docker build -t user/app:v1.0.0 .
docker tag user/app:v1.0.0 user/app:latest
docker push user/app:v1.0.0
