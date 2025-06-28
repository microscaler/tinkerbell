# Controlled Execution Environment with Rust Shell Commands

Building a controlled execution environment means replacing typical shell commands with safe, Rust-based implementations. This approach is inspired by sandboxed AI coding environments (as might be used by advanced models like Google’s *Gemini* or Anthropic’s *Claude*) where system commands are restricted. By implementing core shell functionalities in Rust, we gain **greater control, security, and cross-platform consistency**. In this deep dive, we’ll explore **why** to replace shell commands, **how** to design the Rust command interface, and **what commands** to implement for an Ubuntu LTS-based sandbox (with macOS support via Apple’s new Containerization, and Windows excluded as per our assumptions).

## Why Replace Shell Commands with Rust?

**Security and Control:** Using Rust instead of invoking shell binaries eliminates many injection and misuse risks. For example, calling external programs (even with `std::process::Command`) can be unsafe if an attacker controls the environment or arguments. An attacker could place a malicious `ls` in the PATH or current directory, causing you to execute the wrong binary. By implementing `ls` (and others) in Rust, we avoid dependency on external binaries and the PATH altogether, closing this exploit vector. As one security discussion notes, **using Rust’s `std::fs` APIs is preferable to shelling out** – it’s cross-platform and doesn’t rely on system utilities being present. In short, an internal Rust implementation ensures the command does exactly and *only* what we intend, and nothing more.

**Auditability:** Defining a fixed set of allowed commands (and rejecting everything else) makes the system easier to audit and reason about. We can parse user requests into an **enum or trait-based command type**, ensuring only a finite, whitelisted set of operations can run. This whitelisting drastically limits abuse: no arbitrary command execution, only the specific functionalities we implement.

**Richer Functionality:** Implementing commands in Rust lets us extend them with features or safety checks not present in the standard shell versions. We can add logging, resource limits, or custom filtering for arguments (e.g. disallowing dangerous options). For example, we might forbid recursive deletes or limit how much output a command like `ls` can produce to prevent denial-of-service. We can also handle edge cases (like filenames with weird characters) in a controlled manner.

**Platform Consistency:** By not relying on external binaries, our commands work uniformly across Linux and macOS (as long as the underlying Rust std library supports them). This avoids differences in BSD vs GNU userland tools. It also means fewer moving parts in our sandbox image (no need to include all coreutils binaries). Since Windows is out of scope (unsupported in our design), we don’t need to account for Windows-specific command quirks.

## Sandbox Isolation: Firecracker MicroVMs and Apple’s Containerization

Even with safe Rust commands, running untrusted code or commands requires OS-level isolation. We assume the environment runs inside a sandboxed Linux instance (Ubuntu LTS). On Linux hosts, one approach is to use **Firecracker**, an AWS-developed microVM technology for lightweight virtualization. Firecracker can spin up a minimal VM in milliseconds, providing a *secure, isolated sandbox* with negligible overhead. Each sandbox VM has its own kernel and cannot interfere with the host or other VMs. This yields much stronger isolation than just Docker containers (shared kernel), at the cost of a bit more memory and startup time.

For macOS hosts, Apple’s new **Containerization** framework (introduced in WWDC 2025) offers similar isolation. It effectively runs each container in a lightweight Linux VM on Apple Silicon, since macOS cannot natively run Linux containers. Like Firecracker, Apple’s solution uses one Linux VM per container for strong separation. This means our controlled environment can run on a Mac by leveraging Apple’s hypervisor-based containers, achieving parity with Linux. In both cases, the sandbox VM will contain our Rust-based “shell” program and user code, keeping the host safe.

**Note:** We assume Windows is unsupported in this scenario, simplifying our design to POSIX-like systems. (Windows would require a very different strategy due to its lack of native containerization for Linux and different system API.)

## Core Shell Commands to Implement in Rust

