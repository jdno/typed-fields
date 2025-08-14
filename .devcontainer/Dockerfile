# syntax=docker/dockerfile:1
# Base on Microsoft devcontainers image for good Codespaces defaults
FROM mcr.microsoft.com/devcontainers/rust:1

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
    && rm -rf /var/lib/apt/lists/*

# Developer tools used by the project
RUN cargo install cargo-hack just taplo-cli \
    && npm install -g prettier markdownlint-cli \
    && pip3 install --no-cache-dir --break-system-packages pre-commit yamllint

# Create a non-root user (vscode) if not present (typical in devcontainers base)
# and ensure cargo and rustup are accessible to the user
RUN usermod -aG sudo vscode || true

# Set working directory inside container
WORKDIR /workspaces/typed-fields

# Run the container as the vscode user
USER vscode

# Default command
CMD ["sleep", "infinity"]
