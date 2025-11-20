# matrix Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-11-18

## Active Technologies
- N/A (in-memory layout engine) (002-widget-composability)
- Rust 1.91.1+ (stable channel, 2021 edition) + Zero runtime dependencies (only Rust `std`); optional `serde` feature-gated (002-widget-composability)
- Rust 1.91.1+ (stable channel, 2021 edition) + Zero runtime dependencies (only Rust `std`); optional `serde` feature-gated for serialization, optional `tracing` feature-gated for observability (001-escp2-printer-driver)
- N/A (in-memory driver, communicates with printer via I/O) (001-escp2-printer-driver)

- Rust 1.75+ (stable channel, 2021 edition) + Zero runtime dependencies (only Rust `std`); optional `serde` feature-gated (001-rust-escap-layout-engine)

## Project Structure

```text
backend/
frontend/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 1.75+ (stable channel, 2021 edition): Follow standard conventions

## Recent Changes
- 001-escp2-printer-driver: Added Rust 1.91.1+ (stable channel, 2021 edition) + Zero runtime dependencies (only Rust `std`); optional `serde` feature-gated for serialization, optional `tracing` feature-gated for observability
- 001-escp2-printer-driver: Added [if applicable, e.g., PostgreSQL, CoreData, files or N/A]
- 002-widget-composability: Added Rust 1.91.1+ (stable channel, 2021 edition) + Zero runtime dependencies (only Rust `std`); optional `serde` feature-gated


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
