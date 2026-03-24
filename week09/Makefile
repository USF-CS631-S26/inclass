UNAME_M := $(shell uname -m)
IMAGE_NAME := week09-riscv

ifeq ($(UNAME_M),riscv64)
  CARGO_CMD = cargo
  CARGO_FLAGS =
else
  CARGO_CMD = docker run --rm -v $(CURDIR):/project $(IMAGE_NAME) cargo
  CARGO_FLAGS = --target riscv64gc-unknown-linux-gnu
endif

.PHONY: build run clean docker-image-ensure docker-build docker-shell debug-server debug rust-debug

build: docker-image-ensure
	$(CARGO_CMD) build $(CARGO_FLAGS)

run: docker-image-ensure
	$(CARGO_CMD) run $(CARGO_FLAGS) -- $(ARGS)

clean:
	cargo clean

docker-image-ensure:
ifeq ($(UNAME_M),riscv64)
	@true
else
	@if ! docker image inspect $(IMAGE_NAME) >/dev/null 2>&1; then \
		echo "Building Docker image '$(IMAGE_NAME)'..." >&2; \
		docker build -t $(IMAGE_NAME) .; \
	fi
endif

docker-build:
	docker build -t $(IMAGE_NAME) .

docker-shell:
	docker run --rm -it -v $(CURDIR):/project $(IMAGE_NAME) bash

# Debug: Terminal 1 - start QEMU waiting for GDB connection
debug-server: docker-image-ensure build
	docker run --rm -v $(CURDIR):/project --name week09-debug \
	    $(IMAGE_NAME) \
	    qemu-riscv64 -g 1234 -L /usr/riscv64-linux-gnu \
	    target/riscv64gc-unknown-linux-gnu/debug/week09 $(ARGS)

# Debug: Terminal 2 - attach gdb-multiarch
debug:
	docker exec -it week09-debug \
	    gdb-multiarch -x .gdbinit target/riscv64gc-unknown-linux-gnu/debug/week09

# Debug: Terminal 2 - attach rust-gdb (with Rust pretty-printers)
rust-debug:
	docker exec -it -e GDB=gdb-multiarch week09-debug \
	    rust-gdb \
	    -ex "set architecture riscv:rv64" \
	    -ex "target remote localhost:1234" \
	    -ex "set sysroot /usr/riscv64-linux-gnu" \
	    target/riscv64gc-unknown-linux-gnu/debug/week09