The controlled environment should cover the most common shell operations so that typical tasks (especially those needed in code execution or file management) can be done without escape. Below is a table of **main commands** to implement, each paired with a Rust struct and the trait implementation signature. We’ll use a trait `ShellCommand` (defined later) that all commands implement, providing a common interface (`execute`) for execution in a given context.

| Command                         | Rust Struct            | Trait Implementation (signature)                                                                                                   |
| ------------------------------- | ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `pwd` – Print current directory | `struct PwdCommand;`   | `impl ShellCommand for PwdCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }`   |
| `cd` – Change directory         | `struct CdCommand;`    | `impl ShellCommand for CdCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }`    |
| `ls` – List directory contents  | `struct LsCommand;`    | `impl ShellCommand for LsCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }`    |
| `mkdir` – Create directory      | `struct MkDirCommand;` | `impl ShellCommand for MkDirCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }` |
| `rm` – Remove file or dir       | `struct RmCommand;`    | `impl ShellCommand for RmCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }`    |
| `cp` – Copy files               | `struct CpCommand;`    | `impl ShellCommand for CpCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }`    |
| `mv` – Move/rename files        | `struct MvCommand;`    | `impl ShellCommand for MvCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }`    |
| `cat` – Show file contents      | `struct CatCommand;`   | `impl ShellCommand for CatCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }`   |
| `touch` – Create empty file     | `struct TouchCommand;` | `impl ShellCommand for TouchCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }` |
| `echo` – Print text to output   | `struct EchoCommand;`  | `impl ShellCommand for EchoCommand { fn execute(&self, ctx: &mut Context, args: &[&str]) -> Result<String, ShellError> { ... } }`  |

Each command’s Rust implementation will use safe APIs instead of spawning external processes. For example:

* **`pwd`** uses `std::env::current_dir()` to get the directory.
* **`cd`** uses `std::env::set_current_dir()` to change the process’s working directory (or alternatively updates an internal `Context.current_dir`).
* **`ls`** uses `std::fs::read_dir` to iterate directory entries and gather names/metadata (rather than calling out to the `ls` binary). We can format output similar to `ls -l` if needed, using Rust’s `metadata()` for file sizes, permissions, etc.
* **`mkdir`** uses `std::fs::create_dir_all`.
* **`rm`** uses `std::fs::remove_file` for files, and `remove_dir_all` for directories (if we choose to support recursive remove; we might implement flags for safe behavior, e.g. require a specific flag for recursive deletes to prevent accidents).
* **`cp`** uses `std::fs::copy` under the hood (with our own logic to handle directories recursively if needed).
* **`mv`** uses `std::fs::rename`.
* **`cat`** uses `std::fs::read_to_string` (or stream the file in chunks to handle large files).
* **`touch`** uses `std::fs::OpenOptions::new().create(true).write(true).open(...)` to create the file if it doesn’t exist (and perhaps `utime` to update timestamp if we wanted to mimic touch exactly).
* **`echo`** simply returns the provided text back (this is trivial, but it avoids needing an external echo command and is useful for scripting within the environment).
* **`find`** could be implemented as a more complex command that searches for files matching patterns, using `glob` or similar libraries to handle wildcards and paths.
* **`grep`** could be implemented to search file contents for patterns, using Rust’s regex library to match text.
* **`head`/`tail`** could be implemented to read the first or last N lines of a file, using buffered reading techniques.
* **`chmod`** could be implemented to change file permissions, using `std::fs::set_permissions`.
* **`chown`** could be implemented to change file ownership, using `std::fs::set_owner` (if the user has permissions).
* **`ln`** could be implemented to create hard or symbolic links, using `std::os::unix::fs::symlink` for symlinks or `std::fs::hard_link` for hard links.
* **`df`** could be implemented to show disk usage, using `std::fs::metadata` to get filesystem stats and format them.
* **`du`** could be implemented to show directory sizes, recursively summing file sizes in a directory tree.
* **`ps`** could be implemented to show running processes, using `sysinfo` or similar crates to gather process information (though this might be limited in a sandbox).
* **`kill`** could be implemented to send signals to processes, using `nix::sys::signal` to send signals like `SIGTERM` or `SIGKILL` (if the sandbox allows it).
* **`top`** could be implemented to show a live view of system processes and resource usage, using `sysinfo` to gather stats and format them in a terminal-friendly way.
* **`history`** could be implemented to show the command history, storing commands in a vector or file and displaying them when requested.
* **`clear`** could be implemented to clear the terminal screen, simply returning an ANSI escape code to reset the terminal.
* **`help`** could be implemented to show available commands and their usage, returning a formatted string with command descriptions.
* **`alias`** could be implemented to create command shortcuts, storing aliases in a map and expanding them when commands are executed.
* **`export`** could be implemented to set environment variables, modifying the context or a global environment map.
* **`source`** could be implemented to load environment variables from a file, reading key-value pairs and updating the context.
* **`which`** could be implemented to find the path of a command, searching the context’s known paths or a predefined list of allowed commands.
* **`env`** could be implemented to show or modify environment variables, returning the current environment or allowing updates.


