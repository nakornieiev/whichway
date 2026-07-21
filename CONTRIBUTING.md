# Contributing to whichway

Thanks for considering a contribution — whether that's code, a bug report, or
just a "this happened to me too" story about PATH resolution gone wrong.

## Getting started

```bash
git clone https://github.com/nakornieiev/whichway
cd whichway
cargo build
```

## Running tests

```bash
cargo test
```

Tests use `tempfile` to create isolated, temporary directories and files —
no test touches your real filesystem, home directory, or PATH.

## Before opening a pull request

CI runs the following checks on every push and pull request, across Linux,
macOS, and Windows (for tests) and Linux (for lint):

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

Please run all three locally before opening a PR — it's faster to catch
issues on your machine than to wait for CI to report them, and PRs can't
be merged into `main` until all checks pass.

If `cargo fmt --check` fails, you can auto-fix formatting with:

```bash
cargo fmt
```

If `cargo clippy` reports something you believe is a false positive, explain
why in the PR rather than silently adding `#[allow(...)]` — most of the
time the lint is right, but it's worth a second pair of eyes either way.

## Commit style

This project loosely follows [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add --json output flag
fix: detect manager independently of match kind
refactor: split resolver logic into separate modules
test: add coverage for broken symlink detection
style: fix clippy warnings in doctor.rs
docs: update README example output
```

This isn't strictly enforced, but it makes the history easier to scan and
plays nicely with tools that generate changelogs from commit messages.

## Project structure

A quick map of the codebase:

- `src/main.rs` — CLI argument parsing (via `clap`) and output formatting;
  calls into the library, contains no resolution logic itself
- `src/resolvers.rs` — core resolution logic: finding matches across PATH,
  classifying them as symlinks/shims/binaries, detecting broken links
- `src/shim_detect.rs` — manager detection (asdf/pyenv/nvm) via path and
  shim file content heuristics
- `src/doctor.rs` — PATH-wide scanning: duplicate commands, broken symlinks,
  orphaned shims
- `src/report.rs` — human-readable, colored formatting of results

## Platform-specific code

Some logic (notably symlink creation in tests) differs between Unix and
Windows via `#[cfg(unix)]` / `#[cfg(windows)]`. Code inside a `#[cfg(...)]`
block for a platform you're not currently on **will not be type-checked
locally** — it's only compiled when CI runs on that platform. If you touch
anything platform-specific, double-check the CI results across all three
operating systems before assuming it works.

Also: when building a `PATH`-like string manually (e.g. in tests), always
use `env::join_paths`/`env::split_paths` rather than hardcoding `:` or `;` —
the separator differs by OS, and a hardcoded one will silently break on
the other platform.

## Reporting bugs / suggesting ideas

If you've ever been confused about why a command resolved to the wrong
version, or found a PATH edge case whichway doesn't handle — that's a
useful bug report even without a fix attached. Please include:

- Your OS and shell
- The relevant part of your `PATH` (or which version managers you use)
- What you expected vs. what whichway showed

## License

By contributing, you agree that your contributions will be licensed under
the project's [MIT License](LICENSE).
