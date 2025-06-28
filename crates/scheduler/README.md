# Tinkerbell Scheduler

The `scheduler` crate implements Tinkerbell’s coroutine-based cooperative task scheduler.

> ✅ This crate is a **minimal MVP** that focuses on building and testing the fundamental task management loop — before any agentic cognition is added.

---

## 🎯 MVP Objective

The scheduler should:

- Support creating and running **multiple tasks** concurrently
- Support yielding, blocking, and resuming tasks cooperatively
- Manage tasks via:
  - FIFO ready queue
  - Per-task LIFO call stack (for nested coroutines)
- Support blocking system calls like `Sleep`, `WaitTask`, `ReadWait`
- Be fully testable via deterministic simulations
- Implement a strict test harness that verifies lifecycle behavior, blocking/wakeup, and call-stack trampolining

---

## 🌀 Core Concepts

| Concept         | Description                                                   |
|-----------------|---------------------------------------------------------------|
| `Task`          | A generator-style coroutine that can yield system calls       |
| `Scheduler`     | Manages task queue, blocking map, and the main loop           |
| `SystemCall`    | An abstract yield, e.g., `Sleep`, `Spawn`, `Join`, `Log` |
| `ReadyQueue`    | FIFO queue of runnable tasks built on `VecDeque<TaskId>` |
| `CallStack`     | LIFO per-task stack for nested coroutine trampolining |
| `WaitMap`       | Tracks join/wait conditions for resumption |
| `ready_len()`   | Inspect number of tasks currently queued |

---

## 🔍 Example Flow

```rust
let mut sched = Scheduler::new();
let tid = sched.spawn(echo_loop());
sched.run();
````

Inside `echo_loop`, you might yield:

```rust
yield SystemCall::Sleep(Duration::from_secs(1));
```

The task is then moved to a timed wait queue, and resumed later by the scheduler.

---

### 🔮 Coroutine Implementation Guidance

While the initial MVP of the scheduler may use a simple `Box<dyn Generator<Yield = SystemCall, Return = ()>>` model for tasks, contributors are encouraged to evaluate **long-term strategies** based on two possible coroutine models in Rust:

#### 1. ✅ [May](https://github.com/Xudong-Huang/may) Coroutine Crate

- Mature userland stackful coroutine runtime
- No `Send + Sync` required
- Deterministic, good for local multitasking
- Pros: lightweight, proven, battle-tested
- Cons: Requires external runtime, less idiomatic with `async` ecosystems

#### 2. 🧪 Native Rust 2024 `generators`

- Stabilizing `Generator` trait in `std` as part of coroutine roadmap
- Uses built-in `yield` keyword
- Integrates well with Rust’s future `gen` support and async schedulers
- Pros: native, no external deps, future-proof
- Cons: still under stabilization in parts of the ecosystem

---

### 💡 Guiding Principle

The scheduler crate must:

- Be **agnostic to coroutine engine** (via trait or `dyn Task` abstraction)
- Document internal coroutine usage clearly
- Provide a toggle or feature flag (e.g., `--features native-gen`) if dual support emerges


---

## 🧪 Testing Framework

All tests run **deterministically**. Features under test:


* Task resumption and execution order
* Join dependencies
* Stack-based subcoroutines
* Time-based blocking (`Sleep`)
* I/O placeholder (`ReadWait`)
* Manual signal triggering to unblock tasks

Use:

```bash
cargo test -p scheduler
```

---

## ✅ MVP Checklist

* [x] Task spawning and ID tracking
* [x] FIFO ready queue
* [x] LIFO call stack with trampolining
* [x] Join/wait and requeueing
* [x] Sleep system call (with tick clock or mock timer)
* [x] Test suite for task blocking/resume logic
* [ ] I/O wait (placeholder)
* [ ] Signal/interrupt handling

---

## 🔧 Directory Layout

```txt
crates/scheduler/
├── src/
│   ├── lib.rs             # Scheduler entry point
│   ├── task.rs            # Task struct + state
│   ├── syscall.rs         # SystemCall enum and types
│   ├── ready_queue.rs     # FIFO queue
│   ├── call_stack.rs      # LIFO stack for sub-coroutines
│   ├── wait_map.rs        # Task wait conditions (join, sleep)
│   └── tests/             # Full lifecycle + system call tests
```

---

## 🧱 Non-Functional Requirements

| Characteristic | Implementation Strategy                       |
| -------------- | --------------------------------------------- |
| Deterministic  | Tasks yield values; no threads, no preemption |
| Observable     | All transitions may be traced to logs/PAL     |
| Isolated       | Tasks are logically sandboxed                 |
| Portable       | No OS dependencies; purely Rust coroutines    |
| Replayable     | Future: WAL hook for recording yields         |

---

## 🧩 Future Integrations

Once the scheduler is hardened and verified:

* Add `ReasonAct` agent loop on top
* Connect `executor` for skill invocation
* Begin streaming WAL/PAL logs for tracing

---

## Related Crates

* [`core`](../core) – runtime primitives and ID generators
* [`wal`](../wal) – planned replay integration
* [`daemon`](../daemon) – starts and owns the scheduler lifecycle

---

## 🚀 Goal

This MVP exists to **prove the core scheduler model** is robust and composable.

Until this crate has 100% test coverage on all coroutine lifecycles and blocking behaviors, no agent cognition (`reasonact`) will be integrated.