*Why these commands?* They cover essential **file system navigation and manipulation** tasks in a typical dev workflow. In a coding sandbox, the user (or AI) often needs to navigate directories (`cd`, `pwd`), list files (`ls`), inspect content (`cat`), and create or modify files (`touch`, `echo` for quick output, copying, moving, deleting files). By implementing these, we handle most shell interactions. Other commands (like compilers or language runtimes) could still be provided as *external programs* inside the sandbox (e.g., having Python or GCC installed in the VM), but those are invoked in a controlled way (more on that shortly).

## Designing the Command Trait and Context

To unify command handling, we define a trait (or an abstract base) for all our commands. For instance:

```rust
trait ShellCommand {
    fn execute(&self, context: &mut Context, args: &[&str]) -> Result<String, ShellError>;
}
```

Each command struct (e.g. `LsCommand`, `CdCommand`, etc.) implements this trait. The `execute` method receives a mutable **Context** and arguments. The `Context` is a struct representing the execution environment’s state – for example, it may hold the current working directory, environment variables, open file descriptors, or anything else that commands might need to know or modify. In our simple case, context could be as simple as:

```rust
struct Context {
    current_dir: std::path::PathBuf,
    // ... potentially other fields like environment vars, limits, output streams
}
```

Using a context object rather than global state makes the system more testable and flexible (we could have multiple independent contexts if needed). Some commands (like `cd`) will update `context.current_dir`, and others will just read it. We might integrate the context with actual OS state – for instance, calling `std::env::set_current_dir(&context.current_dir)` so that the **process’s working directory** is in sync. This would let external programs we spawn (if any) start in the correct directory. However, directly using `set_current_dir` affects the whole process, which is fine since our design runs a single command loop in one process (similar to how a shell works). The `pwd` command can just return `context.current_dir` (or call `std::env::current_dir()` which should match if kept in sync).

The trait returns a `Result<String, ShellError>`. We define a custom `ShellError` (or use a crate like `anyhow::Error` for convenience) to uniformly handle errors (file not found, permission denied, etc.). The `String` in the Ok case can contain any output the command wants to show to the user. For example, `ls` would return a newline-separated directory listing (or an empty string if there’s no output), `pwd` returns the current path, `cat` returns the file contents, and commands like `mkdir` or `rm` might return a success message or empty string (they primarily cause side effects). Alternatively, we could design `execute()` to return a structured `CommandResult` that has an output string and perhaps an exit code or flags (to indicate if the environment should terminate on something like an `exit` command). For simplicity, a `Result<String, ShellError>` works: errors can be propagated as needed (and logged or displayed), and output (if any) can be captured.

**Example Trait Implementation Outline:** To illustrate, here’s how a couple of commands might look in code (omitting full error handling and edge cases):

