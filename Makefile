.PHONY: build deploy test clean format lint

# Build all Lambda functions for AWS (ARM64)
build:
	@echo "Building all Lambda functions for AWS ARM64..."
	cargo lambda build --release --arm64

# Build specific Lambda function
build-%:
	@echo "Building $* Lambda function..."
	cargo lambda build --release --arm64 -p $*-service

# Deploy using AWS SAM
deploy:
	@echo "Deploying to AWS..."
	sam build
	sam deploy

# Deploy with guided setup (first time)
deploy-guided:
	@echo "Deploying to AWS (guided)..."
	sam build
	sam deploy --guided

# Run all tests
test:
	@echo "Running tests..."
	cargo test

# Run tests for specific service
test-%:
	@echo "Testing $* service..."
	cargo test -p $*-service

# Start local API Gateway
local:
	@echo "Starting local API Gateway..."
	cargo lambda build
	sam local start-api

# Invoke specific function locally
invoke-%:
	@echo "Invoking $* function locally..."
	cargo lambda invoke $*-service --data-file events/test-$*.json

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf .aws-sam

# Format code
format:
	@echo "Formatting code..."
	cargo fmt --all

# Lint code
lint:
	@echo "Running Clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

# Check code without building
check:
	@echo "Checking code..."
	cargo check --all-targets

# Watch and rebuild on changes
watch:
	@echo "Watching for changes..."
	cargo watch -x "lambda build"

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --no-deps --open

# Install required tools
install-tools:
	@echo "Installing required tools..."
	cargo install cargo-lambda
	cargo install cargo-watch
	pip install aws-sam-cli

# Setup local environment
setup:
	@echo "Setting up local environment..."
	cp .env.example .env
	@echo "Please edit .env with your configuration"

# Run security audit
audit:
	@echo "Running security audit..."
	cargo audit

# Update dependencies
update:
	@echo "Updating dependencies..."
	cargo update

# All-in-one: format, lint, test, build
all: format lint test build
	@echo "✅ All checks passed!"

