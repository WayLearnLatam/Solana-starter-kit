#!/usr/bin/env bash
set -e

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y

echo "--- Instalación de Rust completada ---"