```rust
impl ShellCommand for PwdCommand {
    fn execute(&self, context: &mut Context, _args: &[&str]) -> Result<String, ShellError> {
        Ok(context.current_dir.display().to_string())
    }
}

impl ShellCommand for CdCommand {
    fn execute(&self, context: &mut Context, args: &[&str]) -> Result<String, ShellError> {
        let target = args.get(0).ok_or(ShellError::MissingArgument)?;
        let new_path = context.current_dir.join(target);
        std::env::set_current_dir(&new_path).map_err(ShellError::from)?;  // change process dir
        context.current_dir = std::env::current_dir().map_err(ShellError::from)?;
        Ok(String::new())  // cd produces no direct output
    }
}
```

*Note:* `CdCommand` uses the first argument as the target path, joins it with the current directory (for relative paths), and attempts to change directory. We update both the OS current dir and our context. In case of failure (e.g., path doesn’t exist), we return a ShellError. Other commands like `LsCommand` would use `std::fs::read_dir(&context.current_dir)` and format the entries into a string. Because all operations happen within our Rust code, we can enforce restrictions easily – for instance, disallow navigating above a certain root (preventing access outside a workspace directory), or filter out hidden files if desired. We could also implement glob patterns (`*.txt`) by manually expanding them via the `glob` crate, rather than relying on a shell – this gives us control to, say, limit the number of matches (preventing huge expansions).

## Command Dispatch and Execution Loop

With the commands defined, we need a mechanism to parse user input (or AI instructions) and dispatch to the correct `ShellCommand` implementation. There are two common approaches:

