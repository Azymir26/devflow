use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context, Result};
use colored::Colorize;
use walkdir::WalkDir;

// ── devflow new ──────────────────────────────────────────────────────────────

pub fn new_project(name: &str) -> Result<()> {
    let path = Path::new(name);

    if path.exists() {
        bail!("directory '{}' already exists", name);
    }

    println!(
        "{} Creating project {}...",
        "●".cyan().bold(),
        name.bold()
    );

    // Create directory structure
    fs::create_dir_all(path.join("src"))
        .with_context(|| format!("failed to create directory structure for '{name}'"))?;

    // Write README.md
    fs::write(
        path.join("README.md"),
        format!("# {name}\n\nA new project created with **devflow**.\n"),
    )
    .context("failed to write README.md")?;

    // Write .gitignore
    fs::write(
        path.join(".gitignore"),
        "\
# Build artifacts
/target/
/dist/
/build/

# Dependencies
/node_modules/
/vendor/

# Environment
.env
.env.local

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
",
    )
    .context("failed to write .gitignore")?;

    // Initialize git repo
    run_git(path, &["init"]).context("failed to initialize git repository")?;

    // Stage and make initial commit
    run_git(path, &["add", "."]).context("failed to stage initial files")?;
    if has_git_identity(path) {
        run_git(path, &["commit", "-m", "Initial commit"])
    } else {
        run_git(
            path,
            &[
                "-c", "user.name=devflow",
                "-c", "user.email=devflow@localhost",
                "commit", "-m", "Initial commit",
            ],
        )
    }
    .context("failed to create initial commit")?;

    println!(
        "\n{} Project {} created successfully!\n",
        "✔".green().bold(),
        name.bold()
    );
    println!("  {} cd {name}", "$".dimmed());
    println!("  {} Start building something great!", "→".dimmed());

    Ok(())
}

// ── devflow status ───────────────────────────────────────────────────────────

pub fn status() -> Result<()> {
    let cwd = env::current_dir().context("failed to read current directory")?;

    println!(
        "{} Scanning for Git projects in {}\n",
        "●".cyan().bold(),
        cwd.display().to_string().dimmed()
    );

    let mut found: u32 = 0;

    // Read immediate subdirectories only (depth 1)
    let entries: Vec<_> = WalkDir::new(&cwd)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .collect();

    for entry in &entries {
        let git_dir = entry.path().join(".git");
        if !git_dir.exists() {
            continue;
        }

        found += 1;
        let project_name = entry.file_name().to_string_lossy();

        // Get current branch
        let branch = get_git_output(entry.path(), &["rev-parse", "--abbrev-ref", "HEAD"])
            .unwrap_or_else(|_| "unknown".into());

        // Get short status
        let status_output =
            get_git_output(entry.path(), &["status", "--porcelain"]).unwrap_or_default();

        let (status_label, status_color) = if status_output.is_empty() {
            ("clean", "green")
        } else {
            let changed = status_output.lines().count();
            // Leak a counted string so we can return a &str — fine for CLI lifetime
            let label = format!("{changed} changed");
            (Box::leak(label.into_boxed_str()) as &str, "yellow")
        };

        let colored_status = match status_color {
            "green" => status_label.green().bold(),
            _ => status_label.yellow().bold(),
        };

        println!(
            "  {} {:<25} branch: {:<15} status: {}",
            "→".dimmed(),
            project_name.bold(),
            branch.cyan(),
            colored_status,
        );
    }

    if found == 0 {
        println!("  {} No Git projects found in this directory.", "!".yellow());
    } else {
        println!(
            "\n{} Found {} project{}.",
            "✔".green().bold(),
            found,
            if found == 1 { "" } else { "s" }
        );
    }

    Ok(())
}

// ── devflow push ─────────────────────────────────────────────────────────────

pub fn push(message: &str) -> Result<()> {
    let cwd = env::current_dir().context("failed to read current directory")?;

    // Verify we're inside a git repo
    if !cwd.join(".git").exists() {
        // Could be in a subdirectory — check via git rev-parse
        run_git(&cwd, &["rev-parse", "--git-dir"])
            .context("not inside a Git repository — run this from a project root")?;
    }

    // Stage all changes
    print_step("Staging changes...");
    run_git(&cwd, &["add", "-A"]).context("failed to stage changes")?;

    // Check there is something to commit
    let status = get_git_output(&cwd, &["status", "--porcelain"])?;
    if status.is_empty() {
        println!(
            "\n{} Nothing to commit — working tree is clean.",
            "✔".green().bold()
        );
        return Ok(());
    }

    // Commit
    print_step("Committing...");
    if has_git_identity(&cwd) {
        run_git(&cwd, &["commit", "-m", message]).context("failed to commit")?;
    } else {
        run_git(
            &cwd,
            &[
                "-c", "user.name=devflow",
                "-c", "user.email=devflow@localhost",
                "commit", "-m", message,
            ],
        )
        .context("failed to commit — consider setting git user.name and user.email")?;
    }

    // Detect current branch
    let branch = get_git_output(&cwd, &["rev-parse", "--abbrev-ref", "HEAD"])
        .unwrap_or_else(|_| "main".into());

    // Check if a remote is configured
    let has_remote = get_git_output(&cwd, &["remote"])
        .map(|r| !r.trim().is_empty())
        .unwrap_or(false);

    if has_remote {
        print_step(&format!("Pushing to origin/{branch}..."));
        run_git(&cwd, &["push", "-u", "origin", &branch]).context("failed to push")?;
    } else {
        println!(
            "  {} No remote configured — skipping push.",
            "!".yellow().bold()
        );
    }

    println!(
        "\n{} Done! Changes committed{}.",
        "✔".green().bold(),
        if has_remote { " and pushed" } else { "" }
    );

    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn print_step(msg: &str) {
    println!("  {} {}", "●".cyan(), msg);
}

/// Returns true if git has user.name and user.email configured in the given dir.
fn has_git_identity(dir: &Path) -> bool {
    Command::new("git")
        .args(["config", "user.name"])
        .current_dir(dir)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
        && Command::new("git")
            .args(["config", "user.email"])
            .current_dir(dir)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
}

/// Run a git command, printing nothing on success. Returns an error if the
/// process exits with a non-zero status.
fn run_git(dir: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .with_context(|| format!("failed to execute git {}", args.join(" ")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "git {} failed: {}",
            args.join(" "),
            stderr.trim()
        );
    }

    Ok(())
}

/// Run a git command and capture its stdout as a trimmed String.
fn get_git_output(dir: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .with_context(|| format!("failed to execute git {}", args.join(" ")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
