# MyHome Justfile

# Default task: list all commands
default:
    @just --list

# Drop into the Nix development shell
shell:
    nix develop

# Run the project
dev:
    cargo run --bin myhome

# Type-check the whole workspace (or project)
check:
    cargo check

# Run all tests
test:
    cargo test

# Build release binary
build-release:
    cargo build --release

# Format code
fmt:
    cargo fmt

# Clippy lint
lint:
    cargo clippy -- -Dwarnings

# Clean build artifacts
clean:
    cargo clean
