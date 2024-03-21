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

dev:
	DOC_SOURCE=`pwd`/../../docs-zq2 mkdocs serve

## Build and push the Docker image
image/build-and-push:
	@echo nothing yet.
