
.PHONY: help
help: ## Print info about all commands
	@echo "Commands:"
	@echo
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "    \033[01;32m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: test
test: ## Run all tests
	cargo test -- --test-threads 1

.PHONY: lint
lint: ## Run syntax/style checks
	cargo clippy fatcat-cli

.PHONY: fmt
fmt: ## Run syntax re-formatting
	cargo fmt fatcat-cli

.PHONY: build
build: ## Build
	cargo build
