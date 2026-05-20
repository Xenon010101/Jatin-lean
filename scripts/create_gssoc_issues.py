#!/usr/bin/env python3
import subprocess
import time
import sys

issues = [
    {
        "title": "[GSSOC] [Docs] Add interactive CLI walkthrough or detailed user tutorial in USER_GUIDE.md",
        "labels": ["gssoc", "good first issue", "difficulty:easy", "domain:docs"],
        "body": """### 📝 Description
The current `USER_GUIDE.md` lists the available commands and flags, but it lacks a structured "Getting Started Walkthrough" for newcomers who are installing `jatin-lean` for the first time.

Adding a step-by-step tutorial will make the onboarding experience much smoother and help new users understand the real power of the CLI.

### 🎯 Acceptance Criteria
- Add a **"🚀 Getting Started Walkthrough"** section to the top of `USER_GUIDE.md` (after the introduction).
- The walkthrough should cover:
  1. Installing the package globally.
  2. Creating a sample project with a bloated `node_modules` (provide simple commands to generate dummy files).
  3. Running a local dry-run scan (`jatin-lean node scan`) and interpreting the output table.
  4. Running an interactive prune command (`jatin-lean node prune`).
  5. Creating a custom config file (`rules.toml`) using `jatin-lean node prune --init-config` and showing how to configure it to keep specific folders like `examples/`.
- Use clear, formatting-rich markdown (bash code blocks, tables, lists).

### 📂 Code / Files Involved
- [USER_GUIDE.md](USER_GUIDE.md)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Feature] Add a --keep-license flag to prevent deletion of LICENSE files",
        "labels": ["gssoc", "good first issue", "difficulty:easy", "domain:rust"],
        "body": """### 📝 Description
Currently, `jatin-lean` classifies files like `LICENSE`, `LICENSE.txt`, `LICENCE`, etc., under the `Documentation` category in `rules.rs`. When a user runs a prune with default rules targeting documentation, these license files are permanently deleted.

To follow best practices for open-source compliance and legal safety, we should add a `--keep-license` CLI flag (and a corresponding configuration entry in `rules.toml`) to ensure license files are never deleted, even if documentation pruning is active.

### 🎯 Acceptance Criteria
- Add `keep_license: bool` flag to the CLI options in `src/main.rs`.
- Update `PruneRules` in `src/rules.rs` to support a `keep_license` setting.
- In `rules.rs` classification logic, check if a file matches license naming patterns (e.g., case-insensitive `license*` or `licence*`). If `keep_license` is true, classify it as `None` (kept) or explicitly skip it.
- Add a unit test verifying that with `--keep-license` enabled, LICENSE files are not deleted.

### 📂 Code / Files Involved
- [src/main.rs](src/main.rs)
- [src/rules.rs](src/rules.rs)
- [src/config.rs](src/config.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Feature] Add a --silent or --quiet CLI flag to silence outputs",
        "labels": ["gssoc", "good first issue", "difficulty:easy", "domain:rust"],
        "body": """### 📝 Description
When running `jatin-lean` inside CI/CD pipelines, postinstall scripts, or other automated contexts, you often want to silence the standard terminal outputs, ASCII banners, and active spinners, only logging errors or returning raw JSON output.

We need to add a `--silent` (or `--quiet`, `-q`) CLI flag that completely silences all stdout logging and indicators.

### 🎯 Acceptance Criteria
- Add `--silent` / `-s` (or `--quiet` / `-q`) flag to the `Cli` struct in `src/main.rs`.
- Pass the silence status to logging / display utilities in `src/display.rs`.
- When `--silent` is active, suppress the ASCII banner, progress bar spinners, and analysis tables.
- Ensure that `--json` output still prints normal JSON output even if `--silent` is active (so users can query data silently).

### 📂 Code / Files Involved
- [src/main.rs](src/main.rs)
- [src/display.rs](src/display.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Testing] Add robust unit tests for scanner::last_accessed_days and helper utilities",
        "labels": ["gssoc", "good first issue", "difficulty:easy", "domain:testing"],
        "body": """### 📝 Description
The `last_accessed_days` function in `src/scanner.rs` retrieves package age based on file metadata access time (`atime`). Currently, this function lacks dedicated unit testing.

To ensure this critical heuristic (used for global scanning and age-based pruning) remains bug-free and portable across platforms, we need comprehensive unit tests.

### 🎯 Acceptance Criteria
- Locate `last_accessed_days` in `src/scanner.rs`.
- Add at least 3-4 unit tests in `src/scanner.rs` (under `mod tests`) validating:
  1. Accessing a brand new file returns `Some(0)` (or small integer).
  2. Accessing a non-existent path returns `None`.
  3. Modifying file access time programmatically (using standard file tools) correctly updates the returned value.
- Ensure tests are platform-independent (run on Linux, macOS, and Windows).

### 📂 Code / Files Involved
- [src/scanner.rs](src/scanner.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [CI/CD] Integrate rustfmt and clippy checks in GitHub CI workflow",
        "labels": ["gssoc", "good first issue", "difficulty:easy", "domain:testing"],
        "body": """### 📝 Description
To maintain exceptional code quality and style standards across all incoming pull requests from GSSoC contributors, we need to enforce style (rustfmt) and lint (clippy) checks in our GitHub Action CI pipeline.

### 🎯 Acceptance Criteria
- Modify `.github/workflows/ci.yml` to introduce a new job (or steps within the existing job):
  1. Run `cargo fmt --all -- --check` to verify code is formatted.
  2. Run `cargo clippy --all-targets --all-features -- -D warnings` to fail the build on any compiler warnings.
- The CI must run on all `push` to `main` and all `pull_request` target branches.

### 📂 Code / Files Involved
- [.github/workflows/ci.yml](.github/workflows/ci.yml)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Docs] Create a comprehensive examples directory for Node.js library usage",
        "labels": ["gssoc", "good first issue", "difficulty:easy", "domain:docs"],
        "body": """### 📝 Description
`jatin-lean` can be imported directly as a high-performance Node.js library thanks to native N-API bindings. While `README.md` shows a short sample, having an interactive and comprehensive `examples/` folder will show developers how to utilize the rich API.

### 🎯 Acceptance Criteria
- Create an `examples/` directory at the root of the project.
- Add at least 3 distinct JavaScript examples:
  1. `scan-modules.js`: Shows how to scan a path, print potential savings, list categories, and save a snapshot.
  2. `system-health.js`: Runs both system assessment (`assessSystem()`) and project health checking (`checkHealth()`), printing a colored terminal summary.
  3. `benchmarks-simd.js`: Runs active SIMD and parsing benchmarks (`runBenchmarks()`) and lists results in a table.
- Add a short `README.md` inside `examples/` explaining how to run them (`node scan-modules.js`).

### 📂 Code / Files Involved
- Create `/examples/` directory
- Create `/examples/README.md`

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Feature] Implement Pruning Profiles (aggressive, balanced, conservative)",
        "labels": ["gssoc", "difficulty:medium", "domain:rust"],
        "body": """### 📝 Description
Currently, users have to manually toggle categories or list custom files in `rules.toml`. We should implement standard "Pruning Profiles" to let users choose pre-packaged safety tiers:
- `conservative`: Safest tier. Only prunes highly-safe targets: CI/CD configuration files (`CiConfig`), and obvious test folders (`TestAsset`).
- `balanced` (Default): Prunes Docs, Examples, Maps, CI files, and test files. Safe for 99% of web applications.
- `aggressive`: Prunes everything, including TypeScript source files (`*.ts`, `*.tsx`) and source maps. Max space savings, but high risk if types are loaded dynamically.

### 🎯 Acceptance Criteria
- Add a `--profile` flag to the CLI (`src/main.rs`), accepting one of `conservative`, `balanced`, or `aggressive`.
- Add a `profile` field in `rules.toml` / `src/config.rs`.
- Map each profile to a predefined set of enabled categories in `src/rules.rs`.
- Add unit tests verifying that changing profiles changes which file categories are returned by `classify()`.

### 📂 Code / Files Involved
- [src/rules.rs](src/rules.rs)
- [src/config.rs](src/config.rs)
- [src/main.rs](src/main.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Performance] Parallelize batch file deletion in deleter.rs using Rayon",
        "labels": ["gssoc", "difficulty:medium", "domain:rust"],
        "body": """### 📝 Description
The discovery phase is fully parallelized via Rayon, enabling super-fast scans of thousands of directories. However, the actual deletion phase in `src/deleter.rs` is synchronous and sequential.

Pruning can delete over 10,000+ files. Parallelizing this deletion phase will significantly boost performance on multi-core systems with modern NVMe SSDs that support deep I/O queues.

### 🎯 Acceptance Criteria
- Refactor `execute_deletion` in `src/deleter.rs` to delete files in parallel using `rayon`.
- Group deletion targets by package or chunk, and iterate using `par_iter()`.
- Ensure that shared metrics (e.g., total bytes deleted, progress status) use atomic operations (`AtomicU64`) to avoid race conditions.
- Correctly thread and display the `indicatif` progress bar update concurrently.

### 📂 Code / Files Involved
- [src/deleter.rs](src/deleter.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Feature] Implement a --backup flag and a restore command (Undo / Restore system)",
        "labels": ["gssoc", "difficulty:medium", "domain:rust"],
        "body": """### 📝 Description
Pruning node_modules is a destructive action. If a specific package has bad exports or requires a non-standard file that was deleted, the build will break. Currently, the only way to recover is running `npm install` again, which can be slow.

We need an enterprise-grade "Undo/Restore" capability. When the `--backup` flag is supplied during `prune`, compile all targeted candidates into a compressed archive before deleting. Add a corresponding `restore` command to undo the changes.

### 🎯 Acceptance Criteria
- Add `--backup` flag to the CLI.
- Implement a backup engine in `src/snapshot.rs` (or a new `backup.rs`) that packages files into `.jatin-lean/backups/<backup-id>.tar.gz` (or standard zip format).
- Add a CLI command `jatin-lean node restore <backup-id>` which extracts the archive back to `node_modules`.
- Add integration tests verifying a deleted directory is successfully restored.

### 📂 Code / Files Involved
- [src/cli/node.rs](src/cli/node.rs)
- [src/snapshot.rs](src/snapshot.rs)
- [src/main.rs](src/main.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [N-API] Implement progress callbacks for long-running Node.js library operations",
        "labels": ["gssoc", "difficulty:medium", "domain:javascript"],
        "body": """### 📝 Description
Our Node.js library uses native promises that resolve only when operations like `scanNodeModules` or `checkHealth` are fully complete. On huge monorepos, this can take a few seconds.

To build rich React/Node interfaces, developers need progress updates. We want to extend our N-API bindings to support a JS callback argument that reports the number of packages scanned, size processed, and speed in real-time.

### 🎯 Acceptance Criteria
- Extend N-API module bindings in `src/node_bindings.rs`.
- Allow an optional javascript callback function `(progress: ProgressData) => void` as an argument.
- Use `napi::threadsafe_function::ThreadsafeFunction` in Rust to safely post progress updates back to the Node event loop during the parallel scan.
- Update `index.d.ts` types to support the callback option.

### 📂 Code / Files Involved
- [src/node_bindings.rs](src/node_bindings.rs)
- [index.d.ts](index.d.ts)
- [index.js](index.js)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Feature] Add warnings for broken/uninstalled package.json dependencies",
        "labels": ["gssoc", "difficulty:medium", "domain:rust"],
        "body": """### 📝 Description
The `jatin-lean node health` check assesses the health of direct dependencies. Let's make it much smarter. It should read the user's primary `package.json` in the project root, parse `dependencies` and `devDependencies`, and check if they actually exist inside `node_modules`.

If a dependency is missing, empty, or lacks its own `package.json`, trigger a health warning.

### 🎯 Acceptance Criteria
- In `src/health.rs`, parse the root `package.json`.
- Walk the declared dependencies lists.
- For each, check if `node_modules/<pkg-name>` exists and contains a valid `package.json`.
- Add missing packages to `missingDeps` list and decrease the overall health score.
- Report the missing/broken packages in the visual terminal output.

### 📂 Code / Files Involved
- [src/health.rs](src/health.rs)
- [src/cli/node.rs](src/cli/node.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Bug] Prevent infinite loops and crashes when encountering cyclic symlinks",
        "labels": ["gssoc", "difficulty:medium", "domain:rust"],
        "body": """### 📝 Description
In large monorepos (e.g. PNPM/Yarn workspaces), packages are symlinked to each other. When performing directory walking, standard walkers can enter infinite loops or crash if there are circular or recursive symlinks.

We need to ensure that the scanner is completely safe when encountering symlinks.

### 🎯 Acceptance Criteria
- Inspect `scan_node_modules` in `src/scanner.rs`.
- Ensure the parallel file walker explicitly tracks traversed physical symlink paths.
- If a path is already visited, discard it to avoid circular traversal.
- Add an integration test that creates a cyclic symlink fixture and asserts that `jatin-lean` finishes scanning without hanging or crashing.

### 📂 Code / Files Involved
- [src/scanner.rs](src/scanner.rs)
- [tests/](tests/)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Performance] Implement lock-free collector for parallel scanner using Crossbeam",
        "labels": ["gssoc", "difficulty:hard", "domain:rust"],
        "body": """### 📝 Description
Our parallel scanner walks folders concurrently using Rayon. Currently, candidates are pushed to a vector wrapped in a Mutex: `Arc<Mutex<Vec<PruneCandidate>>>`.

On systems with high core counts (e.g., 16+ cores), worker threads block each other due to lock contention on the Mutex. To achieve true HPC-level performance, we should refactor this to use a lock-free channel (using the `crossbeam-channel` crate already in our Cargo.toml).

### 🎯 Acceptance Criteria
- Refactor `scanner.rs` to use `crossbeam_channel::unbounded()` instead of `Arc<Mutex<Vec>>`.
- Rayon worker threads should own a clone of the channel `Sender` and `send` candidates as they find them.
- The main thread (or a dedicated collector job) should pull candidates from the `Receiver` and build the final `ScanResult`.
- Run benchmarks to prove that lock contention is reduced on large node_modules folders.

### 📂 Code / Files Involved
- [src/scanner.rs](src/scanner.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Platform] Graceful handling of Windows locked-file errors during pruning",
        "labels": ["gssoc", "difficulty:hard", "domain:rust"],
        "body": """### 📝 Description
