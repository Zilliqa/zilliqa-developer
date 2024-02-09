GOBASE = $(shell pwd)
GOBIN = $(GOBASE)/build

.PHONY: test cover deps build start

.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS = -ec

ENVIRONMENT ?= dev
VALID_ENVIRONMENTS := dev stg prd
IMAGE_TAG ?= localhost:5001/scilla-server:latest

# Check if the ENVIRONMENT variable is in the list of valid environments
ifeq ($(filter $(ENVIRONMENT),$(VALID_ENVIRONMENTS)),)
$(error Invalid value for ENVIRONMENT. Valid values are dev, stg, or prd.)
endif

deps:
	go mod download

test:
	go test ./... -cover -covermode=atomic -coverprofile=coverage.txt

cover:
	go tool cover -html=coverage.txt

build:
	CGO_ENABLED=0 GOOS=linux go build -o $(GOBIN)/faucet-service ./cmd

start:
	docker build -t faucet-service . -f Dockerfile && docker run -p 8080:8080 --env-file ./.env faucet-service

## Build and push the Docker image
image/build-and-push:
	docker build --build-arg DEPLOY_ENV=${ENVIRONMENT} -t "${IMAGE_TAG}" .
	docker push "${IMAGE_TAG}"
