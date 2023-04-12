#!/bin/bash

docker stop devex-apollo
docker rm devex-apollo
docker run -d --name devex-apollo devex-apollo:1.0