On Windows, the operating system locks files that are currently held open by active processes (e.g. standard Node dev servers like Vite/Webpack). If a user runs a prune, Windows throws an access denied/locked file error, crashing or halting the thread.

We need to handle this gracefully. Instead of crashing, identify locked files, log a warning, skip deleting that specific file, and continue pruning the rest.

### 🎯 Acceptance Criteria
- In `src/deleter.rs`, catch file removal errors.
- On Windows (gated via `#[cfg(windows)]`), inspect the error code. If the error corresponds to a sharing violation / locked file (error code `32` / `ERROR_SHARING_VIOLATION`), log it specifically as a skipped locked file rather than a fatal failure.
- Add graceful diagnostics output summarizing how many files were skipped because they were locked by active processes.

### 📂 Code / Files Involved
- [src/deleter.rs](src/deleter.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    },
    {
        "title": "[GSSOC] [Performance] Use Memory Mapping (mmap) for zero-copy package.json parsing",
        "labels": ["gssoc", "difficulty:hard", "domain:rust"],
        "body": """### 📝 Description
Scanning large node_modules involves parsing thousands of `package.json` files to extract entry points. Traditional file reading creates a system call to read into a heap-allocated buffer, causing significant overhead.

By memory mapping (`mmap`) package.json files into memory, we can parse the JSON directly off the kernel page cache buffers with zero allocations or intermediate buffers.

### 🎯 Acceptance Criteria
- In `src/scanner.rs` (or a helper utility), replace `fs::read_to_string` for package.json with `memmap2::Mmap::map`.
- Pass the raw mapped byte buffer `&[u8]` directly to `serde_json::from_slice` for zero-copy parsing.
- Ensure safe fallbacks are in place if memory mapping fails (e.g., fallback to traditional file reading if size is 0 or on unsupported platforms).
- Add tests validating parsed package fields off memory-mapped files.

### 📂 Code / Files Involved
- [src/scanner.rs](src/scanner.rs)
- [src/mmap.rs](src/mmap.rs)

### 🤝 How to Claim
Comment `/claim` below and a maintainer will assign it to you!"""
    }
]

def main():
    print(f"Starting to raise {len(issues)} GSSoC issues on GitHub...")
    for idx, issue in enumerate(issues, 1):
        print(f"[{idx}/{len(issues)}] Creating issue: {issue['title']}")
        
        # Build command: gh issue create --title "..." --body "..." --label "gssoc,..."
        cmd = [
            "gh", "issue", "create",
            "--title", issue["title"],
            "--body", issue["body"]
        ]
        
        for label in issue["labels"]:
            cmd.extend(["--label", label])
            
        try:
            res = subprocess.run(cmd, capture_output=True, text=True, check=True)
            print(f"  ✓ Success! Issue URL: {res.stdout.strip()}")
        except subprocess.CalledProcessError as e:
            print(f"  ❌ Failed to create issue. Error:\n{e.stderr}", file=sys.stderr)
            
        # Avoid rapid hits to GitHub API rate limits
        time.sleep(2)

if __name__ == "__main__":
    main()
