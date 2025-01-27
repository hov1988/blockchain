# Rust API Client for Blockchain Wallet

This project implements a simple blockchain wallet and API in Rust. It demonstrates blockchain principles like transactions, wallet generation, and querying balances. The API is documented using [Utoipa](https://github.com/juhaku/utoipa) for OpenAPI (Swagger) generation and leverages the `actix-web` framework for handling HTTP requests.

## Overview

The project uses the [OpenAPI Generator](https://openapi-generator.tech) to generate API documentation and provides a simple implementation of a blockchain and wallet. The wallet supports functionality such as signing transactions, querying balances, and retrieving transaction histories.

- **API version:** 1.0.0
- **Package version:** 1.0.0
- **Build tools:**
  - [Actix Web](https://actix.rs/) for the web framework
  - [Utoipa](https://github.com/juhaku/utoipa) for OpenAPI documentation
  - [Serde](https://serde.rs/) for serialization/deserialization

## Code Quality Tools

This project uses the following tools to maintain code quality:

1. **`rustfmt`**: Ensures code is formatted according to Rust's style guidelines.
2. **`clippy`**: Provides linting for common mistakes, inefficiencies, and best practices.

### Running Code Quality Checks

You can use the provided `Makefile` to run the following commands:

- **Format Code**
  ```bash
  make fmt


## Features

- **Wallet Management:**
  - Create new wallets
  - Retrieve wallet information (address, public key, private key)
  - Sign transactions
- **Blockchain Operations:**
  - Add transactions to the blockchain
  - Retrieve all transactions
  - Query balances for specific addresses
- **API Documentation:** OpenAPI 3.0-compliant documentation with Swagger UI.

## Installation

Clone the repository and ensure the dependencies in `Cargo.toml` are installed:

