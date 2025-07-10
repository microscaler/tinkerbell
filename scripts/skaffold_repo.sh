#!/bin/bash

set -e

WORKSPACE_NAME="tiffany"
CRATES=(
  core
  api
  cli
  executor
  scheduler
  canvas
  wal
  logging
  metrics
  graphdb
  router
  plugins
  daemon
)

echo "Creating Rust workspace: $WORKSPACE_NAME"
mkdir -p $WORKSPACE_NAME/{crates,tools,docs,tests,scripts}
cd $WORKSPACE_NAME

echo "Initializing workspace Cargo.toml"
cat <<EOF > Cargo.toml
[workspace]
members = [
$(for c in "${CRATES[@]}"; do echo "  \"crates/$c\","; done)
]
resolver = "2"
EOF

touch Cargo.lock .gitignore .env README.md Makefile justfile

echo "Creating crates..."
for CRATE in "${CRATES[@]}"; do
  CRATE_PATH="crates/$CRATE"
  mkdir -p $CRATE_PATH/src

  # Choose lib.rs or main.rs
  if [[ "$CRATE" == "api" || "$CRATE" == "cli" || "$CRATE" == "daemon" ]]; then
    cat <<EOF > $CRATE_PATH/src/main.rs
fn main() {
    println!("$CRATE starting...");
}
EOF
  else
    cat <<EOF > $CRATE_PATH/src/lib.rs
// $CRATE library module
EOF
  fi

  # Create Cargo.toml
  cat <<EOF > $CRATE_PATH/Cargo.toml
[package]
name = "$CRATE"
version = "0.1.0"
edition = "2024"

[dependencies]
EOF

  touch $CRATE_PATH/README.md
done

echo "Creating placeholder folders for tools and docs..."
mkdir -p tools/crawl4ai_adapter tools/github_tool
mkdir -p docs/diagrams

echo "Creating placeholder integration tests..."
cat <<EOF > tests/full_pipeline.rs
// Full pipeline integration test stub
#[test]
fn full_pipeline_works() {
    assert!(true);
}
EOF

cat <<EOF > tests/api.rs
// API integration test stub
#[test]
fn api_starts() {
    assert!(true);
}
EOF

echo "Creating example script..."
cat <<EOF > scripts/start.sh
#!/bin/bash
echo "Starting tiffany daemon..."
cargo run -p daemon
EOF

chmod +x scripts/start.sh

echo "🎉 Workspace $WORKSPACE_NAME initialized successfully!"
