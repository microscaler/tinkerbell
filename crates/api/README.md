# API Crate

This crate exposes the gRPC interface used by the Tiffany daemon. The server provides three simple methods:

- `Status` – returns a basic health check message
- `TaskSubmit` – accepts a task id and returns an acknowledgement
- `Ping` – lightweight keepalive

An optional REST proxy is available behind the `rest` feature flag and listens on the port immediately following the gRPC port.
