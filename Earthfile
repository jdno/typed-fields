VERSION 0.8
PROJECT jdno/typed-fields

FROM rust:1.82.0-slim
WORKDIR /typed-fields

all:
    BUILD +deps-latest
    BUILD +deps-minimal
    BUILD +doc
    BUILD +features
    BUILD +format
    BUILD +lint
    BUILD +msrv
    BUILD +test

os:
    # Install clippy and rustfmt
    RUN rustup component add clippy rustfmt

sources:
    FROM +os

    # Copy the source code in a cache-friendly way
    COPY Cargo.toml Cargo.lock ./
    COPY --dir src tests ./

build:
    FROM +sources

    # Build the project to cache the target directory
    RUN cargo build

deps-latest:
    FROM +sources

    # Switch to beta toolchain
    RUN rustup default beta

    # Update the dependencies to the latest versions
    RUN cargo update

    # Run tests to ensure the latest versions are compatible
    RUN RUSTFLAGS="-D deprecated" cargo test --all-features --all-targets --locked

deps-minimal:
    FROM +sources

    # Switch to nightly toolchain
    RUN rustup default nightly

    # Set minimal versions for dependencies
    RUN cargo update -Z direct-minimal-versions

    # Run tests to ensure the minimal versions are compatible
    RUN cargo test --all-features --all-targets --locked

doc:
    FROM +sources

    # Generate the documentation
    RUN cargo doc --all-features --no-deps

    # Save the documentation to the local filesystem
    SAVE ARTIFACT target/doc AS LOCAL target/doc

features:
    FROM +sources

    # Install cargo-hack
    RUN cargo install cargo-hack

    # Test combinations of features
    RUN cargo hack --feature-powerset check --lib --tests

format:
    FROM +sources

    # Check the code formatting
    RUN cargo fmt --all --check

lint:
    FROM +sources

    # Check the code for linting errors
    RUN cargo clippy --all-targets --all-features -- -D warnings

msrv:
    ARG MSRV="1.61.0"

    FROM "rust:$MSRV-slim"

    # Copy the source code in a cache-friendly way
    COPY Cargo.toml Cargo.lock ./
    COPY --dir src tests ./

    # Check that the project compiles with the MSRV
    RUN cargo +$MSRV check --all-features --all-targets

publish:
    FROM +sources

    # Publish the crate to crates.io
    RUN --secret CARGO_REGISTRY_TOKEN cargo publish -v --all-features --token "$CARGO_REGISTRY_TOKEN"

test:
    # Optionally save the report to the local filesystem
    ARG SAVE_REPORT=""

    FROM +os

    # Install cargo-binstall
    RUN cargo install cargo-binstall

    # Install cargo-tarpaulin
    RUN cargo binstall cargo-tarpaulin

    # Copy the source code in a cache-friendly way
    COPY Cargo.toml Cargo.lock ./
    COPY --dir src tests ./

    # Run the tests and measure the code coverage
    # --privileged is required by tarpaulin to set flags on the binary
    RUN --privileged cargo tarpaulin \
        --all-features \
        --all-targets \
        --out Xml \
        --verbose

    # Save the coverage report
    IF [ "$SAVE_REPORT" != "" ]
        SAVE ARTIFACT cobertura.xml AS LOCAL cobertura.xml
    END
