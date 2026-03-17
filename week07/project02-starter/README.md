# project02

## Overview

Project02 has 7 binaries that use RISC-V assembly:

- `get_bitseq`, `get_bitseq_signed` — bit extraction
- `pack_bytes`, `unpack_bytes` — byte packing/unpacking
- `rstr`, `rstr_rec` — string reversal (iterative and recursive)
- `ntlang` — expression compiler targeting RISC-V assembly

## Cross-Compilation Setup

On non-RISC-V hosts (macOS, x86 Linux), binaries are cross-compiled for RISC-V and run via QEMU inside a Docker container. You need a Docker runtime installed — any of the following will work.

### Option 1: Docker Desktop

A GUI-based Docker runtime for macOS, Linux, and Windows.

1. Download and install from [docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop/).
2. Launch Docker Desktop — Docker is ready once the app shows "Engine running."

### Option 2: Colima

A lightweight, CLI-only container runtime. Good for headless / resource-constrained environments.

#### macOS

```bash
brew install colima docker
```

#### Linux (Ubuntu/Debian)

```bash
# Install Docker CLI
sudo apt-get update && sudo apt-get install -y docker.io

# Install Colima
curl -LO https://github.com/abiosoft/colima/releases/latest/download/colima-$(uname -s)-$(uname -m)
sudo install colima-$(uname -s)-$(uname -m) /usr/local/bin/colima
rm colima-$(uname -s)-$(uname -m)
```

#### Starting Colima

```bash
colima start
```

### Option 3: OrbStack

A fast, low-resource Docker runtime for macOS only.

#### macOS

```bash
brew install orbstack
```

Or download from [orbstack.dev](https://orbstack.dev/).

Launch OrbStack — Docker is available automatically once the app is running.

### Verify Docker is Working

Whichever runtime you chose, confirm Docker is available:

```bash
docker run --rm hello-world
```

### Building the Docker Image

```bash
make docker-build
```

This creates the `project02-riscv` image with the RISC-V cross-compiler, QEMU, and Rust toolchain.

## Running Binaries

### Using `riscv-run` (recommended)

```bash
./riscv-run get_bitseq 94116 12 15
./riscv-run rstr "FooBar"
./riscv-run pack_bytes 1 2 3 4
```

This automatically builds the Docker image if needed, cross-compiles, and runs via QEMU.

### Using Make

```bash
# Single binary
make docker-run-get_bitseq ARGS="94116 12 15"

# All binaries
make docker-run-all

# Interactive shell inside the container
make docker-shell
```

### ntlang Compilation

`ntlang -c` compiles an expression to a RISC-V executable, and `riscv-run` executes it:

```bash
riscv-run ntlang -e "(3 * 4) + 5" -c prog
riscv-run ./prog

riscv-run ntlang -e "a0 + a1" -c prog
riscv-run ./prog 3 4

# Assembly only (no linking)
ntlang -e "a0 + a1" -c prog -s    # produces prog.s
```

### Running Natively (inside Docker or on RISC-V)

Inside a Docker shell (`make docker-shell`) or on a native RISC-V machine:

```bash
make build
make run-get_bitseq ARGS="94116 12 15"
```
