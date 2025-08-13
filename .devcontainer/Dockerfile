# syntax=docker/dockerfile:1
# Base on Microsoft devcontainers image for good Codespaces defaults
FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ENV DEBIAN_FRONTEND=noninteractive

# Install base OS packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        clang \
        curl \
        gcc \
        git \
        jq \
        just \
        libpq-dev \
        libssl-dev \
        nodejs \
        npm \
        parallel \
        pkg-config \
        python3 \
        python3-pip \
        rustup \
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup and add common components
ENV PATH="/root/.cargo/bin/:${PATH}"

RUN rustup default stable && \
    rustup component add clippy rustfmt rust-src && \
    rustc -V && \
    cargo -V

# Developer tools used by the project
RUN cargo install cargo-hack taplo-cli \
    && npm install -g prettier markdownlint-cli \
    && pip3 install --no-cache-dir --break-system-packages pre-commit yamllint

# Set working directory inside container
WORKDIR /workspaces/typed-fields

# Default command
CMD ["sleep", "infinity"]
