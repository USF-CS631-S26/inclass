# week09

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

This creates the `week09-riscv` image with the RISC-V cross-compiler, QEMU, GDB, and Rust toolchain.

## Building and Running

```bash
make build
make run
```

## Debugging with GDB

Two options for debugging RISC-V binaries: Makefile targets (two-terminal) or the `riscv-debug` script (single command).

### Option 1: Makefile Targets (two terminals)

In terminal 1, start QEMU paused and waiting for a GDB connection:

```bash
make debug-server
```

In terminal 2, attach a debugger:

```bash
make debug        # gdb-multiarch
make rust-debug   # rust-gdb (with Rust pretty-printers)
```

### Option 2: `riscv-debug` Script (single command)

```bash
./riscv-debug week09            # uses gdb-multiarch
./riscv-debug --rust week09     # uses rust-gdb
```

The script builds the binary, starts QEMU with a GDB stub, attaches the debugger, and cleans up on exit.

### Basic GDB Commands

Once in GDB, the debugger is connected and the program is paused at entry:

```
(gdb) break main          # set breakpoint at main
(gdb) continue             # run to breakpoint
(gdb) step                 # step one source line
(gdb) stepi                # step one instruction
(gdb) next                 # step over function calls
(gdb) info registers       # show all registers
(gdb) print $a0            # print a single register
(gdb) x/10i $pc            # disassemble 10 instructions at PC
(gdb) disassemble          # disassemble current function
(gdb) quit                 # exit
```
