# Makefile for Startup Platform

BINARY_NAME=server
BUILD_DIR=.

GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
GOTEST=$(GOCMD) test
GOGET=$(GOCMD) get
GOMOD=$(GOCMD) mod
GOFMT=$(GOCMD) fmt

build: generate clean
	@echo "Building $(BINARY_NAME)..."
	$(GOBUILD) -o $(BUILD_DIR)/$(BINARY_NAME) ./cmd/server/main.go
	@echo "Built $(BINARY_NAME) successfully!"

clean:
	@echo "Cleaning build artifacts..."
	$(GOCLEAN)
	rm -f $(BUILD_DIR)/$(BINARY_NAME)
	rm -rf tmp/
	@echo "Cleaned successfully!"

deps:
	@echo "Installing dependencies..."
	$(GOGET) -u github.com/a-h/templ/cmd/templ@latest
	$(GOMOD) download

generate:
	@echo "Generating templ components..."
	# Generate all templ files under templates/, including:
	# - templates/layouts/*.templ         (layout & navigation shells)
	# - templates/pages/*.templ           (page-level content)
	# - templates/components/*.templ      (shared UI components)
	templ generate -path templates
	@echo "Templ components generated!"

dev: generate
	@echo "Starting development server..."
	go run cmd/server/main.go

watch: generate
	@echo "Starting comprehensive watch mode (Go + Templ files)..."
	@if command -v inotifywait >/dev/null 2>&1; then \
		while true; do \
			inotifywait -e modify -r . --include '\.(go|templ)$$' 2>/dev/null || break; \
			echo "File changes detected, rebuilding and restarting..."; \
			pkill -f "$(BUILD_DIR)/$(BINARY_NAME)" 2>/dev/null || true; \
			$(MAKE) generate && \
			$(MAKE) build && \
			./$(BUILD_DIR)/$(BINARY_NAME) & \
			echo "Server restarted with PID $$!"; \
		done; \
	else \
		echo "inotifywait not found. Install with: sudo apt-get install inotify-tools (Debian/Ubuntu)"; \
		echo "Falling back to simple file watching with find..."; \
		while true; do \
			pkill -f "$(BUILD_DIR)/$(BINARY_NAME)" 2>/dev/null || true; \
			find . -name "*.go" -o -name "*.templ" | while read file; do \
				if [ "$$file" -nt /tmp/last_build ]; then \
					touch /tmp/last_build; \
					echo "Changes detected, rebuilding and restarting..."; \
					$(MAKE) generate && \
					$(MAKE) build && \
					./$(BUILD_DIR)/$(BINARY_NAME) & \
					break; \
				fi; \
			done; \
			sleep 5; \
		done; \
	fi

air: generate
	@echo "Starting development server with Air live reload..."
	@echo "Killing any existing process on port 8081..."
	@-kill $(lsof -t -i :8081) 2>/dev/null || true
	@sleep 1
	air

air-watch: generate
	@echo "Starting development server with Air live reload (alternative name)..."
	@echo "Killing any existing process on port 8081..."
	@-kill $(lsof -t -i :8081) 2>/dev/null || true
	@sleep 1
	air

dev-watch: generate
	@echo "Starting development server with comprehensive file watching..."
	@if command -v inotifywait >/dev/null 2>&1; then \
		while true; do \
			inotifywait -e modify -r . --include '\.(go|templ)$$' 2>/dev/null || break; \
			echo "File changes detected, rebuilding and restarting..."; \
			pkill -f "$(BUILD_DIR)/$(BINARY_NAME)" 2>/dev/null || true; \
			$(MAKE) generate && \
			$(MAKE) build && \
			./$(BUILD_DIR)/$(BINARY_NAME) & \
			echo "Server restarted with PID $$!"; \
		done; \
	else \
		echo "inotifywait not found. Please install inotify-tools for optimal file watching."; \
		echo "Install with: sudo apt-get install inotify-tools (Debian/Ubuntu) or brew install inotify-tools (macOS)"; \
		echo "Running once without live reload..."; \
		./$(BUILD_DIR)/$(BINARY_NAME); \
	fi

run: generate build
	@echo "Starting $(BINARY_NAME)..."
	@pkill -f "$(BUILD_DIR)/$(BINARY_NAME)" 2>/dev/null || true; \
	echo "Killed any existing server processes"; \
	./$(BUILD_DIR)/$(BINARY_NAME)

test:
	@echo "Running tests..."
	$(GOTEST) ./...

fmt:
	@echo "Formatting Go code with goimports..."
	goimports -w .
	@echo "Go imports and formatting applied!"

format: fmt ## Alias for fmt command
	@echo "Code formatting completed!"

imports: ## Organize imports only
	@echo "Organizing imports..."
	goimports -w -local="" .
	@echo "Imports organized!"

lint: ## Run golangci-lint for code quality
	@echo "Running linter..."
	golangci-lint run --timeout=2m

check: fmt lint build ## Run all checks (fmt + lint + build)
	@echo "All checks completed!"

all: deps generate build
	@echo "Setup complete!"

.PHONY: build clean deps generate dev watch dev-watch run test fmt lint check all
