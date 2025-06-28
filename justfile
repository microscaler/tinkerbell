# Top-level build of all crates
build:
    cargo build --workspace

# Build in release mode
build-release:
    cargo build --workspace --release

# Run all tests
test:
    cargo test --workspace

# Build only docsbookgen (used for mdBook + RustDoc)
build-docsbookgen:
    cargo build --package docsbookgen

# Build documentation for all crates (excluding private/no-docs)
docs:
    cargo doc --workspace --no-deps --all-features

# Clean workspace
clean:
    cargo clean

# Build mdBook structure using docsbookgen
docsbookgen:
    cargo run --package docsbookgen -- build ./docs/mdbook/

# Serve the mdBook locally
serve-docs:
    mdbook serve ./docs/mdbook/ --open

# Rebuild book after content changes
build-docsbook:
    mdbook build ./docs/mdbook/