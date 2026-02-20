PREFIX ?= $(HOME)/.local
BINARY := hevy-bridge

.PHONY: build install clean

build:
	cargo build --release

install: build
	install -Dm755 target/release/$(BINARY) $(PREFIX)/bin/$(BINARY)

clean:
	cargo clean
