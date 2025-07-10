# API Integration Tasks

## Goal
Integrate the gRPC/REST `api` crate into the daemon so that:
- The `tctl` CLI can communicate with a running agent
- Incoming requests are routed to `reasonact`, `canvas`, or task scheduler

## Tasks

- [ ] Move `api/src/main.rs` logic into a new `lib.rs`
- [ ] Define a `start_api_server()` function returning a `JoinHandle` or `Future`
- [ ] In `daemon/bin/tiffany.rs`, call `api::start_api_server()` alongside the scheduler boot
- [ ] Define a simple gRPC service for `Status`, `TaskSubmit`, or `Ping`
- [ ] Add optional REST proxy with `axum` or `warp`
- [ ] Ensure API shutdown is triggered by signal from `daemon::signal` module
