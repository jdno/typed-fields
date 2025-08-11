[private]
default:
    @just --list

# Run a subset of checks as pre-commit hooks
pre-commit:
    #!/usr/bin/env -S parallel --shebang --ungroup --jobs {{ num_cpus() }}
    just prettier true
    just format-toml true
    just format-rust true
    just lint-markdown
    just lint-rust
    just lint-yaml

# Build the documentation
build-docs:
    cargo doc --all-features --no-deps

# Check that Cargo features compile
check-features:
    cargo hack --feature-powerset check --lib --tests

# Check the latest dependencies
check-latest-deps:
    git stash push --keep-index --include-untracked --message "check-latest-deps"
    cargo update
    cargo test --all-features --all-targets --locked
    git checkout -- Cargo.lock
    git stash pop --quiet || exit 0

# Check the minimal dependencies
check-minimal-deps:
    #!/usr/bin/env bash
    set -euo pipefail

    git stash push --keep-index --include-untracked --message "check-minimal-deps"

    toolchain=$(rustup show active-toolchain | cut -d' ' -f1)
    rustup default nightly

    cargo update -Z direct-minimal-versions
    cargo test --all-features --all-targets --locked

    rustup default "${toolchain}"

    git checkout -- Cargo.lock
    git stash pop --quiet || exit 0

# Check the Minimum Supported Rust Version
check-msrv:
    #!/usr/bin/env bash
    set -euo pipefail

    git stash push --keep-index --include-untracked --message "check-msrv"

    toolchain=$(rustup show active-toolchain | cut -d' ' -f1)
    msrv=$(cat Cargo.toml | grep "rust-version" | cut -d'"' -f2)

    rustup default "${msrv}"
    cargo check --all-features --all-targets
    rustup default "${toolchain}"

    git stash pop --quiet || exit 0

# Format JSON files
format-json fix="false": (prettier fix "{json,json5}")

# Format Markdown files
format-markdown fix="false": (prettier fix "md")

# Format Rust files
format-rust fix="false":
    cargo fmt {{ if fix != "true" { "--check" } else { "" } }}

# Format TOML files
format-toml fix="false":
    taplo fmt {{ if fix != "true" { "--diff" } else { "" } }}

# Format YAML files
format-yaml fix="false": (prettier fix "{yaml,yml}")

# Lint Markdown files
lint-markdown:
    markdownlint **/*.md

# Lint dependent crates
lint-dependents:
    cd tests/krate && cargo clippy -- -D warnings

# Lint Rust files
lint-rust:
    cargo clippy --all-targets --all-features -- -D warnings

# Lint TOML files
lint-toml:
    taplo check

# Lint YAML files
lint-yaml:
    yamllint .

# Auto-format files with prettier
prettier fix="false" extension="*":
    prettier {{ if fix == "true" { "--write" } else { "--list-different" } }} --ignore-unknown "**/*.{{ extension }}"

# Publish the crate to crates.io
publish:
    cargo publish -p typed-fields --all-features --token $CARGO_REGISTRY_TOKEN

# Run the tests
test-rust:
    cargo test --all-features --all-targets
