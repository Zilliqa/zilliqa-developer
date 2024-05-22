.PHONY: all
all: image/build-and-push

.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS = -ec

ENVIRONMENT ?= dev
IMAGE_TAG ?= localhost:5001/zilliqa-bridge-web:latest

decrypt-value-files-stg:
ifeq ($(ENVIRONMENT), stg)
	openssl aes-256-cbc -pbkdf2 -k "${ENV_FILES_DECRYPTER_NONPRD}" -in "./infra/environment/staging/.env.enc" -out "./infra/environment/staging/.env" -d
endif

decrypt-value-files-prod:
ifeq ($(ENVIRONMENT), prd)
	openssl aes-256-cbc -pbkdf2 -k "${ENV_FILES_DECRYPTER_PRD}" -in "./infra/environment/production/.env.enc" -out "./infra/environment/production/.env" -d
endif

## Build and push the Docker image
image/build-and-push:
	if [[ "$$ENVIRONMENT" == "prd" ]] ; then \
		docker build --build-arg VITE_ENV="PROD" -t "${IMAGE_TAG}" .; \
	else \
		docker build -t "${IMAGE_TAG}" .; \
	fi
	docker push "${IMAGE_TAG}"