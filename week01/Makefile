# Top-level Makefile for week01 examples
# Builds both C and Rust programs

.PHONY: all clean c rust clean-c clean-rust

all: c rust

c:
	$(MAKE) -C c

rust:
	cd rust && cargo build

clean: clean-c clean-rust

clean-c:
	$(MAKE) -C c clean

clean-rust:
	cd rust && cargo clean
