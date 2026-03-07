# Repository Guidelines

## Project Structure & Module Organization
`csview` is a small Rust CLI crate. Entry points live in `src/main.rs` and `src/cli.rs`; table rendering logic is grouped under `src/table/` (`printer.rs`, `row.rs`, `style.rs`), and shared helpers live in `src/util.rs`. Shell completion scripts are generated into `completions/`. Build-time completion generation is handled by `build.rs`. CI and release automation live in `.github/workflows/`.

## Build, Test, and Development Commands
- `cargo build --locked`: build the debug binary with the checked-in lockfile.
- `cargo run -- sample.csv`: run the CLI locally against a fixture file.
- `cargo test --locked`: run inline unit tests in the crate.
- `cargo fmt -- --check`: verify formatting exactly as CI does.
- `cargo build --locked --release`: build an optimized binary for manual benchmarking or packaging.

## Coding Style & Naming Conventions
Follow Rust 2021 idioms and format with `rustfmt` using `rustfmt.toml` (`max_width = 120`, reordered imports). Use 4-space indentation and keep modules focused by responsibility. Prefer `snake_case` for functions, variables, and modules; `CamelCase` for types and enums; and descriptive flag names in `clap` structs such as `no_headers` or `header_align`. Keep CLI-facing help text short and imperative.

## Testing Guidelines
Tests are colocated with implementation using `#[cfg(test)]` blocks; there is no separate `tests/` directory today. Add focused unit tests next to the module you change, for example in `src/table/printer.rs`. Before opening a PR, run `cargo test --locked` and, when touching formatting or help output, also run `cargo fmt -- --check` and a quick smoke test such as `printf 'a,b\n1,2\n' | cargo run --locked`.

## Commit & Pull Request Guidelines
Recent history follows Conventional Commits, e.g. `feat: colored help message` and `chore(deps): bump clap`. Keep commits scoped and use prefixes like `feat`, `fix`, `chore`, `ci`, or `docs`. PRs should follow `.github/pull_request_template.md`: include a short description, classify the change type, note tested platforms, confirm self-review, and update docs when behavior changes.

## Release & Packaging Notes
Do not edit generated completion files casually; keep them aligned with CLI changes. Cross-platform build, test, packaging, and release behavior is defined in `.github/workflows/CICD.yml` and `.github/workflows/release.yml`, so mirror those commands when validating release-sensitive changes.
