# 🦀 Rust Bootcamp - Dev Container
# Imagen para VS Code Dev Containers

FROM rust:1.92-slim-bookworm

LABEL maintainer="ergrato-dev"
LABEL description="Bootcamp Rust - Dev Container"

# Evitar prompts interactivos
ENV DEBIAN_FRONTEND=noninteractive

# Instalar herramientas del sistema
RUN apt-get update && apt-get install -y \
    git \
    curl \
    vim \
    less \
    tree \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Instalar componentes de Rust
RUN rustup component add \
    rustfmt \
    clippy \
    rust-src \
    rust-docs \
    rust-analyzer

# Instalar herramientas de Cargo
RUN cargo install \
    cargo-watch \
    cargo-edit \
    cargo-expand \
    bacon

# Directorio de trabajo
WORKDIR /workspace

CMD ["bash"]
