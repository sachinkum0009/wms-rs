# AGENTS.md

## Agent Coding & Operation Guidelines for `wms-rs` Workspace

Welcome, agentic contributor! This document guides sophisticated AI and human agents working in the `wms-rs` Rust monorepo. It covers standard workflows, best practices, and crucial coding conventions for effective, correct, and style-compliant contributions.

---

## 1. Build, Lint, & Test Commands

### Workspace
- The project is a multi-crate Rust workspace. All commands can be run at the root or with a `-p <crate>` argument (packages: `wms_core`, `wms-db`, `wms-cli`, `wms-planner`).

### Build
- **All crates (workspace):**
  ```bash
  cargo build
  ```
- **Specific crate:**
  ```bash
  cargo build -p wms-cli
  ```
- **Release mode:**
  ```bash
  cargo build --release
  ```

### Lint
- **Rustfmt (formatting):**
  ```bash
  cargo fmt --all
  ```
- **Clippy (lints):**
  ```bash
  cargo clippy --all --all-targets -- -D warnings
  ```
  (No `.clippy.toml` or `rustfmt.toml` found: use standard style)

### Test
- **All tests, all crates:**
  ```bash
  cargo test
  ```
- **Specific crate:**
  ```bash
  cargo test -p wms-db
  ```
- **Show test output:**
  ```bash
  cargo test -- --nocapture
  ```
- **Single test (by name):**
  ```bash
  cargo test test_function_name
  cargo test -p wms-cli test_function_name
  ```
- **Run only ignored (integration) tests:**
  ```bash
  cargo test -- --ignored
  ```

### Migrations (DB)
- **Using `sqlx`:**
  ```bash
  cargo install sqlx-cli
  sqlx migrate run --database-url $DATABASE_URL
  ```

### Logging/Env
- Configure log levels with `RUST_LOG` (e.g., `RUST_LOG=wms_cli=debug,wms_db=debug`).
- Load variables automatically from `.env` (see `.env.example`).

---

## 2. Code Style & Conventions

#### General Principles
- Use idiomatic, modern Rust (edition 2021).
- Make all code `cargo fmt` and `cargo clippy` clean (no warnings, standard style).
- Strongly prefer explicit over implicit. Use clear types and clear error handling.

### Module & Import Structure
- Use `mod` in `src/main.rs`, `src/lib.rs`, etc. Example:
  ```rust
  mod commands;
  use commands::{system, inventory, order};
  ```
- Re-export traits/types at the lib level for FFI-like crates:
  ```rust
  pub use planner::traits::TaskPlanner;
  pub use types::*;
  ```
- Keep imports minimal, grouped by standard/third-party/local.
- Use fully qualified `use` statements (not `*`).

### Type & Naming
- Modules: `snake_case`. 
- Types (struct/enum/trait): `CamelCase`.
- Struct/enum fields, functions: `snake_case`.
- Traits: use descriptive, non-prefix names, e.g., `TaskPlanner`, `CostEstimator`.
- Acronyms fully uppercase in types: e.g., `DB` not `Db`.
- Constants: `SCREAMING_SNAKE_CASE`.

### Formatting
- Format before every PR/commit using `cargo fmt`. No config: defaults apply.

### Error Handling
- All public APIs return `Result<T, color_eyre::eyre::Report>` (or anyhow).
- Use `color_eyre::bail!`, `context!`, and propagate errors with `?`.
- Use error-context for all external calls, e.g.
  ```rust
  .context("Failed to connect to database")?
  ```
- Prefer rich contextual errors to `unwrap`/`expect`.

### Logging
- Use `tracing` macros: `info!`, `error!`, `warn!`, etc.
- Provide clear, actionable log messages (avoid excessive verbosity in logs).
- Respect log-level settings.
- Do not print secrets (e.g., always mask database URLs in logs).

### Tests
- Co-locate unit tests inline with modules (`#[cfg(test)]`).
- Use `tokio::test` for async; `#[test]` for sync:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      #[tokio::test]
      async fn my_async_test() {...}
  }
  ```
- Integration tests that require real setup should be marked `#[ignore]` and run via `cargo test -- --ignored`.
- Preferred test names: describe behavior ("returns_correct_sum", not "test1").

### Traits, Generics, Extensibility (Planner)
- Use trait-based designs for extensibility, e.g. `TaskPlanner`, `CostEstimator`.
- Use `Default` where sensible for configuration structs.
- Avoid unnecessary `clone`; use references where possible.

### Database & Env
- All DB configuration should come from environment with `.env` autoload (see `DatabaseConfig::from_env`).
- Always mask secrets in logs.
- Database code must never panic on config errors: always surface errors with context.

---

## 3. Adding New Features
- New CLI commands: add module to `src/commands/`, update `mod.rs`, and wire into `main.rs`.
- New planning algorithms: implement `TaskPlanner`, update lib exports (`pub use`).
- Write doc-comments (///) for all public APIs.
- All new features must include tests. If not feasible, explain why in a code comment.

---

## 4. Environment Variables
- Maintain `.env` at project root. Example in `.env.example`.
- Required: `DATABASE_URL`. Optional: DB pool settings, `RUST_LOG`.
- Never check production secrets into version control!

---

## 5. Cursor & Copilot Rules
- None found in `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md`.
- Use only the conventions in this document and the Rust 2021 Edition default style.

---

## 6. Other Best Practices
- Where doubt exists, follow the official Rust Style Guide and Clippy advisories.
- Keep code readable, concise, and documented.
- Mark planned/placeholder/stub implementations with clear TODO/FIXME comments.
- Document all module-level design decisions in code or crate-level README.

---

*This document is maintained by and for agentic coding assistants. If unclear, ask humans to clarify project-specific intent!*
