VERSION 0.8
PROJECT jdno/typed-fields

FROM rust:1.82.0-slim
WORKDIR /typed-fields

IMPORT github.com/earthly/lib/rust

all:
    BUILD +json-format
    BUILD +markdown-format
    BUILD +markdown-lint
    BUILD +rust-deps-latest
    BUILD +rust-deps-minimal
    BUILD +rust-doc
    BUILD +rust-features
    BUILD +rust-format
    BUILD +rust-lint
    BUILD +rust-msrv
    BUILD +rust-test
    BUILD +yaml-format
    BUILD +yaml-lint

json-format:
    FROM +prettier-container

    # Check the JSON formatting
    RUN prettier --check **/*.{json,json5}

markdown-format:
    FROM +prettier-container

    # Check the Markdown formatting
    RUN prettier --check **/*.md

markdown-lint:
    FROM node:alpine
    WORKDIR /typed-fields

    # Install markdownlint
    RUN npm install -g markdownlint-cli

    # Copy the source code into the container
    COPY . .

    # Check the Markdown for linting errors
    RUN markdownlint **/*.md

prettier-container:
    FROM node:alpine
    WORKDIR /typed-fields

    # Install prettier
    RUN npm install -g prettier

    # Copy the source code into the container
    COPY . .

rust-container:
    # Install clippy and rustfmt
    RUN rustup component add clippy rustfmt

    # Initialize the Rust toolchain
    DO rust+INIT --keep_fingerprints=true

rust-sources:
    FROM +rust-container

    # Copy the source code in a cache-friendly way
    COPY --keep-ts Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir src tests ./

rust-build:
    FROM +rust-sources

    # Build the project
    DO rust+CARGO --args="build --all-features --locked"

rust-deps-latest:
    FROM +rust-sources

    # Switch to beta toolchain
    RUN rustup default beta

    # Update the dependencies to the latest versions
    DO rust+CARGO --args="update"

    # Run tests to ensure the latest versions are compatible
    DO rust+CARGO --args="test --all-features --all-targets --locked"

rust-deps-minimal:
    FROM +rust-sources

    # Switch to nightly toolchain
    RUN rustup default nightly

    # Set minimal versions for dependencies
    DO rust+CARGO --args="update -Z direct-minimal-versions"

    # Run tests to ensure the minimal versions are compatible
    DO rust+CARGO --args="test --all-features --all-targets --locked"

rust-doc:
    FROM +rust-sources

    # Generate the documentation
    DO rust+CARGO --args="doc --all-features --no-deps" --output="doc/.*"

    # Save the documentation to the local filesystem
    SAVE ARTIFACT target/doc AS LOCAL target/doc

rust-features:
    FROM +rust-build

    # Install cargo-hack
    DO rust+CARGO --args="install cargo-hack"

    # Test combinations of features
    DO rust+CARGO --args="hack --feature-powerset check --lib --tests"

rust-format:
    FROM +rust-sources

    # Check the code formatting
    DO rust+CARGO --args="fmt --all --check"

rust-lint:
    FROM +rust-build

    # Check the code for linting errors
    DO rust+CARGO --args="clippy --all-targets --all-features -- -D warnings"

rust-msrv:
    # The Earthly Rust library requires a minimum version of 1.70.0, so we
    # cannot use it for this check.
    ARG MSRV="1.61.0"

    FROM "rust:$MSRV-slim"

    # Copy the source code in a cache-friendly way
    COPY Cargo.toml Cargo.lock ./
    COPY --dir src tests ./

    # Check that the project compiles with the MSRV
    RUN cargo +$MSRV check --all-features --all-targets

rust-publish:
    FROM +rust-build

    # Publish the crate to crates.io
    DO rust+CARGO --secret CARGO_REGISTRY_TOKEN --args="publish -v --all-features --token $CARGO_REGISTRY_TOKEN"

rust-test:
    # Optionally save the report to the local filesystem
    ARG SAVE_REPORT=""

    FROM +rust-build

    # Install cargo-binstall
    DO rust+CARGO --args="install cargo-binstall"

    # Install cargo-tarpaulin
    DO rust+CARGO --args="binstall cargo-tarpaulin"

    # Run the tests and measure the code coverage
    # --privileged is required by tarpaulin to set flags on the binary
    DO --allow-privileged rust+CARGO --args="tarpaulin --all-features --all-targets --out Xml --skip-clean --verbose" --output="../cobertura.xml"

    # Save the coverage report
    IF [ "$SAVE_REPORT" != "" ]
        SAVE ARTIFACT cobertura.xml AS LOCAL cobertura.xml
    END

yaml-format:
    FROM +prettier-container

    # Check the YAML formatting
    RUN prettier --check **/*.{yml,yaml}

yaml-lint:
    FROM pipelinecomponents/yamllint:latest
    WORKDIR /typed-fields

    # Copy the source code into the container
    COPY . .

    # Check the YAML files for linting errors
    RUN yamllint .
