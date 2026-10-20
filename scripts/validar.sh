#!/bin/sh
# Validação completa (a mesma do CI): formatação, clippy e cobertura de 100% das linhas.
# Sem Rust local: docker run --rm -v <cargo-reg>:/usr/local/cargo/registry -v <repo>:/w -w /w rust:latest sh scripts/validar.sh
set -e
rustup component add llvm-tools-preview rustfmt clippy >/dev/null 2>&1 || true
command -v cargo-llvm-cov >/dev/null 2>&1 || cargo install cargo-llvm-cov --locked
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy -p suap-sync --features test-support -- -D warnings
cargo llvm-cov --workspace --all-targets --fail-under-lines 100
