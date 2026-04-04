<p align="center">
  <h1 align="center">devflow</h1>
  <p align="center">
    <strong>Manage your Git projects faster from the command line.</strong>
  </p>
  <p align="center">
    <a href="https://github.com/Azymir26/devflow/blob/master/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License"></a>
    <a href="https://github.com/Azymir26/devflow"><img src="https://img.shields.io/github/stars/Azymir26/devflow?style=flat-square&logo=github" alt="Stars"></a>
  </p>
</p>

---

**devflow** is a lightweight Rust CLI that eliminates the repetitive Git commands you type every day. Scaffold new projects, monitor all your repos at a glance, and ship changes in a single command.

## Features

- **`devflow new`** &mdash; Scaffold a project with `git init`, `README.md`, `.gitignore`, and `src/` in one step
- **`devflow status`** &mdash; See the branch and working-tree status of every Git repo in a directory
- **`devflow push`** &mdash; Stage, commit, and push in a single command
- Colored terminal output for instant visual feedback
- Graceful error handling with clear, actionable messages
- Zero configuration required

## Installation

### From source (recommended)

Make sure you have [Rust](https://rustup.rs/) installed, then:

```bash
git clone https://github.com/Azymir26/devflow.git
cd devflow
cargo build --release
```

Copy the binary to a directory on your PATH:

```bash
# Linux / macOS
cp target/release/devflow ~/.cargo/bin/

# Windows (PowerShell)
cp .\target\release\devflow.exe $env:USERPROFILE\.cargo\bin\
```

## Usage

### Create a new project

```bash
devflow new my-app
```

```
● Creating project my-app...

✔ Project my-app created successfully!

  $ cd my-app
  → Start building something great!
```

This creates the following structure:

```
my-app/
├── .git/
├── .gitignore
├── README.md
└── src/
```

### Check status of all projects

Run from a parent directory that contains multiple Git repos:

```bash
cd ~/projects
devflow status
```

```
● Scanning for Git projects in /home/user/projects

  → api-server                branch: main            status: clean
  → frontend                  branch: feat/auth       status: 3 changed
  → docs                      branch: main            status: clean

✔ Found 3 projects.
```

### Push changes

Stage everything, commit, and push &mdash; one command:

```bash
devflow push "Add user authentication"
```

```
  ● Staging changes...
  ● Committing...
  ● Pushing to origin/main...

✔ Done! Changes committed and pushed.
```

If no remote is configured, devflow commits locally and lets you know:

```
  ● Staging changes...
  ● Committing...
  ! No remote configured — skipping push.

✔ Done! Changes committed.
```

### Help

```bash
devflow --help
devflow new --help
```

## How it works

devflow wraps Git commands with sensible defaults:

| Command | What it runs under the hood |
|---|---|
| `devflow new <name>` | `mkdir -p`, write scaffold files, `git init`, `git add .`, `git commit` |
| `devflow status` | Scans subdirectories for `.git/`, runs `git rev-parse` + `git status --porcelain` |
| `devflow push <msg>` | `git add -A`, `git commit -m`, `git push -u origin <branch>` |

## Built with

- [Rust](https://www.rust-lang.org/) &mdash; Fast, reliable, and memory-safe
- [clap](https://crates.io/crates/clap) &mdash; Command-line argument parsing with derive macros
- [colored](https://crates.io/crates/colored) &mdash; Terminal color output
- [anyhow](https://crates.io/crates/anyhow) &mdash; Ergonomic error handling
- [walkdir](https://crates.io/crates/walkdir) &mdash; Recursive directory traversal

## Contributing

Contributions are welcome! Here's how to get started:

1. **Fork** the repository
2. **Create a branch** for your feature or fix:
   ```bash
   git checkout -b feat/my-feature
   ```
3. **Make your changes** and ensure the project builds:
   ```bash
   cargo build
   cargo clippy
   ```
4. **Commit** with a clear message and **open a pull request**

### Ideas for contributions

- Add a `devflow clone` command for batch-cloning repos
- Support custom project templates via `devflow new --template`
- Add a `devflow branch` command for quick branch management
- Parallel status checks for large directories
- Configuration file support (`.devflowrc`)

If you find a bug or have a feature request, please [open an issue](https://github.com/Azymir26/devflow/issues).

## License

This project is licensed under the [MIT License](LICENSE).

---

<p align="center">
  Made with Rust by <a href="https://github.com/Azymir26">Azymir26</a>
</p>
