#!/usr/bin/env bash
set -e

apt-get update && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
      git \
      build-essential \
      pkg-config \
      libssl-dev \
      sudo \
      tini &&

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y

curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

echo "--- Instalación de Rust completada ---"
