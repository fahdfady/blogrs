# Setup Guide

This document provides detailed setup instructions for running the Blog API project on both NixOS and non-NixOS systems.

## NixOS Setup

### Prerequisites

1. Make sure you have Nix installed on your system
2. Enable flakes if you haven't already (optional but recommended)

### Setup Steps

1. check `shell.nix` file in your project root

2. Enter the development environment:
```bash
nix-shell
```

3. Create necessary directories:
```bash
mkdir -p data
```

4. Build and run the project:
```bash
cargo build
cargo run
```

## Non-NixOS Setup

### Prerequisites

1. Install Rust and Cargo (https://rustup.rs/)
2. Install SQLite development libraries:
   - Ubuntu/Debian: `sudo apt-get install libsqlite3-dev`
   - Fedora: `sudo dnf install sqlite-devel`
   - macOS: `brew install sqlite`
   - Windows: SQLite is bundled with Rust on Windows

### Setup Steps

1. Install sqlx-cli:
```bash
cargo install sqlx-cli
```

2. Create necessary directories:
```bash
mkdir -p data
```

3. Build and run the project:
```bash
cargo build
cargo run
```

## Common Steps (All Platforms)

After setting up your platform-specific requirements:

1. Install project dependencies:
```bash
cargo build
```

2. Initialize the database and run migrations:
```bash
# Make sure data directory exists
mkdir -p data
# Create database file
touch data/blog.db
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:3000`.

## Troubleshooting

### Common Issues

1. **Database connection error**:
   - Ensure the `data` directory exists
   - Check file permissions on the `data` directory
   - Verify SQLite is properly installed

2. **Build failures**:
   - Run `cargo clean` and try rebuilding
   - Verify all system dependencies are installed
   - Check Rust toolchain version (`rustc --version`)

3. **Migration errors**:
   - Delete the existing database file and recreate it
   - Ensure migration SQL syntax is correct
   - Check database file permissions

### Getting Help

If you encounter issues:

1. Check the error message carefully
2. Enable backtraces: `export RUST_BACKTRACE=1`
3. Verify all dependencies are correctly installed

## Development Notes

- The server runs in debug mode by default
- Hot reloading is not enabled by default
- SQLite database file is stored in `data/blog.db`
- API runs on `localhost:3000` (or the port specified in the `SERVER_PORT` config file)

## Testing the API

Use the provided curl commands in the README to test the API endpoints. You can also use tools like Postman or Insomnia for a better testing experience.