 # Plan: Enable Rusty to Write Code to Files

## 1. Goals

- **Primary goal**: Turn Rusty from a code-suggestion chat agent into a **coding agent** that can create and modify files for you (similar to Claude CLI / Gemini CLI), while you stay inside the Rusty GUI.
- **Security goal**: The agent must **only** read/write files and run commands **inside the directory where `rusty` was launched** (the launch directory), never outside it.

## 2. Requirements

- **R1 – File operations**
  - Agent can create new files and overwrite existing ones.
  - Agent can read files to understand current project state.
  - Agent can list directories to discover project structure.
- **R2 – Command execution**
  - Agent can run commands like `cargo build`, `cargo test`, `rustc file.rs`, etc.
  - Commands must have timeouts and return stdout/stderr/exit code.
- **R3 – Security**
  - All file paths must be validated so that they **resolve to paths under the launch directory**.
  - Absolute paths outside the launch directory must be rejected.
  - `..` traversal tricks must not escape the launch directory.
  - Commands run with the same permissions as the user (no escalation).
- **R4 – UX / Behavior**
  - Default interaction is **natural language**, e.g.:
    - “Create `src/main.rs` with a hello-world program and run it.”
    - “Add a new example `examples/fib.rs` and run it.”
  - Agent should:
    1. Inspect the project (list/read files).
    2. Propose a plan in its reply (briefly).
    3. Execute the plan using tools (write files, run commands).
    4. Report back what it did with file paths and command outputs.

## 3. High-Level Architecture

### 3.1 Tool layer (Rust library crate)

- Define a **tool executor** in the library (`src/tools/autonomous.rs`) exposing:
  - `write_file(path, contents)`
  - `read_file(path)`
  - `list_files(path)`
  - `run_command(command, cwd, timeout_secs)`
- Each tool:
  - Accepts **relative paths** (interpreted relative to launch directory).
  - Canonicalizes the target path and checks it is **under launch directory**.
  - Returns structured results (success flag, data, and human-readable error messages).

### 3.2 Claude / LLM integration

- Extend the Claude proxy layer (`src/claude_proxy.rs`) to:
  - Advertise the tools (`write_file`, `read_file`, `list_files`, `run_command`) via a JSON schema that matches the Claude tools API.
  - Implement a **tool-use loop**:
    1. Send user query + system prompt + tool definitions.
    2. When the model returns `tool_use` blocks, dispatch them to the tool executor.
    3. Feed `tool_result` blocks back to the model.
    4. Repeat until the model returns a final natural-language answer.
- System prompt must explicitly say:
  - Use tools to **create and modify files**, not just print code.
  - Prefer editing existing files over rewriting whole projects.
  - Never access paths outside `{launch_dir}`.

### 3.3 GUI / Worker integration (rusty_tui crate)

- In `rusty_tui/src/gui/app.rs`:
  - Capture the **launch directory** at startup (e.g., current working directory when `rusty` is invoked).
  - Store `launch_dir` as part of the application state.
- In `rusty_tui/src/gui/worker.rs`:
  - Construct a `ToolExecutor` with the captured `launch_dir`.
  - Route user messages through `run_with_tools(...)` instead of a plain chat call.
  - Emit **progress events** for the UI like:
    - “🔧 Running tool: write_file (hello.rs)”
    - “🔧 Running tool: run_command (cargo build)”
- In `rusty_tui/src/gui/layout.rs` and `rusty_tui/src/gui/messages.rs`:
  - Display tool progress and results inline in the chat, so the user can see exactly what the agent is doing.

## 4. Security Model (Launch Directory Constraint)

### 4.1 Launch directory

- When the user runs `rusty` from directory `D`:
  - Treat `D` as `launch_dir`.
  - All file and command operations **must** be relative to `launch_dir`.

### 4.2 Path validation algorithm

For each requested path `p` from the agent:

1. If `p` is relative, join with `launch_dir`: `candidate = launch_dir.join(p)`.
2. Canonicalize: `resolved = candidate.canonicalize()?`.
3. Check: `if !resolved.starts_with(launch_dir) { reject }`.
4. Only operate on `resolved` if the check passes.

This guarantees:
- No `../` attacks to escape `launch_dir`.
- No writing to `/etc/...` or other system paths.
- The agent can only touch files in the **project tree where `rusty` was started**.

### 4.3 Command execution

- Always execute commands with:
  - `cwd = launch_dir` (or a validated subdirectory).
  - A **configurable timeout** (default 60s, max 300s).
- Return:
  - `stdout` (truncated if necessary).
  - `stderr`.
  - `exit_code`.
  - `timed_out: bool`.

## 5. UX / Prompting Design

- Update the system prompt to encourage autonomous behavior:
  - “When the user asks you to create, modify, or run code, use the file and command tools to actually perform those actions in the project, instead of just replying with code snippets.”
- Provide example prompts in help text (`/help`, docs):
  - “Create a Rust binary crate in this directory with a `main.rs` that prints hello, then run it.”
  - “Add a new module `src/math.rs` with `add` and `sub` functions, and update `src/main.rs` to use it.”
  - “Run the test suite and summarize failures.”
- The agent’s answers should:
  - Describe **what it is about to do** (plan).
  - Execute tools.
  - Summarize **what it actually changed** and link to affected paths.

## 6. Implementation Steps

1. **Tool definitions**
   - Implement/verify `write_file`, `read_file`, `list_files`, `run_command` in `src/tools/autonomous.rs`.
   - Ensure each tool uses the **launch directory security checks**.
2. **Wire tools into Claude proxy**
   - In `src/claude_proxy.rs`, define the tool JSON schema.
   - Implement `run_with_tools` to manage the tool-use loop.
3. **Connect GUI worker to tools**
   - In `rusty_tui/src/gui/app.rs`, capture `launch_dir`.
   - In `rusty_tui/src/gui/worker.rs`, instantiate `ToolExecutor` with `launch_dir` and call `run_with_tools`.
4. **UI feedback**
   - In `rusty_tui/src/gui/messages.rs` and `layout.rs`, add message types and UI sections to show “Agent is thinking…” vs “🔧 Running tool: …”.
5. **System prompt & docs**
   - Update system prompt to emphasize autonomous file-writing and security constraints.
   - Document behavior and examples in `README.md`, `QUICK_START.md`, and `AUTONOMOUS_AGENT.md`.
6. **Testing**
   - Manual tests:
     - “Create a hello world program in `hello.rs` and run it.”
     - “List all `.rs` files in this directory.”
     - “Read `Cargo.toml` and add a dependency, then run `cargo build`.”
   - Negative tests:
     - Ask the agent to write to `/etc/passwd` → must be rejected.
     - Ask the agent to write using `../../` paths that escape the project → must be rejected.

## 7. Future Extensions

- Add higher-level tools:
  - `search_in_files(pattern)` for grep-like queries inside the project.
  - `run_tests` to standardize test execution and result parsing.
- Add optional **git tools** (behind an explicit user opt-in) to commit or revert changes.

