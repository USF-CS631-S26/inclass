UNAME_M := $(shell uname -m)
BINS := get_bitseq get_bitseq_signed pack_bytes unpack_bytes rstr rstr_rec ntlang
IMAGE_NAME := project02-riscv

ifeq ($(UNAME_M),riscv64)
  CARGO_CMD = cargo
  CARGO_FLAGS =
else
  CARGO_CMD = docker run --rm -v $(CURDIR):/project $(IMAGE_NAME) cargo
  CARGO_FLAGS = --target riscv64gc-unknown-linux-gnu
endif

.PHONY: build run-all clean $(addprefix run-,$(BINS))
.PHONY: docker-image-ensure docker-build docker-shell

# --- Primary targets (auto-delegate to Docker on non-RISC-V) ---

build: docker-image-ensure
	$(CARGO_CMD) build $(CARGO_FLAGS)

$(addprefix run-,$(BINS)): run-%: docker-image-ensure
	$(CARGO_CMD) run $(CARGO_FLAGS) --bin $* -- $(ARGS)

run-all: $(addprefix run-,$(BINS))

clean:
	cargo clean

# --- Docker targets ---

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
