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

# Build release binary for Linux (native)
build-linux:
    cargo build --release

# Build release binary for Windows (cross)
build-windows:
    rustup target add x86_64-pc-windows-gnu || true
    cargo build --release --target x86_64-pc-windows-gnu

# Build release binary for macOS (cross/native)
build-macos:
    rustup target add x86_64-apple-darwin || true
    cargo build --release --target x86_64-apple-darwin

# Build release APK for Android
build-android:
    rustup target add aarch64-linux-android || true
    cargo install cargo-apk || true
    cargo apk build --release --lib

# Build release IPA for iOS
build-ios:
    rustup target add aarch64-apple-ios || true
    cargo build --release --target aarch64-apple-ios --lib

# Format code
fmt:
    cargo fmt --all

# Bump version and create release commit
bump-version VERSION:
    @echo "Bumping version to {{VERSION}}..."
    @# Update Cargo.toml (workspace package version)
    @sed -i 's/^version = "[0-9.]\+"/version = "{{VERSION}}"/' Cargo.toml
    @# Update Cargo.lock
    @cargo check > /dev/null 2>&1 || true
    @git add Cargo.toml Cargo.lock
    @git commit -m "release: bump version to {{VERSION}}"
    @echo "Bumped version to {{VERSION}}"

# Clippy lint
lint:
    cargo clippy -- -Dwarnings

# Clean build artifacts
clean:
    cargo clean
