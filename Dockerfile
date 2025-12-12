# 🦀 Rust Bootcamp - Docker Image
# Imagen oficial de Rust con herramientas de desarrollo

FROM rust:1.92-slim-bookworm

LABEL maintainer="ergrato-dev"
LABEL description="Bootcamp Rust Zero to Hero - Entorno de desarrollo"
LABEL version="1.0"

# Evitar prompts interactivos
ENV DEBIAN_FRONTEND=noninteractive

# Instalar herramientas del sistema
RUN apt-get update && apt-get install -y \
    git \
    curl \
    vim \
    less \
    tree \
    && rm -rf /var/lib/apt/lists/*

# Instalar componentes de Rust
RUN rustup component add \
    rustfmt \
    clippy \
    rust-src \
    rust-docs

# Instalar herramientas de Cargo útiles para desarrollo
RUN cargo install \
    cargo-watch \
    cargo-edit \
    cargo-expand \
    bacon

# Crear directorio de trabajo
WORKDIR /workspace

# Configurar shell
ENV SHELL=/bin/bash

# Comando por defecto
CMD ["bash"]
