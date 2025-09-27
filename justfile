# -----------------------------------------------------------------------------
#  Flo Project Command Runner
#  Run `just` to see a list of available commands.
# -----------------------------------------------------------------------------

# --- Primary Development Commands ---

# The master command to run all quality checks before a Pull Request.
check: fmt lint test-all
    @echo "✅ All checks passed! You're ready to create a PR. 🦊"

# Format all code in the workspace.
fmt:
    @echo "📝 Formatting code with rustfmt..."
    cargo fmt --all

# Lint all code in the workspace with Clippy.
lint:
    @echo "🧐 Linting code with Clippy..."
    cargo clippy --workspace --all-targets -- -D warnings

# Run all tests, including fast unit tests and slow integration tests.
test-all:
    @echo "🧪 Running the complete test suite (unit + integration)..."
    cargo test --workspace --features test-integration

# Run only the fast unit tests.
test-fast:
    @echo "🧪 Running fast unit tests..."
    cargo test --workspace

# Build the entire workspace in debug mode.
build:
    @echo "🔨 Building workspace..."
    cargo build --workspace

# Build the entire workspace in release mode for production.
build-release:
    @echo "🚀 Building workspace in release mode..."
    cargo build --workspace --release

# Clean the build artifacts.
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean

# Run the Flo binary with the given arguments.
# Example: `just run -- feature start my-feature`
run ARGS='':
    @echo "▶️  Running flo {{ARGS}}..."
    cargo run -p flo -- {{ARGS}}