# Top-level build of all crates
build:
    cargo build --workspace

# Build in release mode
build-release:
    cargo build --workspace --release

# Run all tests
test:
    cargo test --workspace

# Build only docbookgen (used for mdBook + RustDoc)
build-docbookgen:
    cargo build --package docbookgen

# Build documentation for all crates (excluding private/no-docs)
docs:
    cargo doc --workspace --no-deps --all-features

# Clean workspace
clean:
    cargo clean

# Build mdBook structure using docbookgen
docbookgen:
    cargo run --package docbookgen -- build ./docs/mdbook/

# Serve the mdBook locally
serve-docs:
    mdbook serve ./docs/mdbook/ --open

# Rebuild book after content changes
build-docsbook:
    mdbook build ./docs/mdbook/
