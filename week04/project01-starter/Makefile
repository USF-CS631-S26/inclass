.PHONY: all clean c rust

all: c rust

c:
	$(MAKE) -C c

rust:
	cargo build --manifest-path rust/Cargo.toml

clean:
	$(MAKE) -C c clean
	cargo clean --manifest-path rust/Cargo.toml
