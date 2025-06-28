# CLI → Daemon Integration

## Goal
Ensure the `tctl` CLI can:
- Send structured gRPC requests to the running daemon
- Display output clearly (human or JSON)

## Tasks

- [ ] In `cli/src/commands/status.rs`, add logic to connect via gRPC to the `api` crate
- [ ] Add CLI transport abstraction in `transport.rs` to select between Unix socket and TCP
- [ ] Implement basic `tctl status` command that queries `/status` endpoint
- [ ] Add unit test that mocks an API response and verifies output formatting
- [ ] Extend CLI help output with `--json`, `--plain` flags
- [ ] Replace the placeholder tests in `cli/tests` with minimal integration tests for argument parsing
