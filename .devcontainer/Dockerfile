# syntax=docker/dockerfile:1

# Use a base image with sensible defaults for devcontainers
FROM mcr.microsoft.com/devcontainers/base:ubuntu

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

# Install global developer tools as root
RUN npm install -g prettier markdownlint-cli \
    && pip3 install --no-cache-dir --break-system-packages pre-commit yamllint

# Create a shared location for Rust toolchains
RUN mkdir -p /usr/local/rustup /usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:${PATH}

# Install the stable Rust toolchain
RUN rustup default stable

# Change ownership of Rust directories to the vscode user
RUN chown -R vscode:vscode ${RUSTUP_HOME} ${CARGO_HOME}

# Run commands as the vscode user
USER vscode

# Install Cargo binaries
RUN cargo install cargo-hack just taplo-cli

# Set working directory inside container
WORKDIR /workspaces/typed-fields

# Default command
CMD ["sleep", "infinity"]
