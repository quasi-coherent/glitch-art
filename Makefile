.PHONY: help build release format check
.DEFAULT_GOAL: help

export SHELL = /usr/bin/env bash

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

build:  ## Compile the project with no optimizations
	cargo build

release:  ## Compile the project with optimizations
	cargo build --release

format:  ## Format the source code
	rustfmt src/*.rs

check:  ## Run linter
	cargo clippy
