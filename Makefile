.PHONY: help install deps compile test clean generate all examples

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

examples: ## Run example programs
	@echo "Available examples:"
	@echo "  financial_analyst_medical_devices - Financial analyst searching medical device trials"
	@echo "  stock_catalyst_tracker - Track upcoming clinical trial catalysts for stock analysis"
	@echo ""
	@echo "Run with: make run-example EXAMPLE=<example_name>"

run-example: ## Run a specific example (use EXAMPLE=name)
	@if [ -z "$(EXAMPLE)" ]; then \
		echo "Error: Please specify an example name with EXAMPLE=<name>"; \
		echo "Available examples:"; \
		echo "  financial_analyst_medical_devices"; \
		echo "  stock_catalyst_tracker"; \
		exit 1; \
	fi
	@echo "Running example: $(EXAMPLE)"
	cd examples && cargo run --bin $(EXAMPLE)