* **Enum dispatch:** Define an enum `CommandType { Pwd, Cd(String), Ls, ... }` and implement a match on it. The [REPL shell example by Taylor](https://dev.to/maxtaylor/custom-replshell-in-rust-550j) uses this approach, parsing input into an enum variant for each command (storing any arguments in the variant). Then it matches the variant and calls the appropriate logic. This is simple and very type-safe (the compiler ensures you handle all variants).

* **Trait objects/registry:** Define a registry (e.g., a `HashMap<String, Box<dyn ShellCommand>>`) mapping command names to an instance of the command struct. For example, map `"ls"` -> `LsCommand` object. Then parsing input becomes looking up the command name and calling `execute` on the boxed trait object. This approach is flexible if you want to allow plugging in new command implementations at runtime or have many commands modularized. It is slightly less type-safe (the mapping of string to object can fail if command is not found), but that’s manageable.

Either way, the execution loop will look something like:

1. **Read** input line (from user or AI).
2. **Parse** it by splitting the command name and arguments.
3. **Lookup/Match** the command. If it’s not one of our predefined commands, return an error or “unknown command” (and in an AI context, the AI would be prevented from running anything outside the whitelist).
4. **Execute** by calling the Rust implementation. Catch any errors.
5. **Return/Print** the output or error message. Loop for the next command.

This is essentially a simple shell REPL (Read-Eval-Print Loop) inside the sandbox. In fact, writing a custom REPL in Rust for basic shell commands is a well-trodden path. By controlling the eval step, we ensure only *allowed* operations happen. As one commenter noted, giving users (or AI) “as little control as possible” by deserializing input into a fixed set of commands makes the system much easier to secure and audit.

## Handling External Processes Safely

Our Rust commands cover file system operations, but what about running user code or programs (like executing a Python script, or compiling code)? In a controlled environment, you might also include a command for execution, for example an `ExecCommand` or similar:

* **`exec`** – execute a program/command inside the sandbox. For instance, `exec("python main.py")` or `exec("gcc code.c -o prog")`. Implementing this in Rust means carefully invoking `std::process::Command` with the provided program and args, but **without** involving a shell. We would directly call the binary (e.g., `/usr/bin/python`) with arguments, after validating that the binary is in an allow-list. This ensures no shell injection (Rust’s process spawning doesn’t invoke a shell by default). We can also impose timeouts or resource limits on the spawned process. For example, use Rust threads or async to kill the process if it runs too long, or Linux cgroups to limit CPU/RAM. If using Firecracker, one could even use the jailer or seccomp filters to restrict syscalls of the executed process further.

In our design, the sandbox VM’s OS would have only the necessary languages/tools installed (Python, etc.), and because the VM is isolated, even if the code tries something malicious, it’s constrained. Still, our Rust command wrapper can drop privileges (run the process as a non-root user in the VM) and redirect I/O. The output of the process can be captured and returned by the `exec` command to the user.

For example, `ExecCommand` might be used like: `exec("python", &["main.py"])` and it would return the console output or execution result. Implementing this requires threading/async management, but it’s doable with Rust’s async ecosystem or threads.

## Benefits Recap

By implementing shell commands in Rust and running them in a sandboxed VM, we achieve a **two-layer safety net**:

* **Language-level safety:** Rust’s memory safety and the lack of a need to invoke shell interpreters mean our command implementations are robust against many classic vulnerabilities (buffer overflows, shell injection, etc.). We’re using high-level APIs that handle strings and paths safely (e.g., `std::fs` functions handle weird filenames without issue, and we avoid parsing shell globs or expansions unsafely). The commands only do what we program them to do – no surprise behavior from external programs. As an example, using Rust to get file metadata avoids relying on parsing `ls` output or trusting that `ls` is even present.

* **OS-level isolation:** Running everything inside a locked-down VM (Firecracker or Apple’s container) ensures that even if something goes wrong, the damage is confined. An exploit in our Rust code (unlikely, but hypothetically) or a malicious user program can’t break out to the host. Firecracker and similar microVMs provide strong isolation with minimal overhead – each container has its own kernel, so a bug in one doesn’t affect others. Apple’s solution similarly opts for per-container VMs for security, trading a bit of memory for **significantly better security** separation. This is considered an acceptable trade-off, especially for development environments where security is paramount and the number of concurrent containers is low.

**No Windows Worries:** By not supporting Windows, we avoid the complexities noted by others when it comes to safe process spawning on Windows (argument parsing differences, etc.). Our Rust commands and the sandbox approach assume a Unix-like environment, which keeps things simpler and more secure by design.

## Conclusion

Implementing main shell commands in pure Rust gives us fine-grained control over what actions can be performed in our execution environment. We listed commands like `pwd`, `cd`, `ls`, and file operations, each with a Rust struct and an `execute()` method signature. These form the core interface of our mini-shell. By using a consistent `ShellCommand` trait, we can easily extend or modify commands, and by containing execution in a Firecracker microVM or Apple’s container, we ensure any risky operations are isolated from the host.

This approach results in a **safer, more predictable, and more easily audited environment**. It prevents the AI or user from invoking anything outside the allowed list, while still providing the functionality needed for most tasks. As a bonus, the rich Rust ecosystem (crates for files, globbing, process control, etc.) allows us to implement these features relatively quickly and confidently. In essence, we’re creating a limited shell in Rust – similar to how one might write a custom REPL for commands – but targeting exactly the features we need and nothing more. This forms a solid foundation for any sandboxed execution service or AI coding assistant backend, combining Rust’s safety with robust virtualization security.

**Sources:**

* Max Taylor, *Building a Custom Shell in Rust* – implementing basic shell commands (`ls`, `cd`, etc.) in a Rust REPL.
* Reddit discussion on safe command execution in Rust – advocating for using Rust’s `std::fs` instead of shell `ls`, and whitelisting commands for security.
* Reddit discussion on command injection and PATH exploits – highlighting dangers of relying on external binaries and the need to control execution environment.
* Hacker News discussion on Apple’s Containerization vs Firecracker – notes that using lightweight VMs per container (as in Firecracker) yields stronger isolation with minimal overhead. This underpins our sandbox strategy for macOS and Linux.
