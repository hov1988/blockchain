# Define the cargo command for convenience
CARGO = cargo

# Target to run rustfmt (format the code)
fmt:
	@$(CARGO) fmt --all

# Target to check formatting without modifying code
fmt-check:
	@$(CARGO) fmt --all -- --check

# Target to run clippy (lint the code)
lint:
	@$(CARGO) clippy --all-targets --all-features -- -D warnings

# Target to run both lint and format checks
check:
	@$(MAKE) fmt-check
	@$(MAKE) lint

# Target to build the project
build:
	@$(CARGO) build

# Target to test the project
test:
	@$(CARGO) test

# Target to clean the project
clean:
	@$(CARGO) clean

# Target to run the project
run:
	@$(CARGO) run
