# 🧚‍♀️ Tinkerbell — Autonomous Agentic Runtime

<img src="docs/images/tinkerbell.png" alt="Tinkerbell Logo" width="800"/>

Welcome to **Tinkerbell**, an open-source project designed to build the next generation
of **autonomous agentic runtimes**.

Tinkerbell acts as a long-running, intelligent daemon that can understand user requests, generate
and modify source code, interact with tools and services, and evolve its context over time — all
while maintaining a durable, auditable internal state through a write-ahead log and fine-grained 
Git-based patch tracking.

---

## 🎯 Project Goals

Tinkerbell is not just a chatbot or CLI helper — it is a programmable, multi-modal runtime designed to:

- 🧠 **Maintain Memory**: Persist context between requests with a WAL and state cache
- 🛠️ **Generate & Refactor Code**: Use a virtual file canvas with Git-based micro-commits
- 📡 **Route LLMs Intelligently**: Dynamically select LLMs via [`genai-rs`](https://github.com/jeremychone/rust-genai)
- 🛡️ **Run Safely**: Execute tasks in secure, sandboxed environments (e.g., Firecracker microVMs)
- 📉 **Stay Observable**: Emit logs and Prometheus-compatible metrics for visibility
- 🧩 **Extend Easily**: Load modular tools using the [Model Context Protocol (MCP)](https://modelcontext.org/) (e.g., `crawl4ai`, GitHub adapters)
- 🧬 **Think & Act Like a System**: Model reasoning via coroutine-inspired task scheduling

---

## 🏗️ System Design

The project is implemented as a modular, multi-crate Rust workspace. Major components include:

- `core/` – Orchestrates lifecycle, coordinates components
- `cli/` – Local CLI interface using UNIX sockets
- `api/` – REST/gRPC daemon interface
- `executor/` – Runs user commands, builds, and scripts
- `canvas/` – Virtual code canvas w/ Git diff patching
- `wal/` – Write-ahead log for all task lifecycle events
- `graphdb/` – Embedded Neo4j for semantic memory graphs
- `router/` – Multi-LLM router using `genai-rs`
- `plugins/` – MCP-compatible extension tools
- `scheduler/` – Coroutine-like cooperative task executor

See the full directory tree in the [source](https://github.com/microscaler/tinkerbell).

---

## 📖 Learn More

For in-depth architecture, motivations, and design tradeoffs, refer to the **whitepapers**:

```bash
docs/
├── diagrams/
└── whitepapers/
    ├── Gemini_CLI_Agent_Architecture_and_Reimplementation+Plan.md
    ├── Tinkerbell_System_Architecture_and_Design_Overview.md
    ├── Task_Scheduler.md
    └── Tinkerbell_Concept.md
````

These documents outline how Tinkerbell draws inspiration from Google’s Gemini CLI, Rust’s async/coroutine model, and classic systems design (WALs, schedulers, patch-based VCS). Start with `Tinkerbell System Architecture and Design Overview.md` for a big-picture view.

---

## 🤝 Contributing

We welcome early contributors interested in:

- Rust systems programming
- Task scheduling and actor models
- LLM integration
- Plugin tooling (MCP, WASM, etc.)
- Secure sandboxing and agent execution

> **NOTE:** Contributor setup, coding guidelines, and issue labels will be published soon.

In the meantime, feel free to star 🌟 the project, clone the repo, and explore:

```bash
git clone https://github.com/microscaler/tinkerbell.git
cd tinkerbell
cargo check
````

---

## 🧭 Project Vision

Tinkerbell is a stepping stone toward general-purpose, **autonomous software agents** — systems that can reason, act, 
and self-modify safely and observably. It’s not just a research playground — it’s a practical runtime built to grow 
and learn with its users.

We hope you’ll help shape it.

—
The [Microscaler](https://github.com/microscaler) team

