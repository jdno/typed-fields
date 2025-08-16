# syntax=docker/dockerfile:1

FROM ubuntu:plucky-20250730

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
        just \
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
        vim \
    && rm -rf /var/lib/apt/lists/*

# Install global developer tools as root
RUN npm install -g prettier markdownlint-cli \
    && pip3 install --no-cache-dir --break-system-packages pre-commit yamllint

# Create a non-root user for development
RUN groupadd vscode && useradd --create-home --gid vscode -s /bin/bash vscode \
    && echo vscode ALL=\(root\) NOPASSWD:ALL > /etc/sudoers

# Create workspace directory and set permissions
RUN mkdir -p /workspaces/typed-fields \
    && chown -R vscode:vscode /workspaces/typed-fields

# Run commands in the container as the vscode user
USER vscode

# Set working directory inside container
WORKDIR /workspaces/typed-fields

# Add Cargo binaries to path
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

# Install Rust toolchain
RUN rustup default stable

# Install more developer tools with cargo
RUN cargo install cargo-hack taplo-cli

# Default command
CMD ["sleep", "infinity"]
