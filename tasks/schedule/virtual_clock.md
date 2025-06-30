> NOTE!! This task set depends on completion of implementing_yield.m

### `tasks/scheduler/virtual_clock.md`


# ⏱️ Task: Integrate `TickClock` and Min-Heap Sleepers

## Context
Hard `std::thread::sleep` makes tests slow/nondeterministic.  We will replace it
with virtual time via `TickClock` and a min-heap (`BinaryHeap`) of sleepers.

## Acceptance
* `SystemCall::Sleep(dur)` no longer blocks the OS thread.
* Scheduler advances virtual time by polling the heap.
* New unit-test sleeps 10 ms virtual time, completes instantly in real clock.

---

## Steps

- [ ] **Add field to `Scheduler`**

  ```rust
  clock: TickClock,
  sleepers: BinaryHeap<(Instant, TaskId)>, // min-heap on wake time


* [ ] **Inject `TickClock::new(Instant::now())` in `new()`**

* [ ] **In `SystemCall::Sleep(dur)`**

  ```rust
  let wake_at = self.clock.now() + dur;
  self.sleepers.push(Reverse((wake_at, tid)));
  requeue = false;
  ```

* [ ] **At top of each scheduler loop**

  ```rust
  while let Some(&(wake_at, tid)) = self.sleepers.peek() {
      if wake_at <= self.clock.now() {
          self.sleepers.pop();
          self.ready.push(tid);
      } else {
          break;
      }
  }
  ```

* [ ] **Advance clock when idle**

  If no ready tasks and no IO, tick clock by shortest `wake_at - now`.

* [ ] **Add test**

  `sleep_virtual.rs` ensures `Sleep(Duration::from_millis(50))` returns without
  wall-clock delay (check elapsed real time < 5 ms).

