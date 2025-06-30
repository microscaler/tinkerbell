> NOTE!! This task set depends on completion of virtual_clock.md

### `tasks/scheduler/io_poll.md`  *(feature-gated)*


# 🔌 Task: Wire `SystemCall::IoWait` to Event Polling (MIO)

## Context
Real agent tasks will block on sockets/FIFOs. Replace manual `io_tx` with MIO
(poll/epoll/kqueue) under `cfg(feature = "async-io")`.

## Acceptance
* With  `--features async-io`, tests use `mio::Poll` for readiness.
* Existing behaviour via in-memory channel retained when feature disabled.

---

## Steps

- [ ] **Add `mio` dependency (optional)**

  ```toml
  mio = { version = "0.8", optional = true }


* [ ] **`Scheduler` conditional field**

  ```rust
  #[cfg(feature = "async-io")]
  poll: mio::Poll,
  ```

* [ ] **Map `IoWait(io_id)` to registration**

  *Hint:* store `io_id -> Token` mapping, register FD, push to `wait_map`.

* [ ] **Poll when ready queue empty**

  ```rust
  let events = poll.poll(&mut events, timeout)?;
  for ev in events { complete_io(ev.token().into()); }
  ```

* [ ] **Feature flag guards**

  Provide stub impl (current channel path) when feature is off.

* [ ] **Add doc-test** demonstrating compile with:

  ```bash
  cargo test -p scheduler --features async-io
  ```
