# Makefile for Rust Axum Blog Application

# Variables
CARGO = cargo
PROJECT_NAME = rust-app
BINARY_NAME = rust-app
BUILD_DIR = target
RELEASE_DIR = $(BUILD_DIR)/release
DEBUG_DIR = $(BUILD_DIR)/debug

# Default target
.PHONY: default
default: help

# Help target - displays available commands
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  build      - Build the project in debug mode"
	@echo "  build-rel  - Build the project in release mode"
	@echo "  run        - Build and run the project in debug mode"
	@echo "  run-rel    - Build and run the project in release mode"
	@echo "  install    - Build and install the binary to system"
	@echo "  clean      - Clean build artifacts"
	@echo "  test       - Run tests"
	@echo "  check      - Check code without building"
	@echo "  fmt        - Format code"
	@echo "  clippy     - Run clippy lints"
	@echo "  doc        - Generate documentation"
	@echo "  dev        - Development workflow (fmt, clippy, test, build)"
	@echo "  deploy     - Build for deployment"

# Build in debug mode
.PHONY: build
build:
	@echo "Building in debug mode..."
	$(CARGO) build

# Build in release mode
.PHONY: build-rel
build-rel:
	@echo "Building in release mode..."
	$(CARGO) build --release

# Run in debug mode
.PHONY: run
run: build
	@echo "Running in debug mode..."
	$(CARGO) run

# Run in release mode
.PHONY: run-rel
run-rel: build-rel
	@echo "Running in release mode..."
	$(RELEASE_DIR)/$(BINARY_NAME)

# Install binary to system
.PHONY: install
install: build-rel
	@echo "Installing $(BINARY_NAME) to system..."
	$(CARGO) install --path .

# Clean build artifacts
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean

# Run tests
.PHONY: test
test:
	@echo "Running tests..."
	$(CARGO) test

# Check code without building
.PHONY: check
check:
	@echo "Checking code..."
	$(CARGO) check

# Format code
.PHONY: fmt
fmt:
	@echo "Formatting code..."
	$(CARGO) fmt

# Run clippy lints
.PHONY: clippy
clippy:
	@echo "Running clippy..."
	$(CARGO) clippy -- -D warnings

# Generate documentation
.PHONY: doc
doc:
	@echo "Generating documentation..."
	$(CARGO) doc --open

# Development workflow
.PHONY: dev
dev: fmt clippy test build
	@echo "Development workflow completed successfully!"

# Build for deployment
.PHONY: deploy
deploy: build-rel
	@echo "Deployment build completed!"
	@echo "Binary location: $(RELEASE_DIR)/$(BINARY_NAME)"

# Watch for changes and rebuild (requires cargo-watch)
.PHONY: watch
watch:
	@echo "Watching for changes..."
	@if command -v cargo-watch >/dev/null 2>&1; then \
		cargo watch -x run; \
	else \
		echo "cargo-watch not found. Install with: cargo install cargo-watch"; \
	fi

# Check for outdated dependencies (requires cargo-outdated)
.PHONY: outdated
outdated:
	@echo "Checking for outdated dependencies..."
	@if command -v cargo-outdated >/dev/null 2>&1; then \
		cargo outdated; \
	else \
		echo "cargo-outdated not found. Install with: cargo install cargo-outdated"; \
	fi

# Security audit (requires cargo-audit)
.PHONY: audit
audit:
	@echo "Running security audit..."
	@if command -v cargo-audit >/dev/null 2>&1; then \
		cargo audit; \
	else \
		echo "cargo-audit not found. Install with: cargo install cargo-audit"; \
	fi

# Create release package
.PHONY: package
package: build-rel
	@echo "Creating release package..."
	mkdir -p dist
	cp $(RELEASE_DIR)/$(BINARY_NAME) dist/
	cp Cargo.toml dist/
	cp README.md dist/ 2>/dev/null || echo "README.md not found, skipping..."
	tar -czf $(PROJECT_NAME)-$(shell grep '^version = ' Cargo.toml | cut -d '"' -f 2).tar.gz -C dist .
	@echo "Package created successfully!"

# Docker build (requires Docker)
.PHONY: docker-build
docker-build:
	@echo "Building Docker image..."
	@if [ -f Dockerfile ]; then \
		docker build -t $(PROJECT_NAME):latest .; \
	else \
		echo "Dockerfile not found. Skipping Docker build."; \
	fi

# Show project information
.PHONY: info
info:
	@echo "Project Information:"
	@echo "  Name: $(PROJECT_NAME)"
	@echo "  Version: $(shell grep '^version = ' Cargo.toml | cut -d '"' -f 2)"
	@echo "  Rust version: $(shell rustc --version)"
	@echo "  Cargo version: $(shell cargo --version)"