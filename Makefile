# Define the cargo command for convenience
CARGO = cargo

# Target to run rustfmt (format the code)
fmt:
	@$(CARGO) fmt --all

fix:
	@$(CARGO) fix --allow-dirty

# Target to run clippy (lint the code)
lint:
	@$(CARGO) clippy

# Target to run both lint and format checks
check-fmt:
	@$(MAKE) fmt

check-lint:
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

