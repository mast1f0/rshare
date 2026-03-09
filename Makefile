SHELL := /bin/bash
CARGO ?= cargo
CRATE_DIR := rshare
BINARY := rshare
PREFIX ?= /opt/rshare
DESTDIR ?=
BIN_DIR ?= /usr/local/bin

.PHONY: help build run install uninstall clean

help:
	@echo "Available targets:"
	@echo "  make build      - Build release binary"
	@echo "  make run        - Run the app"
	@echo "  make install    - Install app files to \$$PREFIX and binary to \$$BIN_DIR"
	@echo "  make uninstall  - Remove installed binary from \$$BIN_DIR"
	@echo "  make clean      - Clean build artifacts"

build:
	$(CARGO) build --release --manifest-path $(CRATE_DIR)/Cargo.toml

run:
	$(CARGO) run --manifest-path $(CRATE_DIR)/Cargo.toml

install: build
	install -d "$(DESTDIR)$(PREFIX)"
	install -d "$(DESTDIR)$(PREFIX)/templates"
	install -d "$(DESTDIR)$(PREFIX)/shared"
	install -d "$(DESTDIR)$(BIN_DIR)"
	cp -r "$(CRATE_DIR)/templates/." "$(DESTDIR)$(PREFIX)/templates/"
	install -m 755 "$(CRATE_DIR)/target/release/$(BINARY)" "$(DESTDIR)$(BIN_DIR)/$(BINARY)"

uninstall:
	rm -f "$(DESTDIR)$(BIN_DIR)/$(BINARY)"

clean:
	$(CARGO) clean --manifest-path $(CRATE_DIR)/Cargo.toml
