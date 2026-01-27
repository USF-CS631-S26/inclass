# Makefile for C language feature examples

CC = cc
CFLAGS = -Wall -Wextra -std=c99

# Find all .c files and create corresponding binary names
SOURCES = $(wildcard *.c)
BINARIES = $(SOURCES:.c=)

.PHONY: all clean

all: $(BINARIES)

# Pattern rule: compile any .c file into a binary
%: %.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f $(BINARIES)
