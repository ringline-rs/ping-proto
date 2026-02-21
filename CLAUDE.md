# CLAUDE.md

## Build & Test Commands

```bash
# Build
cargo build

# Lint
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings

# Test
cargo test --all
cargo test --all --release

# Docs
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Architecture

Simple ASCII PING/PONG protocol parser and encoder.

### Key Types

- `Request` — PING request (encode only)
- `Response` — PONG/Error response (parse and encode)
- `ParseError` — Incomplete or invalid parse errors
