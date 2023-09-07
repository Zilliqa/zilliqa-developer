.PHONY: all
all: image/build-and-push

.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS = -ec

IMAGE_TAG ?= localhost:5001/bluebell-playground:latest

## Build and push the Docker image
image/build-and-push:
	docker build -t "${IMAGE_TAG}" .
	docker push "${IMAGE_TAG}"