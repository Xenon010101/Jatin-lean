# 🤝 Contributing to jatin-lean

Thank you for your interest in contributing to **jatin-lean**! This project is participating in the **GirlScript Summer of Code (GSSoC)**, and we are thrilled to welcome new and experienced contributors alike.

Whether you're fixing a bug, adding a new feature, improving documentation, or writing tests, your contributions make a massive difference.

---

## 📝 Table of Contents

1. [Code of Conduct](#-code-of-conduct)
2. [GSSoC Guidelines](#-gssoc-guidelines)
3. [Local Development Setup](#-local-development-setup)
4. [Project Structure](#-project-structure)
5. [Git Workflow](#-git-workflow)
6. [Coding Standards](#-coding-standards)
7. [Submitting a Pull Request](#-submitting-a-pull-request)
8. [Reporting Issues](#-reporting-issues)

---

## 🛡️ Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please treat all contributors with respect, empathy, and professional courtesy.

---

## 🌟 GSSoC Guidelines

If you are contributing as part of **GSSoC**, please follow these rules to ensure your contributions are counted correctly:

1. **Claiming Issues:**
   - Browse the open issues and comment on the one you'd like to work on.
   - Wait for a maintainer to assign the issue to you before starting work. Unassigned PRs might not be merged or counted.
   - Please claim only **one issue at a time**. Once your PR for that issue is submitted and approved, you can claim another.
2. **Issue Labels:**
   - `good first issue` & `difficulty:easy`: 10 points. Perfect for beginners or documentation fixes.
   - `difficulty:medium`: 20 points. Requires functional changes or test suites.
   - `difficulty:hard`: 30 points. Requires deep Rust knowledge, multi-threaded design, or N-API bindings.
3. **Timeline:**
   - After being assigned an issue, you have **3 days** to submit a Draft PR or show progress. If there's no activity, the issue will be unassigned and given to another contributor.

---

## 💻 Local Development Setup

`jatin-lean` is a hybrid project combining a **high-performance Rust core** and **native Node.js N-API bindings**. Setting it up requires tools for both ecosystems.

### Prerequisites

Make sure you have the following installed on your machine:
- **Node.js**: Version 14 or higher (LTS recommended)
- **Rust Toolchain**: Version 1.70 or higher (includes `cargo` and `rustc`)
  - Install Rust via [rustup.rs](https://rustup.rs/)

### Setup Instructions

1. **Fork the Repository:**
   Click the "Fork" button at the top-right of the GitHub repository page.

2. **Clone Your Fork:**
   ```bash
   git clone https://github.com/<your-username>/jatin-lean.git
   cd jatin-lean
   ```

3. **Install Dependencies:**
   ```bash
   npm install
   ```
   *Note: This will also trigger the `./build.sh` script to compile the native Rust bindings (`index.node`) in release mode.*

4. **Verify the Installation:**
   Run the test suites to ensure everything works perfectly on your system:
   ```bash
   # Run JS test suite
   npm test

   # Run Rust unit tests
   cargo test
   ```

---

## 📂 Project Structure

Understanding the layout of the project will help you find files quickly:

```
jatin-lean/
├── src/                   # Rust Source Code
│   ├── main.rs            # CLI Entrypoint (orchestrates the 6 command categories)
│   ├── lib.rs             # Rust library declaration
│   ├── node_bindings.rs   # N-API bindings bridging Rust and Node.js
│   ├── cli/               # CLI Commands logic (subcommands)
│   │   ├── node.rs        # node-modules pruning commands
│   │   ├── system.rs      # System hardware & performance commands
│   │   ├── bench.rs       # Benchmark suites (SIMD, io_uring, hash)
│   │   ├── analyze.rs     # Size and snapshot analysis
│   │   └── memory.rs      # IPC, Arena allocator, memory-mapping commands
│   └── ...                # Dedicated logic modules (simd, io_uring, bpf, etc.)
├── index.js               # Node.js Library Entrypoint
├── index.d.ts             # TypeScript Type Definitions
├── package.json           # NPM Package Configurations
├── Cargo.toml             # Rust Package Configurations
├── DEVELOPER.md           # Deep-dive Developer Architecture Guide
└── DOCUMENTATION.md       # General User API/CLI Documentation
```

---

## 🔄 Git Workflow

We follow a typical Fork-and-Pull git workflow.

1. **Sync your Fork:**
   Ensure your local `main` branch is up-to-date with the official repository:
   ```bash
   git checkout main
   git pull origin main
   ```

2. **Create a Feature Branch:**
   Use a descriptive name for your branch:
   ```bash
   # For bugs
   git checkout -b fix/issue-title
   
   # For features
   git checkout -b feature/issue-title
   
   # For documentation
   git checkout -b docs/issue-title
   ```

3. **Commit Your Changes:**
   Write clear, descriptive commit messages following the Conventional Commits style (e.g., `feat: add --keep-license flag` or `fix: resolve windows path parsing error`).

4. **Push and Open a PR:**
   ```bash
   git push origin <your-branch-name>
   ```
   Go to the parent repository on GitHub, click **Compare & pull request**, fill out the PR template, and submit.

---

## 🎨 Coding Standards

### Rust Code Guidelines

- **Formatting:** Always run `cargo fmt` to format your Rust code before committing:
  ```bash
  cargo fmt
  ```
- **Linting:** Run `cargo clippy` to check for common mistakes and clean code suggestions:
  ```bash
  cargo clippy
  ```
- **Safety:** Do not use `unsafe` blocks unless absolutely necessary for N-API or raw OS bindings. Document any `unsafe` block with a safety explanation.

### JavaScript / TypeScript Guidelines

- Use clean, modern ES6+ syntax.
- Ensure TypeScript type definitions in `index.d.ts` are updated whenever the N-API module interface changes.
- Avoid introducing external runtime dependencies unless discussed.

---

## 🚀 Submitting a Pull Request

Before submitting a Pull Request, double-check the following checklist:

- [ ] The code compiles successfully without warnings (`cargo build` and `npm run build`).
- [ ] You have run `cargo fmt` and fixed all `cargo clippy` lint warnings.
- [ ] All unit and integration tests pass successfully (`cargo test` and `npm test`).
- [ ] If changing CLI behavior, you have updated the helper instructions or documentation.
- [ ] Your branch is rebased with the latest upstream `main` branch.
- [ ] In your PR description, link the issue you are fixing (e.g., `Closes #123`).

---

## 📞 Reporting Issues

If you find a bug or want to suggest a new feature but don't want to code it yourself, please use our [GitHub Issues](https://github.com/decodejatin/jatin-lean/issues) page:

- **Bug Reports:** Provide a minimal reproducible example, your OS version, Node/Rust version, and the error logs.
- **Feature Requests:** Detail the use case, why this feature would be helpful, and a potential API or CLI syntax.

---

Thank you for helping us make **jatin-lean** the most performant system optimization platform! Let's code together! 🚀
