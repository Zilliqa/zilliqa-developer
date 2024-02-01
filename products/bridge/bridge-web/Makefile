.PHONY: all
all: image/build-and-push

.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS = -ec

IMAGE_TAG ?= localhost:5001/zilliqa-bridge-web:latest

## Build and push the Docker image
image/build-and-push:
	docker build -t "${IMAGE_TAG}" .
	docker push "${IMAGE_TAG}"
