# Top-level build of all crates
build:
    export RUSTFLAGS="-Zmacro-backtrace --cfg tokio_unstable"  cargo build --workspace

# Build in release mode
build-release:
    export RUSTFLAGS="-Zmacro-backtrace --cfg tokio_unstable"  cargo build --workspace --release

# Run all tests
test:
    export RUSTFLAGS="-Zmacro-backtrace --cfg tokio_unstable"  cargo test --workspace

# Build only docbookgen (used for mdBook + RustDoc)
build-docbookgen:
    cargo build --package docbookgen

# Build documentation for all crates (excluding private/no-docs)
docs:
    cargo doc --workspace --no-deps --all-features

# Clean workspace
clean:
    export RUSTFLAGS="-Zmacro-backtrace --cfg tokio_unstable" cargo clean

# Build mdBook structure using docbookgen
docbookgen:
    cargo run --package docbookgen -- build ./docs/mdbook/

# Serve the mdBook locally
serve-docs:
    mdbook serve ./docs/mdbook/ --open

# Rebuild book after content changes
build-docsbook:
    mdbook build ./docs/mdbook/
