FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    gcc-riscv64-linux-gnu qemu-user curl build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add riscv64gc-unknown-linux-gnu

WORKDIR /project
