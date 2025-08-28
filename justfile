# Leptos Lucide Tree Icons - Development Justfile
# Run `just` or `just help` to see available commands

# Default recipe shows help
default:
    @just help

# Show all available commands
help:
    @echo "Available commands:"
    @just --list

# Install development dependencies
install-deps:
    @echo "Installing development dependencies..."
    rustup component add rustfmt clippy
    cargo install trunk wasm-bindgen-cli

# Build the library
build:
    @echo "Building leptos-lucide-rs..."
    cargo build

# Build the library in release mode
build-release:
    @echo "Building leptos-lucide-rs (release)..."
    cargo build --release

# Force regeneration of icon components
generate-icons:
    @echo "Regenerating icon components..."
    cargo clean
    cargo build

# Run all tests
test:
    @echo "Running tests..."
    cargo test

# Run tests with verbose output
test-verbose:
    @echo "Running tests (verbose)..."
    cargo test -- --nocapture

# Run clippy and format checks
check:
    @echo "Running clippy..."
    cargo clippy -- -D warnings
    @echo "Checking formatting..."
    cargo fmt --check

# Format code
fmt:
    @echo "Formatting code..."
    cargo fmt

# Run clippy
clippy:
    @echo "Running clippy..."
    cargo clippy -- -D warnings

# Build all examples
examples: build
    @echo "Building examples..."
    cd examples && cargo build --bin simple
    cd examples && cargo build --bin advanced_styling

# Run simple example with trunk
run-simple: build
    @echo "Running simple example on http://localhost:8080..."
    cd examples && trunk serve index.html --port 8080 --open

# Run advanced styling example with trunk
# run-advanced: build
#     @echo "Running advanced styling example on http://localhost:8081..."
#     cd examples && trunk serve advanced.html --port 8081 --open

# Build simple example for production
build-simple: build
    @echo "Building simple example for production..."
    cd examples && trunk build index.html --release

# Build advanced example for production
# build-advanced: build
#     @echo "Building advanced example for production..."
#     cd examples && trunk build advanced.html --release

# Generate documentation
docs:
    @echo "Generating documentation..."
    cargo doc --no-deps --open

# Generate documentation including private items
docs-private:
    @echo "Generating documentation (including private items)..."
    cargo doc --no-deps --document-private-items --open

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    rm -rf target/
    rm -rf examples/dist/

# Run benchmarks
bench:
    @echo "Running benchmarks..."
    cargo bench

# Start development server for simple example
dev: build
    @echo "Starting development server for simple example..."
    cd examples && trunk serve index.html --port 8080

# Start development server for advanced example
dev-advanced: build
    @echo "Starting development server for advanced example..."
    cd examples && trunk serve advanced.html --port 8081

# Watch for changes and rebuild
watch:
    @echo "Watching for changes and rebuilding..."
    cargo watch -x build

# Watch for changes and run tests
watch-test:
    @echo "Watching for changes and running tests..."
    cargo watch -x test

# Audit dependencies for security vulnerabilities
audit:
    @echo "Auditing dependencies..."
    cargo audit

# Update dependencies
update:
    @echo "Updating dependencies..."
    cargo update

# Check for outdated dependencies (requires cargo-outdated)
outdated:
    @echo "Checking for outdated dependencies..."
    cargo outdated

# Generate test coverage report (requires cargo-tarpaulin)
coverage:
    @echo "Generating coverage report..."
    cargo tarpaulin --out Html --output-dir coverage

# Run all linting tools
lint:
    @echo "Running all linting tools..."
    just clippy
    just fmt
    @echo "Linting complete!"

# Check build on different targets
check-targets:
    @echo "Checking WASM target..."
    cargo check --target wasm32-unknown-unknown
    @echo "Checking native target..."
    cargo check

# Run performance tests
perf:
    @echo "Running performance tests..."
    cargo test --release -- --ignored perf

# Generate list of available icons
icon-list:
    @echo "Generating icon list..."
    @cargo build 2>&1 | grep -o "Generated [0-9]* icons" || echo "Build to see icon count"

# Validate generated icon code
validate:
    @echo "Validating generated code..."
    cargo check --tests

# Setup complete development environment
setup: install-deps
    @echo "Setting up development environment..."
    @echo "Installing additional tools..."
    -cargo install cargo-watch
    -cargo install cargo-audit
    -cargo install cargo-outdated
    -cargo install cargo-tarpaulin
    @echo "Setup complete! Run 'just help' to see available commands."

# Run all pre-commit checks
pre-commit:
    @echo "Running pre-commit checks..."
    just fmt
    just clippy
    just test
    just check-targets
    @echo "All checks passed! ✅"

# Quick development build and test
dev-cycle:
    @echo "Quick development cycle..."
    cargo build && cargo test

# Verify the library works correctly
verify:
    @echo "Verifying library..."
    just build
    just test
    just examples
    @echo "Library verification complete! ✅"

# Release a patch version (requires cargo-release)
release-patch:
    @echo "Releasing patch version..."
    cargo release patch --execute

# Release a minor version (requires cargo-release)
release-minor:
    @echo "Releasing minor version..."
    cargo release minor --execute

# Release a major version (requires cargo-release)
release-major:
    @echo "Releasing major version..."
    cargo release major --execute

# Build and run both examples
examples-all: build
    @echo "Building all examples..."
    just build-simple
    just build-advanced
    @echo "All examples built successfully!"

# Test examples compilation
test-examples: build
    @echo "Testing example compilation..."
    cd examples && cargo check --bin simple
    cd examples && cargo check --bin advanced_styling
    @echo "Examples compile successfully!"

# Full development workflow
dev-full: clean build test examples
    @echo "Full development workflow complete!"

# Prepare for release
release-prep: clean pre-commit docs
    @echo "Release preparation complete!"
    @echo "Ready for release! 🚀"
