# Z-roid Development Pipeline

Applies to all repositories in the Z-roid organization.
No exceptions without a documented reason in the PR.

---

## Branch Model

```
main   — production-ready only. protected. no direct pushes. ever.
dev    — integration branch. all features merge here first.
feat/  — new features. branch from dev, merge to dev.
fix/   — bug fixes. branch from dev (or main for hotfixes).
chore/ — tooling, deps, config. branch from dev.
docs/  — documentation only. branch from dev.
```

### Rules

- `main` and `dev` are protected. All changes via PR.
- No force pushes to `main` or `dev`.
- Feature branches are deleted after merge.
- Branch from the latest `dev`, not from an old commit.
- One concern per branch. A branch that touches unrelated files gets rejected.

---

## Commit Convention

Format: `type(scope): description`

| Type | When |
|---|---|
| `feat` | new capability added |
| `fix` | bug corrected |
| `perf` | performance improvement |
| `refactor` | code restructured, no behavior change |
| `docs` | documentation only |
| `test` | tests added or fixed |
| `chore` | deps, tooling, config, CI |
| `ci` | CI pipeline changes |

### Rules

- Description is lowercase, no trailing period.
- Max 72 characters on the subject line.
- Body explains WHY, not what. The diff shows what.
- No `Co-Authored-By` lines. No AI attribution. Ever.
- No `WIP` commits on `dev` or `main`.

Good: `feat(editor): add drag-handle split view`
Bad:  `Updated stuff`, `fix`, `WIP`, `Claude added this`

---

## Code Quality Gates

Every push to `dev` or `main` must pass all of these. CI enforces them.
They must also pass locally before opening a PR.

```sh
just fmt        # cargo fmt — zero tolerance for unformatted code
just clippy     # cargo clippy -D warnings — all warnings are errors
just check      # cargo check — must compile clean
just test       # cargo nextest — all tests must pass
just audit      # cargo audit — no known vulnerabilities
just deny       # cargo deny — license + dep policy enforced
```

If any gate fails the PR is not merged. No exceptions.

---

## Pull Request Rules

- Every change, no matter how small, goes through a PR to `dev`.
- PR title follows the same commit convention format.
- PR description uses the template — fill every section.
- PRs that touch more than one concern get split.
- A PR stays open until CI is green. Not before.
- Squash merge to `dev`. Merge commit to `main` from `dev`.

### Merging to main

`dev` → `main` only when:
- All CI gates pass on `dev`
- All open issues for the milestone are closed or deferred
- A version tag is ready (`v0.x.y`)

---

## Code Standards

### Rust

- Nightly toolchain. Rust 2024 edition.
- `#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]`
  with explicit `#![allow(...)]` for each suppressed lint — no blanket suppression.
- No `unwrap()` or `expect()` in library code. Only in examples and tests.
- No `unsafe` without a documented safety comment explaining the invariant.
- Public API items require documentation comments (`///`).
- No commented-out dead code committed. Delete it.
- No `println!` / `eprintln!` in library code. Use `log::` macros.

### C++ (zyre kernel only)

- C++23 standard.
- `-Wall -Wextra -Wpedantic -Werror` — all warnings are errors.
- clang-tidy must pass before merge.
- No raw `new`/`delete`. Use RAII, smart pointers, or arena allocators.
- Every function that crosses the Rust FFI boundary is documented.

### General

- No magic numbers. Named constants.
- No functions longer than 80 lines without a documented reason.
- No files longer than 1000 lines. Split before it gets there.
- No `TODO` or `FIXME` committed without a linked issue number.

---

## Testing Requirements

- Every new public function gets at least one test.
- Every bug fix gets a regression test.
- Tests live in `#[cfg(test)]` modules in the same file, or in `tests/`.
- Performance-critical code gets a benchmark in `benches/`.
- `just test` must pass on a clean checkout with no prior build artifacts.

---

## AI Agent Rules

When an AI agent (Claude, Gemini, or any other) contributes code:

- The developer reviews every line before committing. No blind commits.
- No `Co-Authored-By` in commit messages.
- AI-generated code is held to the same quality standards as human code.
- If the AI suggests something that bypasses a pipeline rule, reject it.
- The developer is responsible for everything in the repo, regardless of who wrote it.

---

## Versioning

Semantic versioning: `MAJOR.MINOR.PATCH`

| Bump | When |
|---|---|
| PATCH | bug fixes, no API change |
| MINOR | new features, backwards compatible |
| MAJOR | breaking API changes |

Pre-release: `0.x.y` until the project is considered stable.
Tags go on `main` only. Format: `v0.1.0`

---

## Release Checklist

Before tagging a release on `main`:

- [ ] `CHANGELOG.md` updated with all changes since last release
- [ ] All CI gates green
- [ ] `just audit` clean
- [ ] Version bumped in `Cargo.toml`
- [ ] Tested on a clean machine (no cached artifacts)
- [ ] GitHub release created with changelog excerpt
