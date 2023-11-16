.PHONY: all
all: image/build-and-push

.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS = -ec

IMAGE_TAG ?= localhost:5001/zilliqa-isolated-server:latest

## Build and push the Docker image
image/build-and-push:
	docker pull nginx:latest
	docker tag nginx:latest $(shell echo ${IMAGE_TAG} | sed 's/zilliqa-isolated-server/nginx/')
	docker push $(shell echo ${IMAGE_TAG} | sed 's/zilliqa-isolated-server/nginx/')
	docker build -t "${IMAGE_TAG}" .
	docker push "${IMAGE_TAG}"