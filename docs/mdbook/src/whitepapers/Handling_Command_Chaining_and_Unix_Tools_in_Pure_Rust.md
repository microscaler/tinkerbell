# Handling Command Chaining and Unix Tools in Pure Rust

## Command Pipelining in Rust

In Rust, you can replicate Unix-style *piping* by capturing a command’s output and feeding it as input to another process. The standard library’s `std::process::Command` supports this by allowing you to pipe `stdout` to `stdin` of subsequent commands. For example, to mimic a shell pipeline like `echo "Oh no, a tpyo!" | sed 's/tpyo/typo/'`, you can set the first command’s `stdout` to piped and then use it as the second command’s `stdin`:

```rust
use std::process::{Command, Stdio};

// Spawn the first process with piped stdout
let mut first_child = Command::new("echo")
    .arg("Oh no, a tpyo!")
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn echo");

// Take the stdout of the first process
let first_out = first_child.stdout.take().expect("Failed to open echo stdout");

// Spawn the second process, using first_out as its stdin
let output = Command::new("sed")
    .arg("s/tpyo/typo/")
    .stdin(Stdio::from(first_out))
    .output()  // wait for the pipeline to finish
    .expect("Failed to run sed");

println!("{}", String::from_utf8_lossy(&output.stdout));
```

This technique can be extended to multiple chained commands by repeating the pattern (each child’s output piped into the next command’s input). Under the hood, Rust opens OS pipes to connect the processes, just like the shell does. The example above spawns two child processes and connects them without invoking a shell, ensuring we stay within “pure” Rust process control.

**Avoiding Deadlocks:** One thing to watch out for is reading/writing in the correct order to avoid deadlocks. If a pipeline generates large output, one process might block waiting for the next to read its output. To prevent this, you can read the final output with methods like `wait_with_output` (as shown) or spawn reader threads. The Rust documentation notes that if the child’s output buffer fills up, the child can block waiting for the parent to read, so the parent should read concurrently. In our simple example, calling `.output()` handles this by reading `sed`’s output fully once `sed` exits. For more complex pipelines, you might need to read from intermediate outputs in separate threads or use asynchronous I/O to keep the pipeline flowing.

## Hybrid Execution vs. Pure Rust Implementation

While Rust gives you fine-grained control to chain processes, creating complex pipelines manually can become *tedious and error-prone*. In a large system (like Tiffany’s workflow engine), it’s impractical to rewrite every single command from scratch. Thus, a **hybrid approach** is often used: **prefer native Rust code for core functionality, but call external utilities when needed**. The key is to integrate those external calls cleanly.

### Spawning External Commands

For external tools not (yet) rewritten in Rust (e.g. a complex `awk` script or specialized system command), you can use `Command` to spawn them and capture their output. As shown above, you can pipe outputs from one external tool into another. You may even spawn a shell with a one-liner pipeline (`sh -c "cmd1 | cmd2"`), but using `Command` to set up pipes directly (without a shell) is safer and more portable.

To simplify pipelines, the community has developed crates like **`duct`** and **`cmd_lib`**. The `duct` crate, for example, provides a high-level API to compose processes with `|` like a shell. It *“makes it easy to build pipelines and redirect IO like a shell”* while handling cross-platform quirks for you. For instance, with `duct` you can write:

```rust
use duct::cmd;
let result = cmd!("echo", "hi")
    .pipe(cmd!("sed", "s/i/o/"))
    .read()?;
assert_eq!(result, "ho");
```

This snippet runs `echo` and pipes its output to `sed`, then reads the final output (which would be `"ho"` in this case). Similarly, the `cmd_lib` crate offers macros to run shell-like commands with pipes and redirections inside Rust code. These libraries abstract away the manual setup of `Stdio::piped` and thread management, reducing boilerplate.

