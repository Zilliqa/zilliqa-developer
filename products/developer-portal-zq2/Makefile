.PHONY: all
all: image/build-and-push

.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS = -ec

ENVIRONMENT ?= dev
VALID_ENVIRONMENTS := dev stg prd

# Check if the ENVIRONMENT variable is in the list of valid environments
ifeq ($(filter $(ENVIRONMENT),$(VALID_ENVIRONMENTS)),)
$(error Invalid value for ENVIRONMENT. Valid values are dev, stg, or prd.)
endif


HERE=$(shell pwd)

DOC_SOURCE=$(HERE)/docs
export DOC_SOURCE
VERSION=$(shell cat $(DOC_SOURCE)/../VERSION)
export VERSION

dev:
	mkdocs serve

BINDIR=$(HERE)/obj
HERE_FILES=Dockerfile mkdocs.yaml overrides requirements.txt

# Sadly necessary because we need the docs from .., and using that as the dockerfile context
# would be a whole world of pain.
.PHONY: assemble build
assemble:
	rm -rf $(BINDIR)
	mkdir -p $(BINDIR)
	cp -r $(HERE_FILES) $(BINDIR)
	cp -r $(DOC_SOURCE) $(BINDIR)

IMAGE_TAG ?= zq2-docs:latest

build:
	docker buildx build --build-arg VERSION="${VERSION}" . -t $(IMAGE_TAG)

run-image: build
	docker run --rm -p 8080:80 "$(IMAGE_TAG)"

STG_TAG=asia-docker.pkg.dev/prj-d-devops-services-4dgwlsse/zilliqa-public/developer-portal-zq2
## This is an ugly hack - it allows me to push directly to the staging repo without going through CI - rrw 2024-04-16
push-staging-image: build
	docker tag "${IMAGE_TAG}" "${STG_TAG}"
	docker push "${STG_TAG}"
	echo Now restart the pod .. 

## Build and push the Docker image
image/build-and-push: build
	docker push "$(IMAGE_TAG)"
