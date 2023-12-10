# Copyright (c) 2023 und3fy.dev. All rights reserved.
# Created by und3fined <me@und3fy.dev> on 2023 Sep 08.

.PHONY: help
.SILENT: crate dev

FONT_RED := $(shell tput setaf 1)
FONT_GREEN := $(shell tput setaf 2)
FONT_YELLOW := $(shell tput setaf 3)
FONT_BLUE := $(shell tput setaf 4)
FONT_PURPLE := $(shell tput setaf 5)
FONT_CYAN := $(shell tput setaf 6)
FONT_GRAY := $(shell tput setaf 7)
FONT_BLACK := $(shell tput setaf 8)
FONT_BOLD := $(shell tput bold)
FONT_RESET := $(shell tput sgr0)

ROOT := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
RELATIVE_ROOT := $(notdir $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST))))))

# Detect M1/M2 Macs and set a flag.
ifeq ($(shell uname)/$(shell uname -m),Darwin/arm64)
  M1_MAC = true
endif

# Build Flags
BUILD_NUMBER ?= $(BUILD_NUMBER:)
BUILD_DATE = $(shell date -u)
BUILD_HASH = $(shell git rev-parse HEAD)

# If we don't set the build number it defaults to dev
ifeq ($(BUILD_NUMBER),)
	BUILD_DATE := n/a
	BUILD_NUMBER := dev
endif

# Rust region
CARGO=cargo

all: help


hello: ## Say hello
	@echo "Hello world!\nI'm a Makefile and I'm here to help you."


dev: ## Run the project in development mode
	@echo "Running in development mode"
	@echo "ROOT: $(RELATIVE_ROOT)"
	@echo "BUILD_NUMBER: $(BUILD_NUMBER)"
	env RUST_LOG=debug $(CARGO) run $@ --quiet


clean: ## Clean up the project
	rm -rf target
	rm -rf .azoni


crate: ## New crate with cargo. Usage: make crate type=<bin|lib> name=<crate_name>
	if [ -z "$(name)" ]; then \
		echo "Please specify a name for the crate.\n$(FONT_BOLD)Usage$(FONT_RESET): $(FONT_YELLOW)make crate type=<bin|lib> name=<crate_name>$(FONT_RESET)"; \
		exit 1; \
	fi
	if [ -z "$(type)" ]; then \
		echo "Please specify a type for the crate.\n$(FONT_BOLD)Usage$(FONT_RESET): $(FONT_YELLOW)make crate type=<bin|lib> name=<crate_name>$(FONT_RESET)"; \
		exit 1; \
	fi
	if [ "$(type)" != "bin" ] && [ "$(type)" != "lib" ]; then \
		echo "Invalid $(FONT_YELLOW)type$(FONT_RESET) specified. Valid type is $(FONT_BLUE)lib$(FONT_RESET) or $(FONT_BLUE)bin$(FONT_RESET).\n$(FONT_BOLD)Usage$(FONT_RESET): $(FONT_YELLOW)make crate type=<bin|lib> name=<crate_name>$(FONT_RESET)"; \
		exit 1; \
	fi

	@echo "Creating new crate."
	@echo "$(FONT_BOLD)Package$(FONT_RESET): $(FONT_CYAN)azoni-$(name)$(FONT_RESET) • $(FONT_BOLD)Type$(FONT_RESET): $(FONT_CYAN)$(type)$(FONT_RESET)"
	$(CARGO) new --$(type) --name azoni-$(name) crates/$(name) --quiet
	@echo "Open $(FONT_BLUE)crates/$(name)$(FONT_RESET) to start working on your new crate."


# Source https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Show this help
	@echo "Usage: $(FONT_GREEN)make$(FONT_RESET) $(FONT_BLUE)<target>$(FONT_RESET)"
	@echo "Targets:"
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' ./Makefile | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  • \033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo
	@echo You can modify the default settings for this Makefile creating a file config.mk based on the default-config.mk
	@echo
