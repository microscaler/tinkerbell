#!/usr/bin/env bash

set -e

DOCS_OUT_DIR="docsbook"

echo "📚 Generating Rust workspace documentation..."

# Ensure target doc output is clean
rm -rf "$DOCS_OUT_DIR"
mkdir -p "$DOCS_OUT_DIR"

# Generate the documentation
cargo doc --workspace --no-deps

# Copy it to the versioned docsbook folder
cp -r target/doc/* "$DOCS_OUT_DIR/"

echo "✅ Documentation generated in $DOCS_OUT_DIR/"

# Optional: open in browser (comment out if CI or headless)
if [[ "$1" != "--no-open" ]]; then
  if command -v xdg-open &> /dev/null; then
    xdg-open "$DOCS_OUT_DIR/index.html"
  elif command -v open &> /dev/null; then
    open "$DOCS_OUT_DIR/index.html"
  else
    echo "⚠️ Could not auto-open browser. Open $DOCS_OUT_DIR/index.html manually."
  fi
fi
