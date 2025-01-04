VERSION 0.8
PROJECT jdno/typed-fields

FROM rust:1.82.0-slim
WORKDIR /typed-fields

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

COPY_SOURCES:
    FUNCTION

    # Copy the source code into the container
    COPY . .

COPY_RUST_SOURCES:
    FUNCTION

    # Copy the source code in a cache-friendly way
    COPY --keep-ts Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir src tests ./

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
    DO +COPY_SOURCES

    # Check the Markdown for linting errors
    RUN markdownlint **/*.md

prettier-container:
    FROM node:alpine
    WORKDIR /typed-fields

    # Install prettier
    RUN npm install -g prettier

    # Copy the source code into the container
    DO +COPY_SOURCES

rust-container:
    # Install clippy and rustfmt
    RUN rustup component add clippy rustfmt

rust-tarpaulin-container:
    FROM +rust-container

    # Install system-level dependencies
    RUN apt update && apt upgrade -y && apt install -y curl libssl-dev pkg-config

    # Install cargo-tarpaulin
    RUN cargo install cargo-tarpaulin

    # Cache the container
    SAVE IMAGE --cache-hint

rust-sources:
    FROM +rust-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

rust-build:
    FROM +rust-sources

    # Build the project
    RUN cargo build --all-features --locked

rust-deps-latest:
    FROM +rust-sources

    # Switch to beta toolchain
    RUN rustup default beta

    # Update the dependencies to the latest versions
    RUN cargo update

    # Run tests to ensure the latest versions are compatible
    RUN RUSTFLAGS="-D deprecated" cargo test --all-features --all-targets --locked

rust-deps-minimal:
    FROM +rust-sources

    # Switch to nightly toolchain
    RUN rustup default nightly

    # Set minimal versions for dependencies
    RUN cargo update -Z direct-minimal-versions

    # Run tests to ensure the minimal versions are compatible
    RUN cargo test --all-features --all-targets --locked

rust-doc:
    FROM +rust-sources

    # Generate the documentation
    RUN cargo doc --all-features --no-deps

    # Save the documentation to the local filesystem
    SAVE ARTIFACT target/doc AS LOCAL target/doc

rust-features:
    FROM +rust-build

    # Install cargo-hack
    RUN cargo install cargo-hack

    # Test combinations of features
    RUN cargo hack --feature-powerset check --lib --tests

rust-format:
    FROM +rust-sources

    # Check the code formatting
    RUN cargo fmt --all --check

rust-lint:
    FROM +rust-build

    # Check the code for linting errors
    RUN cargo clippy --all-targets --all-features -- -D warnings

rust-msrv:
    ARG MSRV="1.71.1"

    FROM "rust:$MSRV-slim"

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

    # Check that the project compiles with the MSRV
    RUN cargo +$MSRV check --all-features --all-targets

rust-publish:
    FROM +rust-build

    # Copy additional files for the release into the container
    COPY README.md .

    # Publish the crate to crates.io
    RUN --secret CARGO_REGISTRY_TOKEN cargo publish -v --all-features --token "$CARGO_REGISTRY_TOKEN"

rust-test:
    # Optionally save the report to the local filesystem
    ARG SAVE_REPORT=""

    FROM +rust-tarpaulin-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

    # Run the tests and measure the code coverage
    # --privileged is required by tarpaulin to set flags on the binary
    RUN --privileged cargo tarpaulin \
        --all-features \
        --all-targets \
        --out Xml \
        --skip-clean \
        --verbose

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
    DO +COPY_SOURCES

    # Check the YAML files for linting errors
    RUN yamllint .
