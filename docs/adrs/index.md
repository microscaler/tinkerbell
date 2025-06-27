# Tinkerbell ADR Index

This document tracks relevant Architectural Decision Records (ADRs) for the Tinkerbell agentic runtime project. ADRs are numbered sequentially and grouped by functional domain.

---

## 🧠 Agent Architecture

- **ADR-0003**: Task Scheduler Model  
  Coroutine-first, cooperative scheduling with stateful yield and resume semantics.

- **ADR-0004**: Agent Loop and ReAct Design  
  Lifecycle of an agent: reasoning, acting, yielding, and interacting with tools.

- **ADR-0005**: Virtual Canvas Git Strategy  
  Strategy for code change tracking, patch application, and micro-commits.

---

## 📦 Storage & Durability

- **ADR-0006**: WAL Schema and Replay Policy  
  Format of write-ahead log, recovery semantics, and log compaction strategy.

- **ADR-0007**: Process Activity Log (PAL)  
  Real-time activity tracking for agent tasks, complementing the WAL.

- **ADR-0008**: Persistent Agent Memory Strategy  
  Design of Neo4j semantic memory, graph structure, and retrieval policies.
  Emphasize goals, prompts, plan recall, continuity between sessions.


- **ADR-0009**: Code Structure Graph and Symbol Analysis
  graph-based code indexing and semantic AST analysis into
  Neo4j, enabling precise symbol tracking and refactoring intelligence.


- **ADR-0010**: Task Plan Timeline and Execution Metadata
  Structure for task execution metadata, including plan history and decision lineage.
  

---

## 🧩 Modularity & Extensibility

- **ADR-0011**: Plugin and MCP Tooling Architecture  
  How Tinkerbell integrates internal and external tools securely and generically.

- **ADR-0012-**: Agent Skill System (Future Plan)  
  Representation of agent capabilities as composable skills.

---

## 🔐 Runtime & Execution
- **ADR-0013**: Firecracker MicroVM Integration  
  VM lifecycle, execution isolation, resource constraints.

- **ADR-0014**: Command Execution and Safety Policy  
  Shell execution flow, trust boundaries, and `--yolo` mode considerations.

---

## 📡 Interfacing & I/O

- **ADR-0015**: REST/gRPC API Design  
  Interface contract for external task submission, result retrieval, and metadata queries.

- **ADR-0016**: Filesystem Socket Protocol for CLI  
  Local interaction protocol for invoking agent actions via socket.

---

## 🧪 Observability

- **ADR-0017**: Metrics Policy and Instrumentation Plan  
  Prometheus metrics structure, naming conventions, and dashboard philosophy.

- **ADR-0018**: Logging Strategy and Span Hierarchy  
  Use of `tracing`, span lifecycles, log level defaults.

---

## 🧭 Governance & Open Source

- **ADR-0019**: Versioning and Release Policy  
  Semantic versioning, LTS channels, changelog protocol.

- **ADR-0020**: Contributor Roles and Governance  
  Roles, responsibilities, PR review flow, escalation path.

---
