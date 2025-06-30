Below is a **Phase 3 task bundle** that will bring the scheduler up to full **PyOS8-level I/O parity**.
Copy each markdown file into `tasks/scheduler/…` so Codex (or any contributor) can implement them in order.





### 🔖 How to Use

1. Drop each file under `tasks/scheduler/`.
2. Triage them in PR order:

  
  2. `io_event_loop`
  3. `cancel_timeout`
  4. `priority_queue`
  5. `panic_isolation`
3. Run `cargo nextest` with and without `--features async-io` for full coverage.

With these implemented Tiffany’s scheduler will match (and exceed) PyOS8’s capabilities in cooperative stepping, virtual time, real I/O readiness, cancellation, and priority handling.
