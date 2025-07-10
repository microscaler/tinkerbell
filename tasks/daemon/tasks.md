# Tiffany Daemon Crate Tasks

## Goal
Build the binary entrypoint for the `tiffany` agent, responsible for:
- Config file loading
- Signal handling
- Starting the scheduler runtime loop

## Tasks

- [ ] Create `bin/tiffany.rs` to:
    - Load config
    - Initialize logging and metrics
    - Call `daemon::run()`

- [ ] In `lib.rs`, implement `init()` and `run()`
- [ ] Implement signal handling (SIGINT, SIGTERM)
- [ ] Add placeholder config loader (`config.rs`) using `serde` + `toml`
- [ ] Plan file-watcher support for hot-reload of config (inotify / fsevents)
- [ ] Add integration test to spawn the daemon and shut it down gracefully
- [ ] Replace placeholder daemon tests with a check that init and run return successfully
