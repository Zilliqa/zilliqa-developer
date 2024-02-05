.PHONY: all
all: image/build-and-push

.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS = -ec

ENVIRONMENT ?= dev
IMAGE_TAG ?= localhost:5001/zilliqa-bridge-web:latest

## Build and push the Docker image
image/build-and-push:
	ifeq ($(ENVIRONMENT),prd)
		docker build --build-arg VITE_ENV="PROD" -t "${IMAGE_TAG}" .
	else
		docker build -t "${IMAGE_TAG}" .
	endif
	docker push "${IMAGE_TAG}"
