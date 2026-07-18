# whichway

[![CI](https://github.com/nakornieiev/whichway/actions/workflows/ci.yml/badge.svg)](https://github.com/nakornieiev/whichway/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Read-only CLI that explains why a shell command resolves to a specific binary — detects PATH shadowing, orphaned shims, and version manager conflicts (asdf, nvm, pyenv, mise). Diagnoses, never modifies.

`whichway` answers the question every developer has asked at least once:
**"Why does this command resolve to *that* version, and not the one I expected?"**

It doesn't install, remove, or manage anything. It only inspects your shell
environment and explains what it finds — so you can trust it without giving
it write access to your system.

## The problem

If you've used more than one version manager over the years (`nvm`, `pyenv`,
`asdf`, `rbenv`, `mise`, Homebrew, system packages...), your `PATH` has
probably accumulated:

- Multiple binaries with the same name, shadowing each other in an order
  nobody remembers setting
- Shim files left behind by a version manager you uninstalled months ago
- Aliases or `PATH` exports buried somewhere in `.zshrc` / `.bashrc` that
  silently override everything else
- A command that behaves differently in an interactive shell than it does
  in CI, cron, or a login shell

Existing version managers (`asdf`, `mise`, `nvm`, `pyenv`...) solve version
*switching*. None of them are built to diagnose *conflicts between them*,
because each one is naturally focused on itself. `whichway` is manager-agnostic
and only reports — it doesn't take sides.

## What it does

```bash
# Show every match for `python` in PATH, in resolution order,
# and explain what each one is (shim, symlink, real binary, alias)
whichway python

# Scan your whole PATH for problems: duplicates, broken symlinks,
# orphaned shims, conflicting version managers for the same tool
whichway doctor

# Compare what resolves in a login shell vs an interactive shell
# (classic "works in my terminal, breaks in CI" source of confusion)
whichway diff
```

Example output:

```
$ whichway npm
Resolution order for: npm
  1. /Users/vlad/.nvm/versions/node/v24.16.0/bin/npm  [real binary]  ✅ active
  2. /usr/local/bin/npm  [real binary]  shadowed

$ whichway doctor
⚠️ Found duplicates for: npm
        /Users/vlad/.nvm/versions/node/v24.16.0/bin/npm
        /usr/local/bin/npm
⚠️ Found broken symlinks:
        /usr/local/bin/warp-diag
        /usr/local/bin/docker-index
✅ Found no orphan shims
```

## Why

Most of this information already exists — scattered across `which -a`,
`type -a`, manually reading rc files, and institutional knowledge about
"oh yeah, I think pyenv is still installed too." `whichway` collects it
into one readable report instead of a debugging session.

## Status

Early-stage / learning project. Built while learning Rust, with the goal
of solving a real, small, currently-unsolved annoyance rather than
reinventing an existing tool.

**MVP scope:**
- [x] `whichway <cmd>` — resolution chain with explanations
- [x] `whichway doctor` — duplicates, broken symlinks, orphaned shims
- [x] `--json` output for scripting

**Later:**
- [ ] bash / zsh support
- [ ] `whichway diff` — login vs interactive shell comparison
- [ ] fish / nu shell support
- [ ] Detection of conflicting version managers for the same tool

## Installation

```bash
cargo install whichway
```

*(not yet published — coming soon)*

## Contributing

Issues and PRs welcome once the MVP lands. If you've hit a "why is this
resolving to the wrong version" moment yourself, that story is useful —
feel free to open an issue describing your setup.

## License

MIT
