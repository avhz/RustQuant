
# Makefile for common tasks in a Rust project

# Default target
.PHONY: all
all: build

# Build the project
.PHONY: build
build:
	cargo build

# Run tests
.PHONY: test
test:
	cargo test

# Format the code
.PHONY: fmt
fmt:
	cargo +nightly fmt --all

# Check formatting
.PHONY: fmt-check
fmt-check:
	cargo +nightly fmt --check

# Run Clippy for linting
.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Clean the project
.PHONY: clean
clean:
	cargo clean

# Pre-push checks
.PHONY: pre-push
pre-push: test fmt-check lint

# Run the project
.PHONY: run
run:
	cargo run
