CC = cc
RUSTC = rustc

CFLAGS = -Wall #-Werror -Wextra

all: memory_c memory_rs

memory_c: memory.c
	$(CC) $(CFLAGS) -o $@ $<

memory_rs: memory.rs
	$(RUSTC) -o $@ $<

clean:
	rm -f memory_c memory_rs

.PHONY: all clean
