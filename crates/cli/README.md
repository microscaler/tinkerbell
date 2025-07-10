# Tiffany CLI (`tctl`)

`tctl` is the official command-line interface (CLI) for interacting with a running Tiffany agent runtime inside a Firecracker or Apple container.

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
    CLI[tctl -outside container]
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
| Task Control  | `tctl task submit`, `task list`          | ⏳        |
| Canvas Access | `tctl canvas show`, `canvas diff`        | ⏳        |
| Agent Status  | `tctl status`, `tctl agents`             | ✅        |
| Logging       | `tctl logs tail`, `logs grep`            | ⏳        |
| Secrets Mgmt  | `tctl secret add`, `secret list`         | ⏳        |
| Help/Docs     | `tctl --help`, `--json`, `--plain`       | ✅        |
| Lifecycle     | `tctl pause`, `tctl shutdown or restart` | ✅      |
| Memory Access | `tctl memory show`, `memory edit`        | ⏳        |
| Debugging     | `tctl debug`, `debug trace`              | ⏳        |
| Misc          | `tctl config`, `tctl version`            | ⏳        |


> ✅ CLI is fully `clap`-based and designed to support both human-readable and scriptable JSON output.

---

## 🔐 Transport Configuration

`tctl` connects to a running agent using:

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
tctl status
# JSON formatted
tctl status --json
# Plain text
tctl status --plain

# Submit a plan
tctl task submit --file plan.json

# Show logs for a canvas task
tctl logs tail --task-id task_187
```

---

## 📦 Project Layout

```txt
crates/cli/
├── bin/
│   └── tctl.rs         # CLI entrypoint
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

`tctl` is designed to be ergonomic, scriptable, and safe-by-default — providing full visibility and control over autonomous agent operation without requiring privileged access to the agent internals.
