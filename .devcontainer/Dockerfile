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
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup and add common components
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:${PATH}

RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable && \
    rustup component add clippy rustfmt rust-src && \
    rustc -V && \
    cargo -V

# Developer tools used by the project
RUN cargo install cargo-hack taplo-cli \
    && npm install -g prettier markdownlint-cli \
    && pip3 install --no-cache-dir --break-system-packages pre-commit yamllint

# Create a non-root user (vscode) if not present (typical in devcontainers base)
# Ensure cargo and rustup are accessible to the user
RUN usermod -aG sudo vscode || true \
    && chown -R vscode:vscode ${RUSTUP_HOME} ${CARGO_HOME}

# Set working directory inside container
WORKDIR /workspaces/typed-fields

# Pre-create cargo cache directories to allow mounting as volumes without root permission issues
RUN mkdir -p /usr/local/cargo/registry /usr/local/cargo/git \
    && chown -R vscode:vscode /usr/local/cargo

# Default command
CMD ["sleep", "infinity"]
