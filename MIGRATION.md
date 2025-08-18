# Migration from Makefile to Justfile

This document explains the conversion of the project's build system from Make to Just, and how to use the new development workflow.

## Why Just?

We migrated from Make to [Just](https://github.com/casey/just) for several reasons:

- **Better syntax**: Just uses a cleaner, more intuitive syntax
- **Cross-platform**: Works consistently across macOS, Linux, and Windows
- **Better error messages**: More helpful feedback when commands fail
- **Modern features**: Built-in parameter handling, better variable support
- **Rust ecosystem**: Fits well with Rust development workflows

## Installation

Install Just using cargo:

```bash
cargo install just
```

Or using your system package manager:

```bash
# macOS
brew install just

# Ubuntu/Debian
sudo apt install just

# Arch Linux
sudo pacman -S just
```

## Command Mapping

Here's how the old Makefile commands map to the new Justfile:

### Basic Operations

| Makefile | Justfile | Description |
|----------|----------|-------------|
| `make help` | `just help` or `just` | Show available commands |
| `make build` | `just build` | Build the library |
| `make test` | `just test` | Run tests |
| `make clean` | `just clean` | Clean build artifacts |

### Development

| Makefile | Justfile | Description |
|----------|----------|-------------|
| `make examples` | `just examples` | Build all examples |
| `make run-basic` | `just run-simple` | Run simple example |
| `make run-advanced` | `just run-advanced` | Run advanced example |
| `make dev-server` | `just dev` | Start dev server |
| `make watch` | `just watch` | Watch for changes |

### Code Quality

| Makefile | Justfile | Description |
|----------|----------|-------------|
| `make check` | `just check` | Run clippy and format checks |
| `make fmt` | `just fmt` | Format code |
| `make clippy` | `just clippy` | Run clippy |
| `make lint` | `just lint` | Run all linting tools |
| `make pre-commit` | `just pre-commit` | Run all pre-commit checks |

### Advanced Operations

| Makefile | Justfile | Description |
|----------|----------|-------------|
| `make install-deps` | `just install-deps` | Install development dependencies |
| `make setup` | `just setup` | Complete development setup |
| `make verify` | `just verify` | Verify library works correctly |
| `make generate-icons` | `just generate-icons` | Regenerate icon components |

## New Features

The Justfile includes several improvements over the old Makefile:

### 1. Better Example Handling

```bash
# Build specific examples for production
just build-simple
just build-advanced

# Test example compilation without running
just test-examples

# Run examples on different ports
just run-simple    # localhost:8080
just run-advanced  # localhost:8081
```

### 2. Enhanced Development Workflow

```bash
# Quick development cycle
just dev-cycle

# Full development workflow
just dev-full

# Watch and run tests automatically
just watch-test
```

### 3. Improved Tool Management

```bash
# Check for outdated dependencies
just outdated

# Generate test coverage
just coverage

# Audit dependencies for security issues
just audit
```

### 4. Better Release Management

```bash
# Prepare for release
just release-prep

# Release with different version bumps
just release-patch
just release-minor
just release-major
```

## Working Examples

The migration also fixed several issues with the examples:

### Fixed Issues

1. **Missing dependencies**: Added proper `Cargo.toml` for examples
2. **Compilation errors**: Fixed import issues and dependencies
3. **Missing HTML files**: Created proper `index.html` and `advanced.html`
4. **Styling**: Added comprehensive CSS for the advanced example

### Example Structure

```
examples/
├── Cargo.toml              # Example dependencies
├── index.html              # HTML for simple example
├── advanced.html           # HTML for advanced example
├── simple.rs               # Basic icon usage
└── advanced_styling.rs     # Advanced styling examples
```

### Running Examples

The examples now compile and run successfully:

```bash
# Simple example with basic icon usage
just run-simple

# Advanced example with styling, animations, and interactions
just run-advanced

# Build examples for production deployment
just build-simple
just build-advanced
```

## Migration Checklist

If you're migrating your own development workflow:

- [ ] Install Just: `cargo install just`
- [ ] Remove old Makefile habits, use `just` instead of `make`
- [ ] Update CI/CD scripts to use `just` commands
- [ ] Update documentation references from `make` to `just`
- [ ] Use `just --list` to see all available commands
- [ ] Take advantage of new commands like `just dev-cycle` and `just verify`

## Key Differences

### Syntax

**Makefile:**
```makefile
build: ## Build the library
	@echo "Building leptos-lucide..."
	cargo build
```

**Justfile:**
```just
# Build the library
build:
    @echo "Building leptos-lucide..."
    cargo build
```

### Variables and Parameters

**Makefile:**
```makefile
PORT ?= 8080
run-server:
	trunk serve --port $(PORT)
```

**Justfile:**
```just
# Run with custom port
run-server port="8080":
    trunk serve --port {{port}}
```

### Dependencies

**Makefile:**
```makefile
examples: build
	cd examples && cargo build
```

**Justfile:**
```just
# Build all examples
examples: build
    @echo "Building examples..."
    cd examples && cargo build --bin simple
    cd examples && cargo build --bin advanced_styling
```

## Troubleshooting

### Common Issues

1. **Just not found**: Make sure Just is installed and in your PATH
2. **Command not working**: Use `just --list` to see available commands
3. **Examples not compiling**: Run `just test-examples` to check compilation
4. **Icons not generating**: Run `just generate-icons` to force regeneration

### Getting Help

```bash
# List all available commands
just --list

# Show detailed help
just help

# Run verification to check everything works
just verify
```

## Benefits of the Migration

1. **Cleaner syntax**: More readable and maintainable build scripts
2. **Better error handling**: Clearer error messages when things go wrong
3. **Cross-platform consistency**: Works the same on all operating systems
4. **Modern tooling**: Integrates better with modern development workflows
5. **Working examples**: Examples now compile and run out of the box
6. **Enhanced development experience**: More comprehensive command set

## Next Steps

1. Start using `just` commands in your daily development
2. Explore the new commands like `just dev-cycle` and `just watch-test`
3. Customize the Justfile for your specific needs
4. Update any scripts or documentation that referenced the old Makefile commands

The migration to Just provides a more robust, user-friendly development experience while maintaining all the functionality of the original Makefile.