.PHONY: help install deps compile test clean generate all

# Default target
all: install generate compile test

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

install: ## Install all dependencies
	@echo "Installing dependencies..."
	@which cargo > /dev/null || (echo "Error: Rust/Cargo not installed" && exit 1)
	@which openapi-generator-cli > /dev/null || npm install -g @openapitools/openapi-generator-cli
	@echo "Dependencies installed"

deps: install ## Alias for install

generate: ## Generate Rust client from OpenAPI spec
	@echo "Generating Rust client..."
	openapi-generator-cli generate \
		-i ctg-oas-v2.yaml \
		-g rust \
		-o . \
		--additional-properties=packageName=clinicaltrials-gov-api
	@echo "Client generated"

build: ## Build the project
	@echo "Compiling..."
	cargo build
	@echo "Compilation complete"

test: ## Run tests
	@echo "Running tests..."
	cargo test
	@echo "Tests complete"

check: ## Run cargo check
	@echo "Running cargo check..."
	cargo check

fmt: ## Format code
	@echo "Formatting code..."
	cargo fmt

clippy: ## Run clippy linter
	@echo "Running clippy..."
	cargo clippy

clean: ## Clean build artifacts
	@echo "Cleaning..."
	cargo clean
	@echo "Clean complete"

dev: generate compile ## Quick development build

ci: install generate compile test ## Full CI pipeline
