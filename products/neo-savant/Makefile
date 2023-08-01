.PHONY: all
all: help

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

# Ensures that a variable is defined and non-empty
define assert-set
	@$(if $($(1)),,$(error $(1) not defined in $(@)))
endef

DEVOPS_REPO_URL := https://$(GITHUB_PAT)@github.com/Zilliqa/devops.git
TEMP_FOLDER := $(if $(DEVOPS_FOLDER),$(DEVOPS_FOLDER),.z)

devops_repo_cleanup:
	@if [ -z "$(DEVOPS_FOLDER)" ]; then \
		rm -rf $(TEMP_FOLDER); \
	fi

## Download or use the given devops repo to use z locally
z/setup: devops_repo_cleanup
	@if [ -z "$(DEVOPS_FOLDER)" ]; then \
		echo "Downloading devops Git repository..."; \
		git clone $(DEVOPS_REPO_URL) $(TEMP_FOLDER); \
		echo "Downloading completed."; \
	else \
		echo "DEVOPS_FOLDER is not empty. Using the provided folder."; \
	fi
	
	@cd $(TEMP_FOLDER) && \
	source setenv && \
	echo "Compiling z..." && \
	z help && \
	echo "z compiled."

# Generate help output
.PHONY: help
help:
	@awk '/^[-a-zA-Z_0-9%:\\\.\/]+:/ { \
		helpMessage = match(lastLine, /^## (.*)/); \
		if (helpMessage) { \
			helpCommand = $$1; \
			helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
			gsub("\\\\", "", helpCommand); \
			gsub(":+$$", "", helpCommand); \
			printf "  \x1b[32;01m%-35s\x1b[0m %s\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST) | sort -u
	@printf "\n"

## Destroy the local cluster
cluster/destroy:
	z local destroy

## Create a local cluster to deploy the application
cluster/create:
	z local create
	kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml

## Build the Docker image
image/build:
	docker build --build-arg DEPLOY_ENV=${ENVIRONMENT} -t "${IMAGE_TAG}" .

## Push the Docker image
image/push:
	docker push "${IMAGE_TAG}"

## Deploy the application to the target cluster
z/deploy:
	z app sync
