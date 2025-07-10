# Tiffany CLI (`ttnyctl`)

`ttnyctl` is the official command-line interface (CLI) for interacting with a running Tiffany agent runtime inside a Firecracker or Apple container.

This tool enables developers and systems to submit tasks, monitor state, retrieve logs, inspect memory, and control agent lifecycle — all without needing direct access to the agent’s internals.

---

## 🎯 Primary Responsibilities

- Communicate with the `tiffany` agent over **gRPC** or **Unix/vsock socket**
- Submit task plans or commands to be scheduled
- Query the canvas, task logs, and memory state
- Send signals (pause, shutdown, snapshot) to the running agent
- Help developers inspect and debug autonomous reasoning flows

---

## 🧱 System Architecture

```mermaid
graph LR
  subgraph Outside Container
    CLI[ttnyctl -outside container]
  end
  subgraph Inside Container
    %% Padding between containers
    PAD_OUT[" "]:::pad

    subgraph Tiffany Process
        %% Add invisible nodes for padding
        PAD1[" "]:::pad
        API[agent gRPC API]
        Agent[tiffany -inside container]
        Scheduler
        ReasonAct
        PAD2[" "]:::pad
    end
    PAD_OUT2[" "]:::pad
    classDef pad fill:transparent,stroke:transparent;
  end

  CLI -- gRPC via vsock --> API --> Agent
  Agent --> Scheduler --> ReasonAct
````

---

## ✨ Features (Planned & Roadmap)

| Category      | Commands                                 | Status   |
| ------------- |------------------------------------------|----------|
| Task Control  | `ttnyctl task submit`, `task list`          | ⏳        |
| Canvas Access | `ttnyctl canvas show`, `canvas diff`        | ⏳        |
| Agent Status  | `ttnyctl status`, `ttnyctl agents`             | ✅        |
| Logging       | `ttnyctl logs tail`, `logs grep`            | ⏳        |
| Secrets Mgmt  | `ttnyctl secret add`, `secret list`         | ⏳        |
| Help/Docs     | `ttnyctl --help`, `--json`, `--plain`       | ✅        |
| Lifecycle     | `ttnyctl pause`, `ttnyctl shutdown or restart` | ✅      |
| Memory Access | `ttnyctl memory show`, `memory edit`        | ⏳        |
| Debugging     | `ttnyctl debug`, `debug trace`              | ⏳        |
| Misc          | `ttnyctl config`, `ttnyctl version`            | ⏳        |


> ✅ CLI is fully `clap`-based and designed to support both human-readable and scriptable JSON output.

---

## 🔐 Transport Configuration

`ttnyctl` connects to a running agent using:

* **Default (dev):** Unix domain socket (UDS)
* **Firecracker (prod):** vsock address (CID + port)
* **Override:** REST endpoint via `--endpoint` flag

Env vars:

```bash
export TCTL_ADDR=vsock://3:5000
```

---

## 🧪 Usage

```bash
# Show agent status
ttnyctl status
# JSON formatted
ttnyctl status --json
# Plain text
ttnyctl status --plain

# Submit a plan
ttnyctl task submit --file plan.json

# Show logs for a canvas task
ttnyctl logs tail --task-id task_187
```

---

## 📦 Project Layout

```txt
crates/cli/
├── bin/
│   └── ttnyctl.rs         # CLI entrypoint
├── src/
│   ├── lib.rs          # CLI structure + execution
│   ├── commands/       # Subcommand modules
│   └── transport.rs    # gRPC/vsock socket handling
```

---

## 🔧 Development

```bash
# Run the CLI
cargo run -p cli -- status

# Build CLI binary
cargo build -p cli --release
```

---

## Related Crates

* [`daemon`](../daemon) – Runs the actual agent runtime
* [`scheduler`](../scheduler) – Executes tasks
* [`reasonact`](../reasonact) – Cognitive plan engine
* [`canvas`](../canvas) – Memory surface the CLI inspects
* [`wal`](../wal) – Logs the state machine

---

## 🚀 Goals

`ttnyctl` is designed to be ergonomic, scriptable, and safe-by-default — providing full visibility and control over autonomous agent operation without requiring privileged access to the agent internals.
