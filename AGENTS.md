# typed-fields

`typed-fields` is a collection of macros that generate types following the
newtype pattern in Rust.

## Language

- Use American English spelling, e.g. "color" not "colour".

## Markdown

- Use title case in headings and titles.
- Always use the Oxford comma.
- Use reference-style Markdown links, not inline links.
- Table cells must be single-line. Markdown does not support multi-line cells;
  each newline starts a new row. Ignore line length limits for table rows.

## Rust

### Dependencies

- Require the lowest version of a dependency that still compiles, so that
  applications keep the widest choice of versions. Verify the floor with
  `just check-minimal-deps`.
- Write dependency entries without comments. Do not describe what a package
  does, and do not explain a version requirement. Reasoning that matters, such
  as why a floor cannot go lower, belongs in the commit message.

### Derives

- Standard trait order: Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
  Debug, Default
- Third-party derives: alphabetical by crate, then by macro.
- First list traits from the standard library, then from external crates.

### Documentation

- Documentation should explain the "why", not just the "what".
  - **Types**: Explain design decisions, invariants, and relationships to other
    types.
  - **Functions**: Document side effects, caller considerations, and non-obvious
    behavior.
  - **Modules**: Explain the module's role in the system and key concepts.
- Write documentation for a reader that has no prior context, and especially no
  knowledge of the conversation that led to the creation of the code.
- Write for a consumer of the published crate. Internal rationale, such as which
  library a function hides, stays out of the documentation as well. Document
  what the API does, what it requires of the caller, and how it fails.
- Write function/method docs in third-person singular
  ("Returns the..." not "Return the...").
- Do not add a trailing period on the title (i.e. the first line).
- Use reference-style links in doc comments, not inline path links. Paths like
  `super::` and `crate::` should not appear in rendered documentation.
- Use the `/simple-english:simple-english` skill to adhere to the ASD-STE100
  standard for Simplified Technical English.

### Modules

- One public type per module, use submodules for related types.
- Use `mod.rs` for modules that contain submodules.
- Prefer `pub` over `pub(crate)`. Visibility should come from module
  structure, not access modifiers. If a type needs restricted visibility,
  that is usually a signal to restructure the modules.

### Tests

- Use blank lines to separate Arrange/Act/Assert phases.
- Test functions ordered alphabetically within modules.
- Name tests descriptively: `function_name_<condition>_<result>`, e.g.
  `greet_with_name_returns_greeting`.
- Do not test compiler-derived traits (Eq, Ord, Hash, Clone, etc.). Only test
  auto traits (Send, Sync, Unpin) and custom behavior like builder round-trips.
- Each test should have exactly one assertion.

### Type System

- Use enums with meaningful variants instead of bool parameters.
- Fields must never be `pub`. Implement getters instead.

## Version Control

- Never commit directly to `main`, always create a branch or worktree.
- Every commit should be a logical unit of change.
- Every commit must build and pass all checks. Use `just` recipes for
  verification (e.g. `just pre-commit`).
- Fixes and refactoring should be in separate commits from features.
- Each pull request should have one primary commit with a well-crafted
  message — this is what lands in the Git history. Follow-up fixups within
  the same PR can use simple one-liner messages since they get squashed into
  the primary commit on merge.
- Because a pull request lands as a single commit, separate a fix or a
  refactoring from a feature by opening separate pull requests, not by
  adding commits to one.

### Commit Messages

- We use Git as our Version Control System and GitHub to host the code.
- We use pre-commit hooks to verify the changes before committing them.
- We follow this [style guide][git-style-guide] for commit messages:
  - Capitalized, short (50 characters or less) summary in imperative mode
    ("Fix bug", not "Fixed bug")
  - Blank line between summary and body
  - Focus on the "why" — motivation and reasoning — not what changed
  - Minimal formatting or bullet points, plain prose is preferred
  - Full sentences with simple past and present tense
  - Wrap the body at 72 characters
- Write commit messages for a reader that has no prior context and no access to
  the session history.
- Keep commit messages concise. Aim for two or three paragraphs, not more.
- Don't use backticks in commit message titles, but do use them in bodies.
- **Never** write conventional commit messages.
- **Never** add yourself as a co-author.

[git-style-guide]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