**When to use hybrid execution:** If a required functionality is complex or not available as a Rust library, don’t hesitate to call the external command. For example, if your workflow needs to parse text with a complicated AWK script, writing a full AWK parser in Rust might be overkill. Instead, you could invoke `awk` as an external process and capture its output. The goal is to get the job done reliably: **Use Rust natively where you can for safety and performance, but leverage battle-tested tools where rewriting is impractical.** Just be sure to handle errors (e.g. non-zero exit statuses) and output parsing carefully. The `duct` library by default treats a non-zero exit as an error (so you don’t accidentally ignore failures), which is a helpful safety feature.

## Reimplementing Unix Tools in Rust – The New Ecosystem

One exciting development is that many Unix command-line tools are being reimplemented in Rust, often with comparable or improved performance and safety. Canonical (Ubuntu’s publisher) and the open-source community are actively pushing a **Rust-based userland**, which means down the road you might not need to spawn the old C utilities at all – Rust equivalents will be available. Here’s a rundown of relevant tools and their Rust replacements:

* **GNU Coreutils (ls, cp, mv, head, tail, wc, etc.):** The [uutils coreutils](https://github.com/uutils/coreutils) project has rewritten the entire GNU core utilities in Rust. In fact, *all programs have been implemented* (though a few advanced options may still be missing or have slightly different behavior). This project’s goal is full compatibility: *“Differences with GNU are treated as bugs.”* As of version 0.1.0, uutils’ Rust implementations match or even exceed GNU coreutils performance in many cases. Canonical is confident enough in this project that **Ubuntu 25.10 is planned to ship the Rust coreutils as the default system tools**. By Ubuntu 26.04 LTS, these Rust utilities should be fully production-ready. In practical terms, this means commands like `head`, `tail`, `wc`, etc., will be Rust binaries that behave just like the traditional ones. From a Rust developer’s perspective, you could invoke these the same way you would the GNU versions – but you gain memory safety and (potentially) performance benefits behind the scenes. The uutils team emphasizes maintaining compatibility (passing \~500 of the GNU test suite’s \~600 tests so far, with rapid progress), so you can trust these to be drop-in replacements.

  *Can you use uutils in-process?* Currently, uutils is primarily aimed at producing command-line binaries rather than libraries. However, since the project is open-source (MIT-licensed) and organized by utilities, one could study or extract their implementation for use in a Rust program. More conveniently, uutils provides some **helper crates** (e.g., for parsing coreutils-style date strings, terminal width calculations, etc.) that you can use. But for something like `wc` or `tail`, it’s often trivial in Rust to implement these directly (e.g., reading lines from `BufReader` for `wc`, or using file seeks for `tail`). So in many cases you won’t need to call an external `wc` at all – you can count lines/words with a few lines of Rust. The real win is for more complex tools or when you need 100% compatibility with all the quirky options of the GNU versions.

* **grep:** Searching text via regex is well-supported in Rust. Instead of spawning GNU grep, you can use Rust’s built-in `regex` crate to filter lines, or even harness fully-featured tools like **Ripgrep**. *Ripgrep (rg)* is an extremely fast grep alternative written in Rust. It respects `.gitignore` files and is built for speed and usability. Ripgrep is so popular that it’s often aliased as “grep” by power users. While ripgrep is primarily a standalone binary, it’s powered by Rust libraries (e.g. the `regex` and `grep` crates by Andrew Gallant). You could integrate those libraries to search text in-memory, achieving grep-like functionality without spawning a process. If needing simple substring matching, Rust’s standard library or a crate like `aho-corasick` can do it directly. The bottom line: for most “grep” use-cases (searching logs, filtering output), Rust code can do it natively with equal or better performance than the GNU tool. In a pinch, you can still call out to `grep`, but with projects like ripgrep, the ecosystem is moving toward native solutions. (Notably, the uutils project did **not** rewrite grep as part of coreutils – grep is from a separate GNU package – but they list ripgrep as a “friend” project worth using.)

* **awk:** AWK is essentially a small programming language for text processing, which makes it harder to replace with one simple crate. However, there are Rust projects tackling this too. For instance, the **`frawk`** project provides an *Awk-like language implemented in Rust*, including a bytecode interpreter and even an LLVM JIT for performance. It supports CSV/TSV parsing and can leverage multiple cores – in fact, *frawk can process CSV data at over 2 GB/s on a laptop* in parallel mode, showing that Rust can match or beat AWK in heavy-duty text crunching. There’s also **`zawk`**, a crate implementing AWK-like syntax in Rust, and the broader **posixutils-rs** project which has a Rust implementation of AWK aiming for POSIX compliance. If your use of `awk` is simple (e.g., cutting fields or doing arithmetic on columns), you might not need a full AWK engine – a few lines of Rust (splitting strings, using regexes, etc.) can do the job. But for complex AWK scripts, you could integrate an existing Rust AWK engine or fall back to invoking the system `awk`. Since rewriting every quirky AWK script in Rust by hand would be arduous, this is exactly the scenario for *hybrid execution*: use the native approach for the simple cases, and call out to `awk` (or `frawk`) for the complex ones. Over time, as Rust AWK implementations mature, you may be able to use them as libraries to run AWK scripts internally, avoiding external processes entirely.

* **Other tools (head, tail, sort, etc.):** Many of these are covered by Rust coreutils. Tools like `head` and `tail` are straightforward to implement using Rust’s file and buffer APIs, and the uutils versions are available if needed. Sorting (`sort`) can be done with Rust’s `slice::sort` or by collecting lines and sorting (though be mindful of memory for very large inputs). There are also specialized crates (for example, `itertools` crate can help with things like chunking or taking the last N items for a tail-like behavior). Canonical’s initiative means that even if you do call external `tail` or `sort`, on an up-to-date Ubuntu those might actually be Rust executables under the hood, offering safer behavior. It’s worth noting that memory safety in these tools is a big motivation – many GNU tools are old C code that have had security bugs in the past (e.g., buffer overflows). Rewriting in Rust helps eliminate those issues. As one Canonical engineer put it, this is about *“engineering Ubuntu for the next 20 years”* with memory-safe foundations.

* **Find and xargs (findutils):** Searching for files and handling lots of filenames is another area seeing Rust replacements. The uutils project has a **findutils** rewrite underway, including `find`, `xargs`, `locate`, etc. Efficient directory traversal and file matching can be done in Rust with crates like `ignore` (used by ripgrep) which handle filesystem walking and filtering. If Tiffany workflows often use `find` piped into other commands, you might eventually use a Rust library to gather files and then process them in code. Until then, spawning `find` is fine, but keep an eye on the Rust-based findutils as they become production ready.

## The Importance of Canonical’s Rust Userland Effort

Canonical’s push to replace the GNU userland with Rust versions is a strong validation of the “native-first” approach. By Ubuntu 25.10, **the core system utilities will be Rust by default**. This not only improves security and performance but also changes the landscape for developers: it blurs the line between “Rust code” and “system utility.” If you’re automating something in Rust, you can increasingly rely on Rust-based tools being present. For example, if your code spawns `ls` or `cp`, those will be memory-safe Rust programs on a modern Ubuntu. This reduces the risk of encountering the kinds of memory corruption or weird edge-case bugs that sometimes plagued the old C utilities.

Moreover, the fact that these reimplementations are MIT-licensed and memory-safe means you could, in theory, vendor or link them in your own projects. Licensing was a contentious point (GPL vs MIT), but the upside is permissive licensing gives developers flexibility. We might see in the future a **Rust crate for coreutils** where you can call functions like `uutils::head(data, n)` to get the first *n* lines of some text, instead of spawning a process. Even today, you can examine how these tools are implemented in Rust and emulate that logic directly in your code for a fully in-process solution.

Finally, Canonical’s effort is not happening in isolation – it’s part of a broader trend. Projects like **posixutils-rs** aim to implement *everything* (even a shell, `make`, `vi`, etc.) in Rust to meet POSIX 2024 standards. New “modern Unix” replacements are popping up: for example, **`sd`** (stream editor) is a user-friendly Rust alternative to `sed`, and **`choose`** is a handy Rust tool that can replace simple uses of `cut` or `awk`. The ecosystem also includes shells like **Nushell**, which reimagines the shell experience with Rust, and it’s no coincidence the uutils team lists Nushell, Ripgrep, and others as **“friends”** in this venture. All this means that down the line, a tool like Tiffany can be built with a **Rust-first philosophy** – leveraging Rust libraries and tools for most tasks, and minimizing the need to escape to the traditional shell environment.

## Conclusion

To implement something like Tiffany with rich Unix-like functionality, you will likely use a combination of the strategies above:

1. **Rust-native pipelines:** Use `Command` pipes or helper crates to chain processes when you must run external commands. This gives you fine control and keeps your code portable and safe (no shell injection issues, etc.). It’s entirely possible to build complex workflows in Rust that orchestrate commands just like a Bash script would, but with the benefit of Rust’s error handling and type safety.

2. **Native libraries for common tasks:** Prefer doing things in Rust code for tasks like searching, parsing, counting, etc. The Rust standard library and crates are powerful – e.g., use iterators and combinators instead of `grep|awk|wc` where feasible. This avoids spawning processes and can boost performance (data stays in memory and you avoid shell text parsing). As we’ve seen, there are high-quality Rust crates for regex searching, CSV processing, and more that cover many typical uses of tools like grep/awk.

3. **Hybrid approach for edge cases:** When a feature isn’t readily available in Rust (or you’d basically be re-implementing a large tool like `awk` entirely for one complex use-case), don’t shy away from spawning the external command. It’s better to call `awk` or `sed` than to introduce bugs by writing a quick-and-dirty parser in a rush. Over time, you might replace these with Rust solutions as they become mature.

4. **Leverage the Rust userland where available:** Keep an eye on what Canonical and open-source projects are providing. If your environment guarantees Rust coreutils, you gain confidence in those tools’ reliability. Eventually, you might be able to directly use those implementations (via crates or APIs) in your Rust programs. Even today, knowing that *differences from GNU are considered bugs* in uutils means you can generally trust the behavior of Rust tools to match your old workflows.

In summary, implementing rich command chaining in Rust is absolutely feasible and can even improve on the classic shell scripts. With careful use of Rust’s process control and the growing ecosystem of Rust-based utilities (Rusty coreutils, ripgrep, frawk, etc.), you can build a powerful hybrid system. **Tiffany’s functionality can be achieved natively as much as possible, falling back to external commands only when necessary – and even then, those external commands are increasingly likely to be Rust programs themselves.** This approach gives you the best of both worlds: the expressiveness of Unix pipelines and the safety and performance of Rust.

**Sources:**

* Rust Standard Library Documentation – Using `Command` to pipe output into another command
* Rust By Example – Avoiding deadlocks when piping data between child processes
* *cmd\_lib* crate documentation – on the tedium of manual piping and a higher-level solution
* *duct* crate – simplifies creating shell-like pipelines in Rust (example of chaining commands)
* **uutils/coreutils** project – *“cross-platform reimplementation of the GNU coreutils in Rust”* (all core programs implemented). Performance on par with or better than GNU in many cases. Set to debut as default in Ubuntu 25.10.
* The Register – *Ubuntu 25.10 plans to swap GNU coreutils for Rust* (discussion of test suite parity and licensing)
* uutils project site – Goal to *“modernize the utils, while retaining full compatibility”* and plans to replace all essential tools (including findutils, diffutils). Collaboration with other Rust tools like Ripgrep and Nushell.
* Modern Unix tools – **Ripgrep** described as *“an extremely fast alternative to grep”*; **sd** (stream replace) as a modern `sed`; **choose** for `cut`/`awk` tasks.
* Rust AWK implementations – **posixutils-rs** providing a POSIX-compliant AWK in Rust; **frawk** project showcasing a high-performance AWK-like engine (parallel CSV processing at >2GB/s).
