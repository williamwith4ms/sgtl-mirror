# Makefile Version : 1
BIN_NAME		:= sgtl

PREFIX        	?= /usr/local
DESTDIR       	?=

CARGO		  	?= cargo

BASH_COMPLETION_DIR ?= $(DESTDIR)$(PREFIX)/share/bash-completion/completions
ZSH_COMPLETION_DIR  ?= $(DESTDIR)$(PREFIX)/share/zsh/site-functions
FISH_COMPLETION_DIR ?= $(DESTDIR)$(PREFIX)/share/fish/vendor_completions.d

COMPLETIONS_BUILD_DIR := target/completions

.PHONY: all build completions install install-bin install-completions uninstall clean

all: build completions

build:
	$(CARGO) build --release

completions: build
	mkdir -p $(COMPLETIONS_BUILD_DIR)
	$(CARGO) run --quiet --bin completions -- $(COMPLETIONS_BUILD_DIR)

install: install-bin install-completions

install-bin: build
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	install -m 0755 target/release/$(BIN_NAME) $(DESTDIR)$(PREFIX)/bin/$(BIN_NAME)

install-completions: completions
	@echo "Installing completions"
	mkdir -p $(BASH_COMPLETION_DIR)
	mkdir -p $(ZSH_COMPLETION_DIR)
	mkdir -p $(FISH_COMPLETION_DIR)

	install -m 0644 $(COMPLETIONS_BUILD_DIR)/$(BIN_NAME).bash $(BASH_COMPLETION_DIR)/$(BIN_NAME)
	install -m 0644 $(COMPLETIONS_BUILD_DIR)/_$(BIN_NAME) $(ZSH_COMPLETION_DIR)/_$(BIN_NAME)
	install -m 0644 $(COMPLETIONS_BUILD_DIR)/$(BIN_NAME).fish $(FISH_COMPLETION_DIR)/$(BIN_NAME).fish


uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/$(BIN_NAME)
	rm -f $(BASH_COMPLETION_DIR)/$(BIN_NAME)
	rm -f $(ZSH_COMPLETION_DIR)/_$(BIN_NAME)
	rm -f $(FISH_COMPLETION_DIR)/$(BIN_NAME).fish


clean:
	$(CARGO) clean
	-rm -r $(COMPLETIONS_BUILD_DIR)