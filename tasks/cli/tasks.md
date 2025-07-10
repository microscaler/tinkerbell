# CLI → Daemon Integration

## Goal
Ensure the `ttnyctl` CLI can:
- Send structured gRPC requests to the running daemon
- Display output clearly (human or JSON)

## Tasks

- [x] In `cli/src/commands/status.rs`, add logic to connect via gRPC to the `api` crate
- [x] Add CLI transport abstraction in `transport.rs` to select between Unix socket and TCP
- [x] Implement basic `ttnyctl status` command that queries `/status` endpoint
- [x] Add unit test that mocks an API response and verifies output formatting
- [x] Extend CLI help output with `--json`, `--plain` flags
- [x] Replace the placeholder tests in `cli/tests` with minimal integration tests for argument parsing
