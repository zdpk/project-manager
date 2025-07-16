# PM (Project Manager) Makefile
# Provides convenient commands for building and running production and development versions

.PHONY: build-prod build-dev run-prod run-dev clean install-prod install-dev help test

# Default target
help:
	@echo "PM (Project Manager) Build Commands:"
	@echo ""
	@echo "Building:"
	@echo "  make build-prod    - Build production binary (pm)"
	@echo "  make build-dev     - Build development binary (_pm)"
	@echo "  make build-all     - Build both binaries"
	@echo ""
	@echo "Running:"
	@echo "  make run-prod      - Run production binary"
	@echo "  make run-dev       - Run development binary"
	@echo ""
	@echo "Installing:"
	@echo "  make install-prod  - Install production binary"
	@echo "  make install-dev   - Install development binary"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make test          - Run tests"
	@echo ""
	@echo "Examples:"
	@echo "  make run-prod -- init         # Run 'pm init'"
	@echo "  make run-dev -- init          # Run '_pm init' (development binary)"
	@echo "  make run-prod -- add /path    # Run 'pm add /path'"

# Build commands
build-prod:
	@echo "🔨 Building production binary..."
	cargo build --bin pm --release
	@echo "✅ Production binary built: target/release/pm"

build-dev:
	@echo "🔨 Building development binary..."
	cargo build --bin _pm
	@echo "✅ Development binary built: target/debug/_pm"

build-all: build-prod build-dev

# Run commands
run-prod:
	@echo "🚀 Running production binary..."
	cargo run --bin pm -- $(filter-out $@,$(MAKECMDGOALS))

run-dev:
	@echo "🚀 Running development binary..."
	cargo run --bin _pm -- $(filter-out $@,$(MAKECMDGOALS))

# Install commands
install-prod:
	@echo "📦 Installing production binary..."
	cargo install --path . --bin pm --force
	@echo "✅ Production binary installed as 'pm'"

install-dev:
	@echo "📦 Installing development binary..."
	cargo install --path . --bin _pm --force
	@echo "✅ Development binary installed as '_pm'"

# Test and maintenance
test:
	@echo "🧪 Running tests..."
	cargo test

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean complete"

# Allow extra arguments to be passed to run commands
%:
	@: