## Description

Describe what this PR changes and why. If it fixes a bug, explain the root
cause briefly — not just the symptom.

## Type of change

- [ ] Bug fix
- [ ] New feature
- [ ] Refactor (no behavior change)
- [ ] Documentation
- [ ] CI / tooling

## Checklist

I have run the following locally, and they all pass:

- [ ] `cargo test`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo fmt --check`

If this touches platform-specific code (`#[cfg(unix)]` / `#[cfg(windows)]`),
I've checked that CI passes on all three operating systems, not just my own.

## Connected issues

Closes #

## Additional context

Anything else reviewers should know — edge cases considered, alternatives
you tried, or follow-up work you're intentionally leaving out of scope.