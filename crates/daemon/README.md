# Tinkerbell Daemon

The `tinkerbell` binary — built from this crate — is the long-running process that powers a Tinkerbell agent inside a container or microVM.

It is responsible for:

- Bootstrapping the runtime environment
- Starting the coroutine-based scheduler loop
- Initializing logging, tracing, metrics, and memory
- Handling process signals (e.g., shutdown, restart)
- Watching the filesystem for config changes (optional)

> It **does not** process user input or handle REST/gRPC traffic directly. That functionality is delegated to other internal crates like `api`, `scheduler`, and `reasonact`.

---

## 🧱 Responsibilities

| Component         | Description                                                  |
|------------------|--------------------------------------------------------------|
| `main.rs`        | Initializes logging, loads config, enters main runtime loop  |
| `Scheduler`      | Spawned once at startup; manages task execution              |
| `PAL/WAL`        | Initialized for tracing and replayable logging               |
| `Signal Handler` | Catches `SIGINT`, `SIGTERM`, or `SIGHUP` for lifecycle mgmt  |
| `File Watcher`   | (Planned) optional config reload support via inotify/fsevents|

---

## 🛠️ Lifecycle Flow

```mermaid
sequenceDiagram
    participant OS
    participant Daemon
    participant Scheduler
    participant Config

    OS->>Daemon: Start process
    Daemon->>Config: Load config file
    Daemon->>Scheduler: Initialize + start
    Daemon->>Daemon: Listen for shutdown/reload signals

    OS-->>Daemon: SIGINT / SIGTERM
    Daemon->>Scheduler: Graceful shutdown
    Daemon->>OS: Exit(0)
````

---

## ✨ Features

* Controlled process lifecycle (start, stop, reload)
* Clean shutdown of all tasks and background services
* Optional inotify/fsevents config hot-reloading
* Designed to run inside Firecracker or Apple container environments

---

## 📦 Project Layout

```txt
crates/daemon/
├── bin/
│   └── tinkerbell.rs         # Binary entrypoint
├── src/
│   ├── lib.rs                # Init and shutdown interfaces
│   ├── config.rs             # Config loader (TOML, JSON, etc.)
│   └── signal.rs             # Graceful signal handling
```

---

## 🧪 Development

```bash
# Run the daemon
cargo run -p daemon

# Run with debug logging
RUST_LOG=debug cargo run -p daemon

# Watch for config changes (planned)
touch config.toml
```

---

## 🔌 Integration

The daemon links together internal crates:

* [`scheduler`](../scheduler) – runs the event loop
* [`reasonact`](../reasonact) – manages task cognition
* [`canvas`](../canvas), [`wal`](../wal), [`metrics`](../metrics)

It **does not expose** public APIs; it only manages the agent runtime inside a container.

---

## 🚀 Goals

The `tinkerbell` daemon is designed to be:

* Deterministic
* Fast-starting
* Observability-first
* Cleanly shut down on container or VM exit

