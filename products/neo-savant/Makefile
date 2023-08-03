.PHONY: all
all: image/build image/push

.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS = -ec

ENVIRONMENT ?= dev
VALID_ENVIRONMENTS := dev stg prd
IMAGE_TAG ?= localhost:5001/neo-savant:latest

# Check if the ENVIRONMENT variable is in the list of valid environments
ifeq ($(filter $(ENVIRONMENT),$(VALID_ENVIRONMENTS)),)
$(error Invalid value for ENVIRONMENT. Valid values are dev, stg, or prd.)
endif

## Build the Docker image
image/build:
	docker build --build-arg DEPLOY_ENV=${ENVIRONMENT} -t "${IMAGE_TAG}" .

## Push the Docker image
image/push:
	docker push "${IMAGE_TAG}"
